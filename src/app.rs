pub mod field;
pub mod get;
pub mod picture_type;
pub mod set;

use crate::{error::Error, run::Run};
use clap::{Parser, Subcommand};
use get::Get;
use set::Set;
use std::process::ExitCode;

/// The main application.
#[derive(Debug, Parser)]
#[clap(name = "id3")]
pub struct App {
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub command: AppCmd,
}

impl App {
    /// The main program.
    pub(crate) fn main() -> ExitCode {
        match App::parse().run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        }
    }
}

impl Run for App {
    fn run(self) -> Result<(), Error> {
        let App { command } = self;
        command.run()
    }
}

/// Subcommand of the program.
#[derive(Debug, Subcommand)]
pub enum AppCmd {
    /// Show or export metadata.
    #[clap(subcommand)]
    Get(Get),
    /// Modify metadata.
    #[clap(subcommand)]
    Set(Set),
    // TODO: id3 set [--no-backup]
    // TODO: id3 delete [--no-backup]
}

impl Run for AppCmd {
    fn run(self) -> Result<(), Error> {
        match self {
            AppCmd::Get(proc) => proc.run(),
            AppCmd::Set(proc) => proc.run(),
        }
    }
}
