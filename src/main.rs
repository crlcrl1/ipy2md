mod markdown_parser;
mod markdown_writer;
mod params;
mod util;

use clap::Parser;
use markdown_parser::Notebook;
use params::Params;
use std::fs::read_to_string;
use util::{create_directory, get_directory, show_error};

fn main() {
    let params = Params::parse();
    let input = if let Ok(input) = read_to_string(&params.input_path) {
        input
    } else {
        show_error("Could not read the input file");
    };
    // Get the notebook
    let notebook = Notebook::from_string(&input, params.image_dir.clone()).unwrap_or_else(|| {
        show_error("Could not parse the input file");
    });
    create_directory(&get_directory(&params.output_path));
    create_directory(&format!(
        "{}/{}",
        get_directory(&params.output_path),
        &params.image_dir,
    ));

    // Generate markdown string
    let markdown_string = markdown_writer::get_markdown_string(&notebook, &params);

    if let Err(e) = std::fs::write(&params.output_path, markdown_string) {
        show_error(&format!("Could not write the output file: {}", e));
    }
}
