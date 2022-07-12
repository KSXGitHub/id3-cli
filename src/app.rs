pub mod field;
pub mod view;

use crate::{error::Error, run::Run};
use clap::{Args, Parser, Subcommand};
use std::process::ExitCode;
use view::ViewCmd;

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
    /// View metadata.
    #[clap(subcommand)]
    View(ViewCmd),
    // /// Modify a field of metadata.
    // Modify(ModifyArgs),
    // /// Erase all metadata.
    // Erase(EraseArgs),
}

impl Run for AppCmd {
    fn run(self) -> Result<(), Error> {
        match self {
            AppCmd::View(cmd) => cmd.run(),
        }
    }
}

/// CLI arguments of the `modify` subcommand.
#[derive(Debug, Args)]
pub struct ModifyArgs {
    /// Don't create backup.
    #[clap(long)]
    pub no_backup: bool,
}

/// CLI arguments of the `erase` subcommand.
#[derive(Debug, Args)]
pub struct EraseArgs {
    /// Don't create backup.
    #[clap(long)]
    pub no_backup: bool,
}
