use crate::{error::Error, run::Run};
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

impl<Args> Run for Field<Args>
where
    Args: ArgsTable,
    Args::Text: Debug,
    Args::Comment: Debug,
    Args::Picture: Debug,
    Text<Args>: Run,
    Frame<Args>: Run,
{
    fn run(self) -> Result<(), Error> {
        match self {
            Field::Text(proc) => proc.run(),
            Field::Frame(proc) => proc.run(),
        }
    }
}

/// Text field subcommand.
#[derive(Debug, Subcommand)]
pub enum Text<Args: ArgsTable> {
    Title(Args::Text),
    Artist(Args::Text),
    Album(Args::Text),
    AlbumArtist(Args::Text),
    Genre(Args::Text),
    GenreCode(Args::Text),
}

/// Frame field subcommand.
#[derive(Debug, Subcommand)]
pub enum Frame<Args: ArgsTable> {
    Comment(Args::Comment),
    Picture(Args::Picture),
}

impl<Args> Run for Frame<Args>
where
    Args: ArgsTable,
    Args::Comment: Run,
    Args::Picture: Run,
{
    fn run(self) -> Result<(), Error> {
        match self {
            Frame::Comment(proc) => proc.run(),
            Frame::Picture(proc) => proc.run(),
        }
    }
}
