use clap::Parser;

use crate::arguments::Args;

mod arguments;
mod grep;
mod kmp;
mod printer;
mod searcher;

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    for line in grep::grep(args)? {
        println!("{line}")
    }

    Ok(())
}
