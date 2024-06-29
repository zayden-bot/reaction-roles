use serenity::async_trait;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

#[async_trait]
pub trait ReactionRoleRow {
    async fn create(
        pool: &PgPool,
        guild_id: impl TryInto<i64>,
        channel_id: impl TryInto<i64>,
        message_id: impl TryInto<i64>,
        role_id: impl TryInto<i64>,
        emoji: &str,
    ) -> sqlx::Result<PgQueryResult>;

    async fn delete(
        pool: &PgPool,
        guild_id: impl TryInto<i64>,
        channel_id: impl TryInto<i64>,
        message_id: impl TryInto<i64>,
        emoji: &str,
    ) -> sqlx::Result<PgQueryResult>;
}
