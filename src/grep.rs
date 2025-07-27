use std::{
    fs::File,
    io::{self, BufRead},
    rc::Rc,
};

use crate::{
    arguments::{Algo, Args, Mode},
    grep_error::{ErrorType, GrepResult},
    kmp,
    printer::{construct_line, construct_line_all, construct_reverse_line},
    searcher::{SearchResult, Searcher},
};

struct DataHandler<'a> {
    target: &'a str,
    line: &'a str,
    line_pos: usize,
    result: &'a mut Vec<String>,
    args: &'a Args,
    searcher: Rc<dyn Searcher>,
}

pub fn grep(args: Args) -> GrepResult<Vec<String>> {
    match args.algo {
        Algo::Kmp => sub_grep(args, Rc::new(kmp::KnuthMorrisPratt::default())),
        Algo::BoyerMoore => panic!("not implemented yet"),
    }
}

fn sub_grep(args: Args, searcher: Rc<dyn Searcher>) -> GrepResult<Vec<String>> {
    match File::open(&args.file) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            let data = get_update_functions(&args);
            let target = update_string(&args.substring, &data);

            let mut result = Vec::new();

            for (line_pos, line) in reader.lines().map_while(Result::ok).enumerate() {
                let line = update_string(&line, &data);

                let mut data_handler = DataHandler {
                    target: &target,
                    line: &line,
                    line_pos,
                    result: &mut result,
                    args: &args,
                    searcher: searcher.clone(),
                };

                mode_handle(&mut data_handler);
            }

            if result.is_empty() {
                return Err(ErrorType::NotFound);
            }

            Ok(result)
        }
        Err(err) => Err(ErrorType::IOError(Rc::new(err))),
    }
}

fn mode_handle(data_handler: &mut DataHandler) {
    match data_handler.args.mode {
        Mode::Left => handle_left(data_handler),
        Mode::Right => handle_right(data_handler),
        Mode::Reverse => handle_reverse(data_handler),
        Mode::All => handle_all(data_handler),
        Mode::Whole => handle_whole(data_handler),
    }
}

fn handle_left(data_handler: &mut DataHandler) {
    let res = data_handler
        .searcher
        .search_left(data_handler.target, data_handler.line);
    construct_left_right(data_handler, res);
}

fn handle_right(data_handler: &mut DataHandler) {
    let res = data_handler
        .searcher
        .search_right(data_handler.target, data_handler.line);
    construct_left_right(data_handler, res);
}

fn handle_reverse(data_handler: &mut DataHandler) {
    if !data_handler
        .searcher
        .reverse(data_handler.target, data_handler.line)
    {
        return;
    }

    let construct = construct_reverse_line(
        String::from(data_handler.line),
        data_handler.line_pos,
        &data_handler.args.show_config,
    );
    data_handler.result.push(construct);
}

fn handle_all(data_handler: &mut DataHandler) {
    let res = data_handler
        .searcher
        .search_all(data_handler.target, data_handler.line);

    if let Some(vec) = res {
        if vec.is_empty() {
            return;
        }

        data_handler.result.push(construct_line_all(
            data_handler.line,
            data_handler.line_pos,
            data_handler.target,
            vec,
            &data_handler.args.show_config,
        ));
    }
}

fn handle_whole(data_handler: &mut DataHandler) {
    let res = data_handler
        .searcher
        .search_left(data_handler.target, data_handler.line);

    if let Some((l, r)) = res {
        if l == 0 && r == data_handler.line.len() {
            let line = construct_line(
                l,
                r,
                (data_handler.line_pos, String::from(data_handler.line)),
                &data_handler.args.show_config,
            );
            data_handler.result.push(line);
        }
    }
}

fn construct_left_right(data_handler: &mut DataHandler, res: SearchResult) {
    if let Some(pair) = res {
        let res = construct_line(
            pair.0,
            pair.1,
            (data_handler.line_pos, String::from(data_handler.line)),
            &data_handler.args.show_config,
        );
        data_handler.result.push(res);
    }
}

fn update_string<F>(src: &str, funcs: &[F]) -> String
where
    F: Fn(String) -> String,
{
    let tmp = String::from(src);
    funcs.iter().fold(tmp, |acc, f| f(acc))
}

fn get_update_functions(args: &Args) -> Vec<fn(String) -> String> {
    let mut funcs: Vec<fn(String) -> String> = Vec::new();

    if args.ignore_case {
        funcs.push(|x: String| x.to_lowercase());
    }

    funcs
}
