use clap::{Args, Subcommand};
use std::fmt::Debug;

/// Table of [`Args`] types to used in [`Field`].
pub trait ArgsTable {
    /// CLI arguments for text fields.
    type Text: Args;
    /// CLI arguments for the picture field.
    type Picture: Args;
    /// CLI arguments for the comment field.
    type Comment: Args;
}

/// Field-specific subcommand.
#[derive(Debug, Subcommand)]
pub enum Field<Args>
where
    Args: ArgsTable,
    Args::Text: Debug,
    Args::Comment: Debug,
    Args::Picture: Debug,
{
    /// Text field.
    #[clap(flatten)]
    Text(Text<Args>),
    /// Frame field.
    #[clap(flatten)]
    Frame(Frame<Args>),
}

/// Text field subcommand.
#[derive(Debug, Subcommand)]
pub enum Text<Args: ArgsTable> {
    Title(Args::Text),
    Artist(Args::Text),
    Album(Args::Text),
    AlbumArtist(Args::Text),
    Genre(Args::Text),
}

/// Frame field subcommand.
#[derive(Debug, Subcommand)]
pub enum Frame<Args: ArgsTable> {
    Picture(Args::Picture),
    Comment(Args::Comment),
}
