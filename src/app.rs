pub mod backup;
pub mod delete;
pub mod field;
pub mod get;
pub mod set;

use crate::{error::Error, run::Run};
use backup::Backup;
use clap::{Parser, Subcommand};
use delete::Delete;
use get::Get;
use set::Set;
use std::process::ExitCode;

/// The main application.
#[derive(Debug, Parser)]
#[clap(name = "id3", version)]
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
    /// Run backup without modification.
    Backup(Backup),
    /// Show or export metadata.
    #[clap(subcommand)]
    Get(Get),
    /// Modify metadata.
    #[clap(subcommand)]
    Set(Set),
    /// Delete metadata.
    #[clap(subcommand)]
    Delete(Delete),
}

impl Run for AppCmd {
    fn run(self) -> Result<(), Error> {
        match self {
            AppCmd::Backup(proc) => proc.run(),
            AppCmd::Get(proc) => proc.run(),
            AppCmd::Set(proc) => proc.run(),
            AppCmd::Delete(proc) => proc.run(),
        }
    }
}
