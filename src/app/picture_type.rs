use clap::{Subcommand, ValueEnum};
use id3::frame;

/// Standard picture type.
#[derive(Debug, Clone, Copy, ValueEnum, Subcommand)]
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
