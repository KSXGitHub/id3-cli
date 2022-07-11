use crate::args::{
    field::{Field, Frame, Text},
    view::{CommentViewArgs, PictureViewArgs, TextViewArgs, ViewCmd},
};
use id3::{Tag, TagLike};
use pipe_trait::Pipe;

pub fn view(args: ViewCmd) -> Result<(), String> {
    match args {
        Field::Text(Text::Title(args)) => view_text(args, Tag::title),
        Field::Text(Text::Artist(args)) => view_text(args, Tag::artist),
        Field::Text(Text::Album(args)) => view_text(args, Tag::album),
        Field::Text(Text::AlbumArtist(args)) => view_text(args, Tag::album_artist),
        Field::Text(Text::Genre(args)) => view_text(args, Tag::genre),
        _ => unimplemented!(),
    }
}

fn view_text(args: TextViewArgs, get: impl FnOnce(&Tag) -> Option<&str>) -> Result<(), String> {
    let TextViewArgs { input_audio } = args;
    let tag = input_audio
        .pipe(Tag::read_from_path)
        .map_err(|e| e.to_string())?;
    if let Some(title) = get(&tag) {
        println!("{title}");
    }
    Ok(())
}
