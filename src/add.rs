use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateEmbed, CreateMessage, EditInteractionResponse,
    GuildId, Mentionable, MessageId, ReactionType, ResolvedValue,
};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::reaction_role_row::ReactionRoleRow;
use crate::Result;

pub(crate) async fn add<Row: ReactionRoleRow>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &PgPool,
    guild_id: &GuildId,
    channel_id: &ChannelId,
    reaction: ReactionType,
    options: &HashMap<&str, &ResolvedValue<'_>>,
) -> Result<()> {
    let role = match options.get("role") {
        Some(ResolvedValue::Role(role)) => *role,
        _ => unreachable!("Role is required"),
    };

    let message_id = match options.get("message_id") {
        Some(ResolvedValue::String(message_id)) => Some(MessageId::new(message_id.parse()?)),
        _ => None,
    };

    let message = match message_id {
        Some(message_id) => channel_id.message(ctx, message_id).await?,
        None => {
            channel_id
                .send_message(
                    ctx,
                    CreateMessage::new().embed(CreateEmbed::new().description(format!(
                        "{} | {}",
                        reaction,
                        role.mention()
                    ))),
                )
                .await?
        }
    };

    Row::create(
        pool,
        *guild_id,
        *channel_id,
        message.id,
        role.id,
        &reaction.to_string(),
    )
    .await?;

    message.react(ctx, reaction).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Reaction role added."),
        )
        .await?;

    Ok(())
}