use serenity::async_trait;
use sqlx::any::AnyQueryResult;
use sqlx::{FromRow, Pool};

#[async_trait]
pub trait ReactionRoleSchema<Db: sqlx::Database> {
    async fn create_row(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        role_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;

    async fn get_guild_rows(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
    ) -> sqlx::Result<Vec<ReactionRole>>;

    async fn delete_row(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;
}

#[derive(FromRow)]
pub struct ReactionRole {
    pub id: i32,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub role_id: i64,
    pub emoji: String,
}
