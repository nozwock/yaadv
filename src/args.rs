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
    /// Fetch your AOC inputs
    #[command(short_flag = 'I')]
    Inputs(Inputs),
    /// Manage your AOC session token
    #[command(
        short_flag = 'C',
        arg_required_else_help = true,
        after_help = r#"To learn how to get your session token, take a look at:
https://github.com/nozwock/yaadv#setting-up-the-cli"#
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
        long_help = r#"Set formatted output path for fetched inputs
Valid subtituted tokens: `{{day}}`, `{{year}}`
For eg. `yaadv -Id 1 -y 2022 -p "./inputs/day{{day}}.input"` will generate "./inputs/day1.input""#
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
