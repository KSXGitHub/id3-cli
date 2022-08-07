use clap::{Subcommand, ValueEnum};
use derive_more::{Display, From};
use id3::frame;
use serde::{Deserialize, Serialize};

/// Standard picture type.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Display, ValueEnum, Subcommand,
)]
#[serde(rename_all = "PascalCase")]
#[clap(rename_all = "PascalCase")]
pub enum PictureType {
    Other,
    Icon,
    OtherIcon,
    CoverFront,
    CoverBack,
    Leaflet,
    Media,
    LeadArtist,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    ScreenCapture,
    BrightFish,
    Illustration,
    BandLogo,
    PublisherLogo,
}

impl From<PictureType> for frame::PictureType {
    fn from(picture_type: PictureType) -> Self {
        match picture_type {
            PictureType::Other => frame::PictureType::Other,
            PictureType::Icon => frame::PictureType::Icon,
            PictureType::OtherIcon => frame::PictureType::OtherIcon,
            PictureType::CoverFront => frame::PictureType::CoverFront,
            PictureType::CoverBack => frame::PictureType::CoverBack,
            PictureType::Leaflet => frame::PictureType::Leaflet,
            PictureType::Media => frame::PictureType::Media,
            PictureType::LeadArtist => frame::PictureType::LeadArtist,
            PictureType::Artist => frame::PictureType::Artist,
            PictureType::Conductor => frame::PictureType::Conductor,
            PictureType::Band => frame::PictureType::Band,
            PictureType::Composer => frame::PictureType::Composer,
            PictureType::Lyricist => frame::PictureType::Lyricist,
            PictureType::RecordingLocation => frame::PictureType::RecordingLocation,
            PictureType::DuringRecording => frame::PictureType::DuringRecording,
            PictureType::DuringPerformance => frame::PictureType::DuringPerformance,
            PictureType::ScreenCapture => frame::PictureType::ScreenCapture,
            PictureType::BrightFish => frame::PictureType::BrightFish,
            PictureType::Illustration => frame::PictureType::Illustration,
            PictureType::BandLogo => frame::PictureType::BandLogo,
            PictureType::PublisherLogo => frame::PictureType::PublisherLogo,
        }
    }
}

impl TryFrom<frame::PictureType> for PictureType {
    type Error = u8;
    fn try_from(picture_type: frame::PictureType) -> Result<Self, Self::Error> {
        match picture_type {
            frame::PictureType::Other => Ok(PictureType::Other),
            frame::PictureType::Icon => Ok(PictureType::Icon),
            frame::PictureType::OtherIcon => Ok(PictureType::OtherIcon),
            frame::PictureType::CoverFront => Ok(PictureType::CoverFront),
            frame::PictureType::CoverBack => Ok(PictureType::CoverBack),
            frame::PictureType::Leaflet => Ok(PictureType::Leaflet),
            frame::PictureType::Media => Ok(PictureType::Media),
            frame::PictureType::LeadArtist => Ok(PictureType::LeadArtist),
            frame::PictureType::Artist => Ok(PictureType::Artist),
            frame::PictureType::Conductor => Ok(PictureType::Conductor),
            frame::PictureType::Band => Ok(PictureType::Band),
            frame::PictureType::Composer => Ok(PictureType::Composer),
            frame::PictureType::Lyricist => Ok(PictureType::Lyricist),
            frame::PictureType::RecordingLocation => Ok(PictureType::RecordingLocation),
            frame::PictureType::DuringRecording => Ok(PictureType::DuringRecording),
            frame::PictureType::DuringPerformance => Ok(PictureType::DuringPerformance),
            frame::PictureType::ScreenCapture => Ok(PictureType::ScreenCapture),
            frame::PictureType::BrightFish => Ok(PictureType::BrightFish),
            frame::PictureType::Illustration => Ok(PictureType::Illustration),
            frame::PictureType::BandLogo => Ok(PictureType::BandLogo),
            frame::PictureType::PublisherLogo => Ok(PictureType::PublisherLogo),
            frame::PictureType::Undefined(value) => Err(value),
        }
    }
}

/// Standard or non-Standard picture type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, Display, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PictureTypeExtra {
    Defined(PictureType),
    Undefined(u8),
}

impl From<frame::PictureType> for PictureTypeExtra {
    fn from(picture_type: frame::PictureType) -> Self {
        match picture_type.try_into() {
            Ok(picture_type) => PictureTypeExtra::Defined(picture_type),
            Err(picture_type) => PictureTypeExtra::Undefined(picture_type),
        }
    }
}
