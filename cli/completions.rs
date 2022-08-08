use clap_utilities::CommandFactoryExtra;
use id3_cli::app::App;
use std::process::ExitCode;

fn main() -> ExitCode {
    App::run_completion_generator()
}
