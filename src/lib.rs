pub mod app;
pub mod args;
pub mod text_data;

/// The main program.
pub fn main() -> std::process::ExitCode {
    app::App::main()
}

pub use clap;
pub use id3;
