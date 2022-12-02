use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // ! Not certain if I want to add this feat...though, If I did, I'd also have to add a feat for submiting answers aswell.
    // /// Fetch problem statement from AOC
    // // Prints to stdout
    // #[command(short_flag = 'p')]
    // Problem(Problem),
    /// Fetch your inputs from AOC
    #[command(short_flag = 'I')]
    Inputs(Inputs),
    /// Manage your AOC session token
    #[command(
        short_flag = 'C',
        arg_required_else_help = true,
        after_long_help = r#"
How to get your session token from browser?
    1. In a logged in session, get your session token from Cookies, under the "Storage"/"Application" tab, using Firefox/Chromium devtools respectively.
    2. Or instead open the "Network" tab in devtools, and get the token from the Cookie header.
    "#
    )]
    Credentials(Credentials),
}

// #[derive(Args, Debug)]
// pub struct Problem {
//     #[arg(short, long, required = true)]
//     pub day: Option<u32>,
//     /// Current AOC year by default
//     #[arg(short, long)]
//     pub year: Option<i32>,
// }

#[derive(Args, Debug)]
pub struct Inputs {
    /// All 25 days by default
    #[arg(short, long)]
    pub day: Option<u32>,
    /// Current AOC year by default
    #[arg(short, long)]
    pub year: Option<i32>,
    /// Set formatted output path for fetched inputs
    #[arg(
        short = 'o',
        long,
        value_name = "PATTERN",
        long_help = r#"
Set formatted output path for fetched inputs
Valid subtituted tokens: `{{day}}`, `{{year}}`
For eg. `yadv -Id 1 -y 2022 -p "./inputs/day{{day}}.input"` will generate "./inputs/day1.input"
    "#
    )]
    pub formatted_path: Option<String>,
}

#[derive(Args, Debug)]
pub struct Credentials {
    /// Show stored session token
    #[arg(short, long, exclusive = true)]
    pub show: bool,
    /// Set session token in interactive mode
    #[arg(short, long, exclusive = true)]
    pub interactive: bool,
    /// Set session token
    #[arg(short, long, exclusive = true)]
    pub token: Option<String>,
}
