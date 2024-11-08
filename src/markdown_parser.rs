use crate::util::show_warning;
use ansi_to_html::convert;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use image::{DynamicImage, ImageReader};
use json::{parse, JsonValue};
use std::io::Cursor;

#[derive(Default)]
struct ImageNameGenerator {
    counter: usize,
}

impl ImageNameGenerator {
    fn generate(&mut self, image_type: &str) -> String {
        self.counter += 1;
        format!("image_{}.{}", self.counter, image_type)
    }
}

struct Context {
    image_name_generator: ImageNameGenerator,
    image_dir: String,
}

impl Context {
    fn new(image_dir: String) -> Self {
        Self {
            image_name_generator: ImageNameGenerator::default(),
            image_dir,
        }
    }

    fn generate_image_path(&mut self, image_type: &str) -> String {
        format!(
            "{}/{}",
            self.image_dir,
            self.image_name_generator.generate(image_type)
        )
    }
}

/// A struct representing a Jupyter notebook
pub struct Notebook {
    pub cells: Vec<Cell>,
}

fn convert_space(input: &str) -> String {
    let mut flag = false;
    let mut output = String::new();
    for char in input.chars() {
        if char == '<' {
            flag = true;
        } else if char == '>' {
            flag = false;
        } else if char == ' ' && !flag {
            output.push_str("&nbsp;");
            continue;
        }
        output.push(char);
    }
    output
}

fn get_image(data: &[u8]) -> Option<DynamicImage> {
    let fig = BASE64_STANDARD.decode(data);
    if let Ok(fig) = fig {
        ImageReader::new(Cursor::new(fig))
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()
    } else {
        None
    }
}

impl Notebook {
    /// Parse the JSON string and return a [Notebook] struct
    pub fn from_string(json_string: &str, image_dir: String) -> Option<Self> {
        let parsed = parse(json_string).ok()?;
        let mut context = Context::new(image_dir);
        let cells = parsed["cells"]
            .members()
            .filter_map(|cell| {
                let cell_type = match cell["cell_type"].as_str()? {
                    "code" => CellType::Code,
                    "markdown" => CellType::Markdown,
                    _ => return None,
                };
                let source_part = &cell["source"];
                let source = Self::get_source(source_part);
                let (outputs, images) = Self::get_output(&cell["outputs"], &mut context);
                let error_outputs = Self::get_error_output(&cell["outputs"]);
                Some(Cell::new(cell_type, source, outputs, error_outputs, images))
            })
            .collect();
        Some(Self { cells })
    }

    /// Get the source code from the source part of a cell
    fn get_source(source_part: &JsonValue) -> String {
        if source_part.is_string() {
            source_part.as_str().unwrap_or_default().to_string()
        } else {
            source_part
                .members()
                .map(|line| line.as_str().unwrap_or_default())
                .collect::<Vec<&str>>()
                .join("")
        }
    }

    /// Get the output from the output part of a cell
    fn get_output(output_part: &JsonValue, ctx: &mut Context) -> (String, Vec<Image>) {
        let mut res = String::new();
        let mut images = vec![];
        output_part
            .members()
            .map(|section| {
                let output_type = section["output_type"].as_str().unwrap_or_default();
                match output_type {
                    "stream" => Self::get_stream_output(&section),
                    "execute_result" => Self::get_execute_result_output(&section),
                    "display_data" => Self::get_display_data_output(&section, ctx),
                    _ => ("".to_string(), vec![]),
                }
            })
            .for_each(|(output, imgs)| {
                res.push_str(&output);
                images.extend(imgs);
            });
        (res, images)
    }

    fn get_execute_result_output(output_part: &JsonValue) -> (String, Vec<Image>) {
        let data = &output_part["data"];
        let mut res = vec![];
        data.entries().for_each(|(key, value)| match key {
            "text/plain" => value.members().for_each(|line| {
                res.push(line.as_str().unwrap_or_default().trim_end().to_string() + "<br/>\n")
            }),
            "text/html" | "text/markdown" => value.members().for_each(|line| {
                res.push(line.as_str().unwrap_or_default().trim_end().to_string() + "\n")
            }),
            _ => {}
        });
        (res.join(""), vec![])
    }

    fn get_stream_output(output_part: &JsonValue) -> (String, Vec<Image>) {
        let text = output_part["text"]
            .members()
            .map(|line| line.as_str().unwrap_or_default().trim_end().to_string() + "<br/>\n")
            .collect();
        (text, vec![])
    }

    fn get_error_output(output_part: &JsonValue) -> String {
        output_part
            .members()
            .filter_map(|section| {
                let output_type = section["output_type"].as_str().unwrap_or_default();
                if output_type == "error" {
                    // Get Python traceback
                    let traceback = section["traceback"]
                        .members()
                        .map(|line| convert(line.as_str().unwrap_or_default()).unwrap_or_default())
                        .map(|line| convert_space(&line.replace("\n", "<br/>\n")))
                        .collect::<Vec<String>>()
                        .join("<br/>\n");
                    Some(traceback)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get the display data output (e.g. images) from the output part of a cell
    fn get_display_data_output(output_part: &JsonValue, ctx: &mut Context) -> (String, Vec<Image>) {
        let data = &output_part["data"];
        let mut text = vec![];
        let mut images = vec![];
        data.entries().for_each(|(key, value)| {
            if key.starts_with("image/") {
                if let Some(image) = get_image(value.as_str().unwrap_or_default().as_bytes()) {
                    let name = ctx.generate_image_path(&key[6..]);
                    images.push(Image {
                        path: name.clone(),
                        data: image,
                    });
                    text.push(format!("<img src=\"{}\"/>", name));
                } else {
                    show_warning("Failed to decode image");
                }
            }
        });
        (text.join("\n"), images)
    }
}

pub struct Image {
    pub path: String,
    pub data: DynamicImage,
}

/// A struct representing a cell in a Jupyter notebook
pub enum CellType {
    Code,
    Markdown,
}

/// A struct representing a cell in a Jupyter notebook
pub struct Cell {
    pub cell_type: CellType,
    pub source: String,
    pub outputs: String,
    pub error_outputs: String,
    pub images: Vec<Image>,
}

impl Cell {
    pub fn new(
        cell_type: CellType,
        source: String,
        outputs: String,
        error_outputs: String,
        images: Vec<Image>,
    ) -> Self {
        Self {
            cell_type,
            source,
            outputs,
            error_outputs,
            images,
        }
    }
}
