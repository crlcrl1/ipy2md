mod markdown_writer;
mod params;
mod parser;

use clap::Parser;
use params::Params;
use parser::Notebook;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let params = Params::parse();
    let input = if let Ok(input) = read_to_string(&params.input_path) {
        input
    } else {
        show_error("Could not read the input file");
    };
    let notebook = Notebook::from_string(&input).unwrap_or_else(|| {
        show_error("Could not parse the input file");
    });
    let markdown = markdown_writer::get_markdown_string(&notebook, &params);
    if let Err(e) = std::fs::write(&params.output_path, markdown) {
        show_error(&format!("Could not write the output file: {}", e));
    }
}

fn show_error(msg: &str) -> ! {
    eprintln!("\x1b[1;31m{}\x1b[0m", msg);
    exit(1);
}
