use chrono::Datelike;
use clap::Parser;
use colored::*;
use std::process;
use yadv::{args::Cli, credentials::Secrets};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let curr_aoc_yr;
    {
        let curr = chrono::Utc::now().naive_utc();
        let mut yr = curr.year();
        // since AOC starts in december
        if curr.month() != 12 {
            yr -= 1;
        }
        curr_aoc_yr = yr;
    }

    match cli.command {
        yadv::args::Commands::Inputs(_inputs) => {}
        yadv::args::Commands::Credentials(creds) => {
            if let Some(token) = creds.token {
                Secrets {
                    session_token: Some(token),
                }
                .store()?;
            }

            if creds.interactive {
                let token = inquire::Password::new("Your session token:")
                    .with_display_mode(inquire::PasswordDisplayMode::Masked)
                    .without_confirmation()
                    .prompt()?;

                let old_token = Secrets::load();
                if old_token.get_session_token().is_some() {
                    let confirm = inquire::Confirm::new(
                        "Your previous session token will be overwritten, continue?",
                    )
                    .with_default(false)
                    .prompt()?;
                    if !confirm {
                        process::exit(0);
                    }
                }
                Secrets {
                    session_token: Some(token),
                }
                .store()?;
            }

            if creds.show {
                let token = Secrets::load();
                match token.get_session_token() {
                    Some(token) => println!("Your session token: {}", token.bright_blue()),
                    None => {
                        println!("{}", "No session token found!".red());
                        process::exit(1);
                    }
                }
            }
        }
    }

    Ok(())
}
