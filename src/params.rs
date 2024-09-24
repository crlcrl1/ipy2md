use clap::Parser;

/// The command line parameters
#[derive(Debug, Parser)]
#[command(
    version,
    about = "A simple CLI app to generate markdown files from ipynb files"
)]
pub(crate) struct Params {
    #[arg(short, long = "input", help = "The input ipynb file", required = true)]
    pub(crate) input_path: String,

    #[arg(
        short,
        long = "output",
        default_value = "output.md",
        help = "The output file"
    )]
    pub(crate) output_path: String,

    #[arg(short, long, help = "Whether to show the output in the ipynb file")]
    pub(crate) show_output: bool,

    #[arg(
        short,
        long,
        help = "Whether to show separator between blocks",
        default_value = "false"
    )]
    pub(crate) block_separator: bool,
}
