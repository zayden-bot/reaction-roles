use serenity::all::ReactionConversionError;
use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingGuildId,
    InvalidMessageId(String),
    ReactionConversionError(ReactionConversionError),
}

impl Error {
    pub fn invalid_message_id(id: &str) -> Self {
        Self::InvalidMessageId(format!("Invalid message ID: {}", id))
    }
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Self::MissingGuildId => zayden_core::Error::MissingGuildId.to_response(),
            Self::InvalidMessageId(msg) => msg,
            Self::ReactionConversionError(_) => "Failed to convert emoji to reaction",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<ReactionConversionError> for Error {
    fn from(err: ReactionConversionError) -> Self {
        Self::ReactionConversionError(err)
    }
}
