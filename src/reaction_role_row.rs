use serenity::async_trait;
use sqlx::any::AnyQueryResult;
use sqlx::Pool;

#[async_trait]
pub trait ReactionRoleRow<D: sqlx::Database> {
    async fn create(
        pool: &Pool<D>,
        guild_id: impl Into<i64>,
        channel_id: impl Into<i64>,
        message_id: impl Into<i64>,
        role_id: impl Into<i64>,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;

    async fn delete(
        pool: &Pool<D>,
        guild_id: impl Into<i64>,
        channel_id: impl Into<i64>,
        message_id: impl Into<i64>,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;
}
