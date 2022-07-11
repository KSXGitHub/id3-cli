pub mod view;

use crate::{args::CliArgs, error::Error};
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
        dbg!(self.args);
        Ok(())
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
