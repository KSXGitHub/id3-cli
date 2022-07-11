use clap::{Args, Subcommand};

/// Field-specific subcommand.
#[derive(Debug, Subcommand)]
pub enum Field<TextArgs: Args, FrameArgs: Args> {
    /// Text field.
    #[clap(flatten)]
    Text(Text<TextArgs>),
    /// Frame field.
    #[clap(flatten)]
    Frame(Frame<FrameArgs>),
}

/// Text field subcommand.
#[derive(Debug, Subcommand)]
pub enum Text<TextArgs: Args> {
    Title(TextArgs),
    Artist(TextArgs),
    Album(TextArgs),
    AlbumArtist(TextArgs),
    Genre(TextArgs),
}

/// Frame field subcommand.
#[derive(Debug, Subcommand)]
pub enum Frame<FrameArgs: Args> {
    Picture(FrameArgs),
    Comment(FrameArgs),
}
