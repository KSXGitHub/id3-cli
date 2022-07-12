pub mod app;
pub mod args;
pub mod error;
pub mod run;
pub mod text_data;
pub mod text_format;

/// The main program.
pub fn main() -> std::process::ExitCode {
    app::App::main()
}

pub use clap;
pub use id3;
