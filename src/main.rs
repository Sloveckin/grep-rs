use std::process::ExitCode;

use clap::Parser;

use crate::arguments::Args;

mod arguments;
mod grep;
mod grep_error;
mod kmp;
mod printer;
mod searcher;

fn main() -> ExitCode {
    let args = Args::parse();
    match grep::grep(args) {
        Ok(lines) => {
            lines.iter().for_each(|line| println!("{line}"));
            ExitCode::SUCCESS
        }
        Err(err) => {
            err.display();
            ExitCode::FAILURE
        }
    }
}
