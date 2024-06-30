use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CommandNotInGuild,
    InvalidMessageId(String),
    InvalidEmoji(String),

    MemberNotFound(serenity::all::Reaction),
    GuildNotFound(serenity::all::Reaction),
    UserNotFound(serenity::all::Reaction),

    Serenity(serenity::Error),
    Sqlx(sqlx::Error),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> String {
        match self {
            Self::CommandNotInGuild => String::from("This command must be used in a guild."),
            Self::InvalidMessageId(id) => format!("Invalid message ID: {}", id),
            Self::InvalidEmoji(e) => format!("Invalid emoji: {}", e),
            _ => String::new(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(e: serenity::Error) -> Self {
        Self::Serenity(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}
