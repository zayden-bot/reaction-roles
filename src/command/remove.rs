use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, GuildId, MessageId,
    ReactionType, ResolvedValue,
};
use sqlx::Pool;
use std::collections::HashMap;

use crate::reaction_roles_manager::ReactionRolesManager;
use crate::Result;

pub(crate) async fn remove<Db, Row>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    channel_id: ChannelId,
    guild_id: GuildId,
    reaction: ReactionType,
    options: &HashMap<&str, &ResolvedValue<'_>>,
) -> Result<()>
where
    Db: sqlx::Database,
    Row: ReactionRolesManager<Db>,
{
    let message_id = match options.get("message_id") {
        Some(ResolvedValue::String(message_id)) => MessageId::new(message_id.parse()?),
        _ => unreachable!("Message ID is required"),
    };

    Row::delete_row(
        pool,
        guild_id,
        channel_id,
        message_id,
        &reaction.to_string(),
    )
    .await?;

    channel_id
        .delete_reaction_emoji(ctx, message_id, reaction)
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Reaction role removed."),
        )
        .await?;

    Ok(())
}