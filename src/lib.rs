pub mod app;
pub mod args;

/// The main program.
pub fn main() -> std::process::ExitCode {
    app::App::main()
}

pub use clap;
pub use id3;
