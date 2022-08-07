use crate::text_data::picture_type::PictureTypeExtra;
use id3::frame::Picture as Id3Picture;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Picture {
    pub mime_type: String,
    pub picture_type: PictureTypeExtra,
    pub description: String,
    pub size: usize,
}

impl Picture {
    pub fn from_id3_ref(source: &Id3Picture) -> Self {
        let Id3Picture {
            mime_type,
            picture_type,
            description,
            data,
        } = source;
        let mime_type = mime_type.to_string();
        let picture_type = (*picture_type).into();
        let description = description.to_string();
        let size = data.len();
        Picture {
            mime_type,
            picture_type,
            description,
            size,
        }
    }

    pub fn with_id(self, id: u16) -> WithId {
        WithId { id, picture: self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct WithId {
    pub id: u16,
    #[serde(flatten)]
    pub picture: Picture,
}
