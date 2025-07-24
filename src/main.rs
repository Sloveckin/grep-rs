use clap::Parser;
use std::process::ExitCode;

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

#[cfg(test)]
mod tests {
    use colored::Colorize;
    use std::io::Write;
    use tempfile::NamedTempFile;

    use crate::{arguments::Args, grep};

    fn create_wanted_string(vec: Vec<String>) -> String {
        vec.iter().fold(String::new(), |acc, v| acc + v)
    }

    fn fill_file(file: &mut NamedTempFile, lines: Vec<&str>) -> std::io::Result<()> {
        for line in lines {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    #[test]
    fn simple_test() -> std::io::Result<()> {
        let mut file = NamedTempFile::new()?;
        let substring = String::from("aba");
        fill_file(&mut file, vec!["aba", "abacaba"])?;

        let args = Args::new_with_default(substring, file.path().to_str().unwrap().to_string());

        match grep::grep(args) {
            Ok(lines) => {
                assert_eq!(lines.len(), 2);
                assert_eq!(
                    lines[0],
                    create_wanted_string(vec!["aba".red().to_string()])
                );
                assert_eq!(
                    lines[1],
                    create_wanted_string(vec![
                        "aba".red().to_string(),
                        "c".to_string(),
                        "aba".red().to_string()
                    ])
                );

                Ok(())
            },
            Err(_) => panic!("not expected branch")
        }
    }
}
