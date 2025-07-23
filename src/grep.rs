use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::{
    arguments::{Algo, Args, Mode},
    kmp,
    printer::{construct_line, construct_line_all, construct_reverse_line},
    searcher::Searcher,
};

pub fn grep(args: Args) -> Result<Vec<String>, std::io::Error> {
    match args.algo {
        Algo::Kmp => sub_grep(args, &kmp::KnuthMorrisPratt::default()),
        Algo::BoyerMoore => panic!("not implemented yet"),
    }
}

fn sub_grep(args: Args, algo: &impl Searcher) -> Result<Vec<String>, std::io::Error> {
    let reader = io::BufReader::new(File::open(&args.file)?);
    let data = get_update_functions(&args);
    let target = update_string(args.substring, &data);

    let mut result = Vec::new();

    for (pos, line) in reader.lines().map_while(Result::ok).enumerate() {
        let line = update_string(line, &data);

        match args.mode {
            Mode::Left => {
                let res = algo.search_left(&target, &line);

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
                let res = algo.search_right(&target, &line);

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
                if algo.reverse(&target, &line) {
                    result.push(construct_reverse_line(line, pos, &args.show_config));
                }
            }
            Mode::All => {
                let res = algo.search_all(&target, &line);

                if let Some(vec) = res {
                    result.push(construct_line_all(
                        &line,
                        pos,
                        &target,
                        vec,
                        &args.show_config,
                    ));
                }
            }
            Mode::Whole => {
                let res = algo.search_left(&target, &line);

                if let Some((l, r)) = res {
                    if l == 0 && r == target.len() {
                        result.push(construct_line(l, r, (pos, line), &args.show_config));
                    }
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
