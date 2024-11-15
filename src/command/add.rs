use serenity::all::{
    ChannelId, CommandOptionType, Context, CreateCommandOption, CreateEmbed, CreateMessage,
    GuildId, Mentionable, MessageId, ReactionType, ResolvedValue,
};
use sqlx::Pool;
use std::collections::HashMap;

use crate::reaction_roles_manager::ReactionRolesManager;
use crate::{Error, Result};

pub(super) async fn run<Db, Row>(
    ctx: &Context,
    pool: &Pool<Db>,
    guild_id: GuildId,
    channel_id: ChannelId,
    reaction: ReactionType,
    options: &HashMap<&str, &ResolvedValue<'_>>,
) -> Result<()>
where
    Db: sqlx::Database,
    Row: ReactionRolesManager<Db>,
{
    let role = match options.get("role") {
        Some(ResolvedValue::Role(role)) => *role,
        _ => unreachable!("Role is required"),
    };

    let message_id = match options.get("message_id") {
        Some(ResolvedValue::String(id)) => {
            let id = id
                .parse()
                .map_err(|_| Error::InvalidMessageId(id.to_string()))?;
            Some(MessageId::new(id))
        }
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

    Row::create_row(
        pool,
        guild_id,
        channel_id,
        message.id,
        role.id,
        &reaction.to_string(),
    )
    .await?;

    message.react(ctx, reaction).await?;

    Ok(())
}

pub(super) fn register() -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, "add", "Adds a reaction role")
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "emoji",
                "The emoji of the reaction role",
            )
            .required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::Role,
                "role",
                "The role to add when the emoji is reacted to",
            )
            .required(true),
        )
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::Channel,
            "channel",
            "The channel the message is in",
        ))
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::String,
            "message_id",
            "The message id of the reaction role message",
        ))
}
