use ansi_to_html::convert;
use json::{parse, JsonValue};

/// A struct representing a Jupyter notebook
pub struct Notebook {
    pub cells: Vec<Cell>,
}

#[inline(always)]
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

impl Notebook {
    /// Parse the JSON string and return a `Notebook` struct
    pub fn from_string(json_string: &str) -> Option<Self> {
        let parsed = parse(json_string).ok()?;
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
                let outputs = Self::get_output(&cell["outputs"]);
                let error_outputs = Self::get_error_output(&cell["outputs"]);
                Some(Cell::new(cell_type, source, outputs, error_outputs))
            })
            .collect();
        Some(Self { cells })
    }

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

    fn get_output(output_part: &JsonValue) -> String {
        output_part
            .members()
            .map(|section| {
                let output_type = section["output_type"].as_str().unwrap_or_default();
                match output_type {
                    "stream" => Self::get_stream_output(&section),
                    "execute_result" => Self::get_execute_result_output(&section),
                    _ => "".to_string(),
                }
            })
            .collect()
    }

    #[inline(always)]
    fn get_execute_result_output(output_part: &JsonValue) -> String {
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
        res.join("")
    }

    #[inline(always)]
    fn get_stream_output(output_part: &JsonValue) -> String {
        output_part["text"]
            .members()
            .map(|line| line.as_str().unwrap_or_default().trim_end().to_string() + "<br/>\n")
            .collect()
    }

    fn get_error_output(output_part: &JsonValue) -> String {
        output_part
            .members()
            .filter_map(|section| {
                let output_type = section["output_type"].as_str().unwrap_or_default();
                if output_type == "error" {
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
}

/// A struct representing a cell in a Jupyter notebook
pub enum CellType {
    Code,
    Markdown,
}

pub struct Cell {
    pub cell_type: CellType,
    pub source: String,
    pub outputs: String,
    pub error_outputs: String,
}

impl Cell {
    pub fn new(
        cell_type: CellType,
        source: String,
        outputs: String,
        error_outputs: String,
    ) -> Self {
        Self {
            cell_type,
            source,
            outputs,
            error_outputs,
        }
    }
}
