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
    use std::{io::Write, vec};
    use tempfile::NamedTempFile;

    use crate::{
        arguments::{Algo, Args, Color, Mode, ShowConfig},
        grep,
    };

    fn create_show_config() -> ShowConfig {
        ShowConfig {
            number: false,
            color: Color::Red,
            window_size: 10,
        }
    }

    pub fn new_with_default(substring: String, file: String) -> Args {
        Args {
            substring,
            file,
            mode: Mode::All,
            algo: Algo::Kmp,
            ignore_case: true,
            show_config: create_show_config(),
        }
    }

    fn create_wanted_string(vec: Vec<String>) -> String {
        vec.iter().fold(String::new(), |acc, v| acc + v)
    }

    fn create_file(lines: Vec<&str>) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();

        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }

        file
    }

    #[test]
    fn left_test() {
        let substring = String::from("aba");
        let file = create_file(vec!["aba", "abacaba"]);

        let mut args = new_with_default(substring, file.path().to_str().unwrap().to_string());
        args.mode = Mode::Left;

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
                        "aba".to_string()
                    ])
                );
            }
            Err(_) => panic!("not expected branch"),
        }
    }

    #[test]
    fn right_test() {
        let substring = String::from("aba");
        let file = create_file(vec!["aba", "abacaba"]);

        let mut args = new_with_default(substring, file.path().to_str().unwrap().to_string());
        args.mode = Mode::Right;

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
                        "aba".to_string(),
                        "c".to_string(),
                        "aba".red().to_string()
                    ])
                );
            }
            Err(_) => panic!("not expected branch"),
        }
    }

    fn test_color(
        substring: String,
        color_mode: Color,
        file_data: Vec<&str>,
        wanted: Vec<Vec<String>>,
    ) {
        let file = create_file(file_data);

        let mut args = new_with_default(substring, file.path().to_str().unwrap().to_string());
        args.show_config.color = color_mode;

        match grep::grep(args) {
            Ok(lines) => {
                assert_eq!(lines.len(), wanted.len());

                for i in 0..lines.len() {
                    assert_eq!(lines[i], create_wanted_string(wanted[i].clone()))
                }
            }
            Err(_) => panic!("not expected branch"),
        }
    }

    #[test]
    fn red_test() {
        let first = vec!["aba".red().to_string()];
        let second = vec![
            "aba".red().to_string(),
            "c".to_string(),
            "aba".red().to_string(),
        ];

        test_color(
            "aba".to_string(),
            Color::Red,
            vec!["aba", "abacaba"],
            vec![first, second],
        );
    }

    #[test]
    fn green_color() {
        let first = vec!["aba".green().to_string()];
        let second = vec![
            "aba".green().to_string(),
            "c".to_string(),
            "aba".green().to_string(),
        ];
        test_color(
            "aba".to_string(),
            Color::Green,
            vec!["aba", "abacaba"],
            vec![first, second],
        );
    }

    #[test]
    fn blue_color() {
        let first = vec!["aba".blue().to_string()];
        let second = vec![
            "aba".blue().to_string(),
            "c".to_string(),
            "aba".blue().to_string(),
        ];
        test_color(
            "aba".to_string(),
            Color::Blue,
            vec!["aba", "abacaba"],
            vec![first, second],
        );
    }

    #[test]
    fn with_number() {
        let substring = String::from("Hello");
        let file = create_file(vec!["hehe", "Hello, World", "Hello"]);

        let mut args = new_with_default(substring, file.path().to_str().unwrap().to_string());
        args.show_config.number = true;

        match grep::grep(args) {
            Ok(lines) => {
                assert_eq!(lines.len(), 2);

                // To lower case, because default flag for ignoring this = true
                assert_eq!(
                    lines[0],
                    create_wanted_string(vec![
                        "2:".to_string(),
                        "hello".red().to_string(),
                        ", world".to_string(),
                    ])
                );

                assert_eq!(
                    lines[1],
                    create_wanted_string(vec!["3:".to_string(), "hello".red().to_string(),])
                );
            }
            Err(_) => panic!("not expected error"),
        }
    }

    #[test]
    fn reverse() {
        let substring = String::from("no_line");
        let file = create_file(vec![
            "no_line",
            "this is true line",
            "a lot of text with no_line",
        ]);

        let mut args = new_with_default(substring, file.path().to_str().unwrap().to_string());
        args.mode = Mode::Reverse;

        match grep::grep(args) {
            Ok(lines) => {
                assert_eq!(lines.len(), 1);

                assert_eq!(
                    lines[0],
                    create_wanted_string(vec!["this is true line".to_string()])
                );
            }
            Err(_) => panic!("not expected branch"),
        }
    }
}
