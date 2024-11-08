use crate::markdown_parser::{CellType, Notebook};
use crate::params::Params;
use crate::util::{get_directory, show_warning};
use std::path::Path;

/// Convert a notebook to markdown
///
/// # Arguments
/// * `notebook` - The notebook to convert
/// * `params` - The parameters to use
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
                    if !cell.images.is_empty() {
                        for image in &cell.images {
                            let path =
                                Path::new(&get_directory(&params.output_path)).join(&image.path);
                            image.data.save(&path).unwrap_or_else(|_| {
                                if let Some(path) = path.to_str() {
                                    show_warning(&format!("Could not save image {}", path));
                                } else {
                                    show_warning("Invalid image path");
                                }
                            });
                        }
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
