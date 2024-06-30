use serenity::all::{
    ChannelId, CommandOptionType, Context, CreateCommandOption, GuildId, MessageId, ReactionType,
    ResolvedValue,
};
use sqlx::Pool;
use std::collections::HashMap;

use crate::reaction_roles_manager::ReactionRolesManager;
use crate::Result;

pub(super) async fn run<Db, Row>(
    ctx: &Context,
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

    Ok(())
}

pub(super) fn register() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "remove",
        "Removes a reaction role",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "message_id",
            "The message id of the reaction role message",
        )
        .required(true),
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "emoji",
            "The emoji of the reaction role",
        )
        .required(true),
    )
    .add_sub_option(CreateCommandOption::new(
        CommandOptionType::Channel,
        "channel",
        "The channel the message is in",
    ))
}
