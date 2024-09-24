mod markdown_writer;
mod params;
mod parser;
mod util;

use crate::util::{create_directory, get_directory, show_error};
use clap::Parser;
use params::Params;
use parser::Notebook;
use std::fs::read_to_string;

fn main() {
    let params = Params::parse();
    let input = if let Ok(input) = read_to_string(&params.input_path) {
        input
    } else {
        show_error("Could not read the input file");
    };
    // Get the notebook
    let notebook = Notebook::from_string(&input).unwrap_or_else(|| {
        show_error("Could not parse the input file");
    });
    // Generate markdown string
    let markdown_string = markdown_writer::get_markdown_string(&notebook, &params);

    create_directory(&get_directory(&params.output_path));
    if let Err(e) = std::fs::write(&params.output_path, markdown_string) {
        show_error(&format!("Could not write the output file: {}", e));
    }
}
