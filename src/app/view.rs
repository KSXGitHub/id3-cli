use crate::{
    args::{
        field::{Field, Frame, Text},
        view::{
            CommentViewArgs, PictureFileArgs, PictureListArgs, PictureViewArgs, PictureViewCmd,
            TextViewArgs, ViewCmd,
        },
    },
    error::{
        Error, NoPicTypeMultiPic, PictureFileWriteFailure, PictureNotFound, PictureTypeNotFound,
    },
    text_data::picture::Picture,
};
use id3::{Tag, TagLike};
use pipe_trait::Pipe;
use serde_json::json;
use std::fs::write;

pub fn view(args: ViewCmd) -> Result<(), Error> {
    match args {
        Field::Text(Text::Title(args)) => view_text(args, Tag::title),
        Field::Text(Text::Artist(args)) => view_text(args, Tag::artist),
        Field::Text(Text::Album(args)) => view_text(args, Tag::album),
        Field::Text(Text::AlbumArtist(args)) => view_text(args, Tag::album_artist),
        Field::Text(Text::Genre(args)) => view_text(args, Tag::genre),
        Field::Frame(Frame::Comment(args)) => view_comment(args),
        Field::Frame(Frame::Picture(args)) => view_picture(args),
    }
}

fn view_text(args: TextViewArgs, get: impl FnOnce(&Tag) -> Option<&str>) -> Result<(), Error> {
    let TextViewArgs { input_audio } = args;
    let tag = input_audio.pipe(Tag::read_from_path)?;
    if let Some(title) = get(&tag) {
        println!("{title}");
    }
    Ok(())
}

fn view_comment(args: CommentViewArgs) -> Result<(), Error> {
    let CommentViewArgs {
        lang,
        description,
        format,
        input_audio,
    } = args;
    let tag = input_audio.pipe(Tag::read_from_path)?;
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
    let serialized = format.serialize(&comments)?;
    println!("{serialized}");
    Ok(())
}

fn view_picture(args: PictureViewArgs) -> Result<(), Error> {
    let PictureViewArgs { command } = args;
    match command {
        PictureViewCmd::List(args) => view_picture_list(args),
        PictureViewCmd::File(args) => view_picture_file(args),
    }
}

fn view_picture_list(args: PictureListArgs) -> Result<(), Error> {
    let PictureListArgs {
        format,
        input_audio,
    } = args;
    let tag = input_audio.pipe(Tag::read_from_path)?;
    let pictures: Vec<_> = tag.pictures().map(Picture::from_id3_ref).collect();
    let serialized = format.serialize(&pictures)?;
    println!("{serialized}");
    Ok(())
}

fn view_picture_file(args: PictureFileArgs) -> Result<(), Error> {
    let PictureFileArgs {
        picture_type,
        input_audio,
        output_picture,
    } = args;
    let tag = input_audio.pipe(Tag::read_from_path)?;
    let data = if let Some(picture_type) = picture_type {
        let lowercase_picture_type = picture_type.to_lowercase();
        &tag.pictures()
            .find(|picture| {
                picture.picture_type.to_string().to_lowercase() == lowercase_picture_type
            })
            .ok_or_else(|| PictureTypeNotFound { picture_type })?
            .data
    } else {
        let mut iter = tag.pictures().map(|picture| &picture.data);
        let data = iter.next().ok_or_else(|| PictureNotFound)?;
        if iter.next().is_some() {
            return NoPicTypeMultiPic.pipe(Error::from).pipe(Err);
        }
        data
    };
    write(output_picture, data)
        .map_err(PictureFileWriteFailure::from)
        .map_err(Error::from)
}
