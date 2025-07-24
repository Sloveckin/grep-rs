use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Mode {
    Left,
    Right,
    All,
    Reverse,
    Whole,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Algo {
    Kmp,
    BoyerMoore,
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
    #[arg(short, long, value_enum, default_value_t = Mode::All, ignore_case = true)]
    pub mode: Mode,

    #[arg(short, long, value_enum, default_value_t = Algo::Kmp, ignore_case = true)]
    pub algo: Algo,

    /// Ignore case
    #[arg(short, long)]
    pub ignore_case: bool,

    #[command(flatten)]
    pub show_config: ShowConfig,
}


impl Args {
    pub fn new_with_default(substring: String, file: String) -> Self {
        Args { substring, file, mode: Mode::All, algo: Algo::Kmp, ignore_case: true, show_config: ShowConfig::default() } 
    }
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

/// For testing
impl Default for ShowConfig {
    fn default() -> Self {
        ShowConfig {
            number: false,
            color: Color::Red,
            window_size: 10,
        }
    }
}
