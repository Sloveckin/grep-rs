use std::{cmp::{max, min}, vec};

use colored::{ColoredString, Colorize};

use crate::arguments::{Color, ShowConfig};

pub fn construct_line(left: usize, right: usize, pair: (usize, String), show_config: &ShowConfig) -> String {

    let edge_size = show_config.window_size;

    let start = max(0, left.saturating_sub(edge_size));
    let end = min(pair.1.len(), right + edge_size);

    let substring = &pair.1[start..end];

    let pattern_start = left - start;
    let pattern_end = right - start;

    let left_part = &substring[..pattern_start];
    let pattern = &substring[pattern_start..pattern_end];
    let right_part = &substring[pattern_end..];

    let mut res = String::new();

    if show_config.number {
        res = res + &format!("{}:", pair.0 + 1);
    }

    res = res
        + &format!(
            "{}{}{}",
            left_part,
            take_color(pattern, show_config.color),
            right_part
        );

    res
}

pub fn construct_reverse_line(line: String, line_ind: usize, show_config: &ShowConfig) -> String {
    let funcs = get_update_functions(show_config, line_ind);

    update_string(line, funcs)
}

// I hate myself...
fn get_update_functions(config: &ShowConfig, line_ind: usize) -> Vec<Box<dyn Fn(String) -> String>> {
    let mut funcs: Vec<Box<dyn Fn(String) -> String>> = Vec::new();

    if config.number {
        funcs.push(Box::new(move |x: String| format!("{}:", line_ind + 1) + &x));
    }

    funcs
}


fn update_string<F>(src: String, funcs: Vec<F>) -> String 
    where F: Fn(String) -> String
{
    funcs.iter().fold(src, |src, f| f(src))
}

fn take_color(source: &str, color: Color) -> ColoredString {
    match color {
        Color::Red => source.red(),
        Color::Green => source.green(),
        Color::Blue => source.blue(),
    }
}
