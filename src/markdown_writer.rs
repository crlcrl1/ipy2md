use crate::params::Params;
use crate::parser::{CellType, Notebook};

pub fn get_markdown_string(notebook: &Notebook, params: &Params) -> String {
    let mut markdown = String::new();
    let cell_count = notebook.cells.len();
    for (i, cell) in notebook.cells.iter().enumerate() {
        match cell.cell_type {
            CellType::Code => {
                markdown.push_str("```python\n");
                markdown.push_str(&cell.source);
                markdown.push_str("\n```\n");
                if params.show_output {
                    if !cell.outputs.is_empty() {
                        markdown.push_str("\n<p>\n");
                        markdown.push_str(cell.outputs.as_str());
                        markdown.push_str("\n</p>\n\n");
                    }
                    if !cell.error_outputs.is_empty() {
                        markdown.push_str("<p style='font-family: Consolas,system-ui'>");
                        markdown.push_str(cell.error_outputs.as_str());
                        markdown.push_str("\n</p>\n\n");
                    }
                }
            }
            CellType::Markdown => {
                markdown.push_str(&cell.source);
                markdown.push_str("\n");
                markdown.push_str("\n");
            }
        }
        if params.block_separator && i != cell_count - 1 {
            markdown.push_str("\n------\n\n");
        } else {
            markdown.push_str("\n");
        }
    }
    markdown
}
