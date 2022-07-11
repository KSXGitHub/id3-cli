use crate::args::{
    field::{Field, Frame, Text},
    view::{CommentViewArgs, PictureViewArgs, TextViewArgs, ViewCmd},
};
use id3::{Tag, TagLike};
use pipe_trait::Pipe;

pub fn view(args: ViewCmd) -> Result<(), String> {
    match args {
        Field::Text(Text::Title(TextViewArgs { input })) => {
            let tag = input.pipe(Tag::read_from_path).map_err(|e| e.to_string())?;
            if let Some(title) = tag.title() {
                println!("{title}");
            }
            Ok(())
        }
        _ => unimplemented!(),
    }
}
