use chrono::Datelike;
use clap::Parser;
use yadv::args::Cli;

fn main() {
    let cli = Cli::parse();
    let curr_aoc_yr;
    {
        let curr = chrono::Utc::now().date_naive();
        let mut yr = curr.year();
        if curr.month() != 12 {
            yr -= 1;
        }
        curr_aoc_yr = yr;
    }

    dbg!(&cli);
    dbg!(&curr_aoc_yr);
}
