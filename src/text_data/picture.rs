use id3::frame::Picture as Id3Picture;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Picture {
    pub mime_type: String,
    pub picture_type: String,
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
        let picture_type = picture_type.to_string();
        let description = description.to_string();
        let size = data.len();
        Picture {
            mime_type,
            picture_type,
            description,
            size,
        }
    }
}
