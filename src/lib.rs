pub mod app;
pub mod backup;
pub mod error;
pub mod run;
pub mod text_data;
pub mod text_format;
pub mod utils;

/// The main program.
pub fn main() -> std::process::ExitCode {
    app::App::main()
}

pub use chrono;
pub use clap;
pub use id3;
