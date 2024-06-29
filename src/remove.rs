use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, GuildId, MessageId,
    ReactionType, ResolvedValue,
};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::reaction_role_row::ReactionRoleRow;
use crate::Result;

pub(crate) async fn remove<Row: ReactionRoleRow>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &PgPool,
    channel_id: &ChannelId,
    guild_id: &GuildId,
    reaction: ReactionType,
    options: &HashMap<&str, &ResolvedValue<'_>>,
) -> Result<()> {
    let message_id = match options.get("message_id") {
        Some(ResolvedValue::String(message_id)) => MessageId::new(message_id.parse()?),
        _ => unreachable!("Message ID is required"),
    };

    Row::delete(
        pool,
        guild_id.get(),
        channel_id.get(),
        message_id.get(),
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
