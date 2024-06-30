pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CommandNotInGuild,

    InvalidGuildId,
    InvalidChannelId,
    InvalidMessageId,
    InvalidRoleId,

    MemberNotFound(serenity::all::Reaction),
    GuildNotFound(serenity::all::Reaction),
    UserNotFound(serenity::all::Reaction),

    ParseInt(std::num::ParseIntError),
    Serenity(serenity::Error),
    ReactionConversion(serenity::all::ReactionConversionError),
    Sqlx(sqlx::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<serenity::Error> for Error {
    fn from(e: serenity::Error) -> Self {
        Self::Serenity(e)
    }
}

impl From<serenity::all::ReactionConversionError> for Error {
    fn from(e: serenity::all::ReactionConversionError) -> Self {
        Self::ReactionConversion(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}
