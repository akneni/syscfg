use std::{ffi::OsString, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cli {
    pub command: CliCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    MkSnap { output: Option<PathBuf> },
    Save { output: Option<PathBuf> },
    Load { input: Option<PathBuf> },
}

pub fn parse() -> Cli {
    parse_from(std::env::args_os()).unwrap_or_else(|err| err.exit())
}

pub fn parse_from<I, T>(args: I) -> Result<Cli, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = command().try_get_matches_from(args)?;
    Ok(Cli::from_matches(&matches))
}

fn command() -> Command {
    Command::new("syscfg")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("mksnap").arg(
                Arg::new("output")
                    .short('o')
                    .value_name("path")
                    .value_parser(clap::value_parser!(PathBuf)),
            ),
        )
        .subcommand(
            Command::new("save").arg(
                Arg::new("output")
                    .short('o')
                    .value_name("path")
                    .value_parser(clap::value_parser!(PathBuf)),
            ),
        )
        .subcommand(
            Command::new("load").arg(
                Arg::new("input")
                    .short('i')
                    .value_name("path")
                    .value_parser(clap::value_parser!(PathBuf)),
            ),
        )
}

impl Cli {
    fn from_matches(matches: &ArgMatches) -> Self {
        let command = match matches.subcommand() {
            Some(("mksnap", matches)) => CliCommand::MkSnap {
                output: matches.get_one::<PathBuf>("output").cloned(),
            },
            Some(("save", matches)) => CliCommand::Save {
                output: matches.get_one::<PathBuf>("output").cloned(),
            },
            Some(("load", matches)) => CliCommand::Load {
                input: matches.get_one::<PathBuf>("input").cloned(),
            },
            _ => unreachable!("subcommand is required"),
        };

        Self { command }
    }
}
