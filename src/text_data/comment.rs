use id3::{frame, Frame};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};

/// Representation of the comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Comment<Language, Description, Content> {
    /// Language code (ISO 639-2) of the comment.
    pub language: Language,
    /// Description of the comment.
    pub description: Description,
    /// Content of the comment.
    pub content: Content,
}

impl<Language, Description, Content> Comment<Language, Description, Content> {
    /// Convert all fields to string slices.
    pub fn as_str_slices(&self) -> Comment<&'_ str, &'_ str, &'_ str>
    where
        Language: AsRef<str>,
        Description: AsRef<str>,
        Content: AsRef<str>,
    {
        Comment {
            language: self.language.as_ref(),
            description: self.description.as_ref(),
            content: self.content.as_ref(),
        }
    }

    /// Convert all fields to owned strings.
    pub fn to_owned_strings(&self) -> Comment<String, String, String>
    where
        Language: ToString,
        Description: ToString,
        Content: ToString,
    {
        Comment {
            language: self.language.to_string(),
            description: self.description.to_string(),
            content: self.content.to_string(),
        }
    }
}

impl From<frame::Comment> for Comment<String, String, String> {
    fn from(comment: frame::Comment) -> Self {
        Comment {
            language: comment.lang,
            description: comment.description,
            content: comment.text,
        }
    }
}

impl<'a> From<&'a frame::Comment> for Comment<&'a str, &'a str, &'a str> {
    fn from(comment: &'a frame::Comment) -> Self {
        Comment {
            language: &comment.lang,
            description: &comment.description,
            content: &comment.text,
        }
    }
}

impl<Language, Description, Content> From<Comment<Language, Description, Content>>
    for frame::Comment
where
    Language: Into<String>,
    Description: Into<String>,
    Content: Into<String>,
{
    fn from(comment: Comment<Language, Description, Content>) -> Self {
        frame::Comment {
            lang: comment.language.into(),
            description: comment.description.into(),
            text: comment.content.into(),
        }
    }
}

impl<Language, Description, Content> From<Comment<Language, Description, Content>> for Frame
where
    Language: Into<String>,
    Description: Into<String>,
    Content: Into<String>,
{
    fn from(comment: Comment<Language, Description, Content>) -> Self {
        comment.pipe(frame::Comment::from).into()
    }
}
