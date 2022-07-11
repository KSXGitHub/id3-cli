use clap::ValueEnum;
use serde::Serialize;

/// Format of input/output text values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(about = "")]
pub enum TextFormat {
    Json,
    Yaml,
    Toml,
}

impl TextFormat {
    /// Convert a [serializable](Serialize) value to a string according to the specified [format](TextFormat).
    pub fn serialize(self, value: &impl Serialize) -> Result<String, String> {
        match self {
            TextFormat::Json => serde_json::to_string_pretty(value).map_err(|e| e.to_string()),
            TextFormat::Yaml => serde_yaml::to_string(value).map_err(|e| e.to_string()),
            TextFormat::Toml => toml::to_string_pretty(value).map_err(|e| e.to_string()),
        }
    }
}
