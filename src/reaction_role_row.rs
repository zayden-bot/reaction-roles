use serenity::async_trait;
use sqlx::any::AnyQueryResult;
use sqlx::Pool;

#[async_trait]
pub trait ReactionRoleRow<Db: sqlx::Database> {
    async fn create(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        role_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;

    async fn delete(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;
}
