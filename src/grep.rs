use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::{
    arguments::{Args, Mode},
    kmp,
    printer::{construct_line, construct_line_all, construct_reverse_line},
    searcher::Searcher,
};

pub fn grep(args: Args) -> Result<Vec<String>, std::io::Error> {
    let reader = io::BufReader::new(File::open(&args.file)?);

    let kmp = kmp::KnuthMorrisPratt::default();

    let data = get_update_functions(&args);

    let target = update_string(args.substring, &data);

    let mut result = Vec::new();

    for (pos, line) in reader.lines().map_while(Result::ok).enumerate() {
        let line = update_string(line, &data);

        match args.mode {
            Mode::Left => {
                let res = kmp.search_left(&target, &line);

                if let Some(pair) = res {
                    result.push(construct_line(
                        pair.0,
                        pair.1,
                        (pos, line),
                        &args.show_config,
                    ));
                }
            }
            Mode::Right => {
                let res = kmp.search_right(&target, &line);

                if let Some(pair) = res {
                    result.push(construct_line(
                        pair.0,
                        pair.1,
                        (pos, line),
                        &args.show_config,
                    ));
                }
            }
            Mode::Reverse => {
                // Can I write more simple?
                let res = kmp.reverse(&target, &line);

                if res.is_some() && res.unwrap() {

                }
            }
            Mode::All => {
                let res = kmp.search_all(&target, &line);

                if let Some(vec) = res {
                    println!("{line}, {:?}", vec);
                    result.push(construct_line_all(&line, pos, &target, vec, &args.show_config));
                }
            }
        }
    }

    match !result.is_empty() {
        true => Ok(result),
        false => panic!(),
    }
}

fn update_string<F>(src: String, funcs: &[F]) -> String
where
    F: Fn(String) -> String,
{
    funcs.iter().fold(src, |acc, f| f(acc))
}

fn get_update_functions(args: &Args) -> Vec<fn(String) -> String> {
    let mut funcs: Vec<fn(String) -> String> = Vec::new();

    if args.ignore_case {
        funcs.push(|x: String| x.to_lowercase());
    }

    funcs
}
