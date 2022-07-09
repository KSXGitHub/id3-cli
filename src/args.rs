use clap::{Args, Parser, Subcommand};

/// CLI arguments of the program.
#[derive(Debug, Parser)]
pub struct CliArgs {
    /// Subcommand to execute.
    #[clap(subcommand)]
    command: CliCmd,
}

/// Subcommand of the program.
#[derive(Debug, Subcommand)]
pub enum CliCmd {
    /// View metadata.
    View,
    /// Modify a field of metadata.
    Modify(ModifyArgs),
    /// Erase all metadata.
    Erase(EraseArgs),
}

/// CLI arguments of the `modify` subcommand.
#[derive(Debug, Args)]
pub struct ModifyArgs {
    /// Don't create backup.
    #[clap(long)]
    no_backup: bool,
}

/// CLI arguments of the `erase` subcommand.
#[derive(Debug, Args)]
pub struct EraseArgs {
    /// Don't create backup.
    #[clap(long)]
    no_backup: bool,
}
