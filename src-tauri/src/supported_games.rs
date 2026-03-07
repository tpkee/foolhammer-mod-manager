use std::fmt;

/// A simple enumeration of games that are supported by the manager.
///
/// This type is used throughout the application as the canonical representation
/// of a game identifier. It serializes to the Steam App ID (e.g. `1142710`) and
/// implements conversions to/from numeric and string forms.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum SupportedGames {
    Warhammer3,
}

impl fmt::Display for SupportedGames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id: u32 = (*self).into();
        write!(f, "{}", id)
    }
}

impl From<SupportedGames> for String {
    fn from(value: SupportedGames) -> Self {
        value.to_string()
    }
}

impl From<SupportedGames> for u32 {
    fn from(value: SupportedGames) -> Self {
        match value {
            SupportedGames::Warhammer3 => 1142710,
        }
    }
}

impl TryFrom<u32> for SupportedGames {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1142710 => Ok(SupportedGames::Warhammer3),
            _ => Err(format!("Unsupported game id: {}", value)),
        }
    }
}

impl TryFrom<String> for SupportedGames {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .parse::<u32>()
            .map_err(|e| format!("Invalid game id '{}': {}", value, e))
            .and_then(SupportedGames::try_from)
    }
}