pub mod view;

use crate::{
    args::{CliArgs, CliCmd},
    error::Error,
};
use clap::Parser;
use std::process::ExitCode;

/// Application parameters.
#[derive(Debug)]
pub struct App {
    /// CLI arguments.
    pub args: CliArgs,
}

impl App {
    /// Initialize application from environment.
    pub fn from_env() -> Self {
        let args = CliArgs::parse();
        App { args }
    }

    /// Run the application.
    pub fn run(self) -> Result<(), Error> {
        let CliArgs { command } = self.args;
        match command {
            CliCmd::View(args) => view::view(args),
        }
    }

    /// The main program.
    pub(crate) fn main() -> ExitCode {
        match App::from_env().run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        }
    }
}
