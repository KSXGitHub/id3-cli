pub mod field;
pub mod view;

use clap::{Args, Parser, Subcommand};
use view::ViewCmd;

/// CLI arguments of the program.
#[derive(Debug, Parser)]
#[clap(name = "id3")]
pub struct CliArgs {
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub command: CliCmd,
}

/// Subcommand of the program.
#[derive(Debug, Subcommand)]
pub enum CliCmd {
    /// View metadata.
    #[clap(subcommand)]
    View(ViewCmd),
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
    pub no_backup: bool,
}

/// CLI arguments of the `erase` subcommand.
#[derive(Debug, Args)]
pub struct EraseArgs {
    /// Don't create backup.
    #[clap(long)]
    pub no_backup: bool,
}
