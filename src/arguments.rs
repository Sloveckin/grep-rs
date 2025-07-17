use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Mode {
    Left,
    Right,
    All,
    Reverse,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Green,
    Red,
    Blue,
}

#[derive(Parser, Debug)]
pub struct Args {
    /// String that need to find
    pub substring: String,

    /// File name
    pub file: String,

    /// Search from left side or right
    #[arg(short, long, value_enum, default_value_t = Mode::Left, ignore_case = true)]
    pub mode: Mode,

    /// Ignore case
    #[arg(short, long)]
    pub ignore_case: bool,

    #[command(flatten)]
    pub show_config: ShowConfig,
}

#[derive(Parser, Debug, Clone, Copy)]
pub struct ShowConfig {
    /// Show number of line
    #[arg(short, long)]
    pub number: bool,

    /// Color of selected part. Default = Red
    #[arg(short, long, value_enum, default_value_t = Color::Red, ignore_case = true)]
    pub color: Color,

    /// Size of selected window in string. Default value = 10
    #[arg(short, long, default_value = "10", ignore_case = true)]
    pub window_size: usize,
}
