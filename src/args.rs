use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // /// Fetch problem statement from AOC
    // // Prints to stdout
    // #[command(short_flag = 'p')]
    // Problem(Problem),
    /// Fetch your inputs from AOC
    #[command(short_flag = 'i')]
    Inputs(Inputs),
    /// Manage AOC session token
    #[command(short_flag = 'c', arg_required_else_help = true)]
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
    /// Set output directory for fetched inputs; `./inputs` by default
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<String>,
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
