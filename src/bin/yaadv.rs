use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, io::Write, process, time::Duration};
use yaadv::{api::fetch_inputs, args::Cli, credentials::Secrets, inputs::AdvInput};

fn download_inputs(inputs: &Vec<AdvInput>, session_token: &str) -> Result<Vec<String>> {
    fs::create_dir_all(
        inputs
            .iter()
            .next()
            .context("no input file to download")?
            .path()
            .parent()
            .context("no parent folder exists")?,
    )?;

    let mut out_err = vec![];

    for (input, resp) in fetch_inputs(inputs, session_token)
        .into_iter()
        .enumerate()
        .map(|(i, resp)| (&inputs[i], resp))
    {
        match resp {
            Ok(resp) => fs::File::create(input.path())?.write_all(resp.as_bytes())?,
            Err(err) => out_err.push(format!("{} {}", "Error:".red(), err.to_string().red())),
        };
    }

    Ok(out_err)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        yaadv::args::Commands::Inputs(inputs) => {
            let days = if let Some(day) = inputs.day {
                vec![day]
            } else {
                (1..=25).collect()
            };

            let year = if let Some(year) = inputs.year {
                year
            } else {
                let curr = chrono::Utc::now().naive_utc();
                let mut yr = curr.year();
                // since AOC starts in december
                if curr.month() != 12 {
                    yr -= 1;
                }
                yr
            };

            let sp = ProgressBar::new_spinner();
            sp.set_message("Downloading...");
            sp.enable_steady_tick(Duration::from_millis(80));
            sp.set_style(
                ProgressStyle::with_template("{spinner:.blue} {msg}")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );

            let inputs = days
                .into_iter()
                .map(|day| {
                    AdvInput::new(day, year)
                        .with_formatted_path(inputs.formatted_path.as_ref().map(|s| s.as_str()))
                })
                .collect();

            let errs = download_inputs(
                &inputs,
                &Secrets::load()
                    .session_token
                    .context("No session token found!\nPlease add a sesssion token first")?,
            )?;

            sp.finish_and_clear();
            errs.into_iter().for_each(|err| eprintln!("{}", err));
            println!("{}", "Done downloading input files!".green());
        }
        yaadv::args::Commands::Credentials(creds) => {
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
                    Some(token) => println!("Your session token: {}", token.bright_cyan()),
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
