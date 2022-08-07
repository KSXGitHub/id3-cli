use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use id3_cli::app::App;
use std::{fs::File, io, path::PathBuf, process::ExitCode};
use thiserror::Error;

/// Generate completions.
#[derive(Debug, Parser)]
struct ComGen {
    /// Bin name.
    #[clap(long, short)]
    name: String,
    /// Target shell.
    #[clap(long, short, value_enum)]
    shell: Shell,
    /// Output file.
    #[clap(long, short)]
    output: PathBuf,
}

/// Error of the generator.
#[derive(Debug, Error)]
enum Error {
    #[error("{}: {error}", file.to_string_lossy())]
    FileSystem {
        file: PathBuf,
        #[source]
        error: io::Error,
    },
}

impl ComGen {
    /// Run the generator.
    fn run(self) -> Result<(), Error> {
        let ComGen {
            name,
            shell,
            output,
        } = self;
        let mut cmd = App::command();
        let mut output_file = File::create(&output).map_err(|error| Error::FileSystem {
            file: output.clone(),
            error,
        })?;
        generate(shell, &mut cmd, name, &mut output_file);
        Ok(())
    }

    /// The program that generates shell completions.
    fn main() -> ExitCode {
        match ComGen::parse().run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        }
    }
}

fn main() -> ExitCode {
    ComGen::main()
}
