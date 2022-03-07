pub mod cli;
use crate::cli::runner::run;
use crate::cli::parser::CliParser;
use clap::{Parser};

fn main() {
    // Parse input args
    let cli = CliParser::parse();
    // Run using CLI args
    run(&cli);
}
