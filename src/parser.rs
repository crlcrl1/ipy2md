use json::{parse, JsonValue};

pub struct Notebook {
    pub cells: Vec<Cell>,
}

impl Notebook {
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
                Some(Cell::new(cell_type, source, outputs))
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

    fn get_output(output_part: &JsonValue) -> Vec<String> {
        let output_type = output_part[0]["output_type"].as_str().unwrap_or_default();
        match output_type {
            "stream" => Self::get_stream_output(&output_part[0]),
            "execute_result" => Self::get_execute_result_output(&output_part[0]),
            _ => vec![],
        }
    }
    fn get_execute_result_output(output_part: &JsonValue) -> Vec<String> {
        let data = &output_part["data"];
        let mut res = vec![];
        data.entries().for_each(|(key, value)| match key {
            "text/plain" => value.members().for_each(|line| {
                res.push(line.as_str().unwrap_or_default().trim_end().to_string() + "</br>\n")
            }),
            "text/html" | "text/markdown" => value.members().for_each(|line| {
                res.push(line.as_str().unwrap_or_default().trim_end().to_string() + "\n")
            }),
            _ => {}
        });
        res
    }

    fn get_stream_output(output_part: &JsonValue) -> Vec<String> {
        output_part["text"]
            .members()
            .map(|line| line.as_str().unwrap_or_default().trim_end().to_string() + "</br>\n")
            .collect()
    }
}

pub enum CellType {
    Code,
    Markdown,
}

pub struct Cell {
    pub cell_type: CellType,
    pub source: String,
    pub outputs: Vec<String>,
}

impl Cell {
    pub fn new(cell_type: CellType, source: String, outputs: Vec<String>) -> Self {
        Self {
            cell_type,
            source,
            outputs,
        }
    }
}
