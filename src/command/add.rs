use serenity::all::{
    ChannelId, Context, CreateEmbed, CreateMessage, GuildId, Mentionable, MessageId, ReactionType,
    ResolvedValue,
};
use sqlx::{Database, Pool};
use std::collections::HashMap;

use crate::reaction_roles_manager::ReactionRolesManager;
use crate::{Error, Result};

use super::ReactionRoleCommand;

impl ReactionRoleCommand {
    pub(super) async fn add<Db: Database, Manager: ReactionRolesManager<Db>>(
        ctx: &Context,
        pool: &Pool<Db>,
        guild_id: GuildId,
        channel_id: ChannelId,
        reaction: ReactionType,
        mut options: HashMap<&str, ResolvedValue<'_>>,
    ) -> Result<()> {
        let Some(ResolvedValue::Role(role)) = options.remove("role") else {
            unreachable!("Role is required");
        };

        let message_id = match options.remove("message_id") {
            Some(ResolvedValue::String(id)) => Some(MessageId::new(
                id.parse()
                    .map_err(|_| Error::InvalidMessageId(id.to_string()))?,
            )),
            _ => None,
        };

        let message = match message_id {
            Some(message_id) => channel_id.message(ctx, message_id).await.unwrap(),
            None => channel_id
                .send_message(
                    ctx,
                    CreateMessage::new().embed(CreateEmbed::new().description(format!(
                        "{} | {}",
                        reaction,
                        role.mention()
                    ))),
                )
                .await
                .unwrap(),
        };

        Manager::create_row(
            pool,
            guild_id,
            channel_id,
            message.id,
            role.id,
            &reaction.to_string(),
        )
        .await
        .unwrap();

        message.react(ctx, reaction).await.unwrap();

        Ok(())
    }
}
