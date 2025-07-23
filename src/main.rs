use clap::Parser;

use crate::arguments::Args;

mod arguments;
mod grep;
mod kmp;
mod printer;
mod searcher;

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let result = grep::grep(args)?;

    for line in result {
        println!("{line}")
    }

    Ok(())
}
