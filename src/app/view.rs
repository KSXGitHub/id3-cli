use crate::args::{
    field::{Field, Frame, Text},
    text_format::TextFormat,
    view::{CommentViewArgs, PictureViewArgs, TextViewArgs, ViewCmd},
};
use id3::{Tag, TagLike};
use pipe_trait::Pipe;
use serde_json::json;

pub fn view(args: ViewCmd) -> Result<(), String> {
    match args {
        Field::Text(Text::Title(args)) => view_text(args, Tag::title),
        Field::Text(Text::Artist(args)) => view_text(args, Tag::artist),
        Field::Text(Text::Album(args)) => view_text(args, Tag::album),
        Field::Text(Text::AlbumArtist(args)) => view_text(args, Tag::album_artist),
        Field::Text(Text::Genre(args)) => view_text(args, Tag::genre),
        Field::Frame(Frame::Comment(args)) => view_comment(args),
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

fn view_comment(args: CommentViewArgs) -> Result<(), String> {
    let CommentViewArgs {
        lang,
        description,
        format,
        input_audio,
    } = args;
    let tag = input_audio
        .pipe(Tag::read_from_path)
        .map_err(|e| e.to_string())?;
    let comments: Vec<_> = tag
        .comments()
        .filter(|comment| lang.as_ref().map_or(true, |lang| &comment.lang == lang))
        .filter(|comment| {
            description
                .as_ref()
                .map_or(true, |description| &comment.description == description)
        })
        .map(|comment| {
            json!({
                "lang": comment.lang,
                "description": comment.description,
                "text": comment.text,
            })
        })
        .collect();
    let serialized = match format {
        TextFormat::Json => serde_json::to_string_pretty(&comments).map_err(|e| e.to_string())?,
        TextFormat::Toml => toml::to_string_pretty(&comments).map_err(|e| e.to_string())?,
        TextFormat::Yaml => serde_yaml::to_string(&comments).map_err(|e| e.to_string())?,
    };
    println!("{serialized}");
    Ok(())
}
