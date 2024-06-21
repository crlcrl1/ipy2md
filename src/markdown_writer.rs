use crate::params::Params;
use crate::parser::{CellType, Notebook};

pub fn get_markdown_string(notebook: &Notebook, params: &Params) -> String {
    let mut markdown = String::new();
    for cell in &notebook.cells {
        match cell.cell_type {
            CellType::Code => {
                markdown.push_str("```python\n");
                markdown.push_str(&cell.source);
                markdown.push_str("\n```\n");
                if params.show_output {
                    markdown.push_str("\n");
                    for output in &cell.outputs {
                        markdown.push_str(output);
                    }
                    markdown.push_str("\n");
                }
            }
            CellType::Markdown => {
                markdown.push_str(&cell.source);
                markdown.push_str("\n");
                markdown.push_str("\n");
            }
        }
    }
    markdown
}
