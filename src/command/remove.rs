use serenity::all::{ChannelId, Context, GuildId, MessageId, ReactionType, ResolvedValue};
use sqlx::{Database, Pool};
use std::collections::HashMap;

use crate::reaction_roles_manager::ReactionRolesManager;
use crate::{Error, Result};

use super::ReactionRoleCommand;

impl ReactionRoleCommand {
    pub(super) async fn remove<Db: Database, Manager: ReactionRolesManager<Db>>(
        ctx: &Context,
        pool: &Pool<Db>,
        channel_id: ChannelId,
        guild_id: GuildId,
        reaction: ReactionType,
        mut options: HashMap<&str, ResolvedValue<'_>>,
    ) -> Result<()> {
        let Some(ResolvedValue::String(id)) = options.remove("message_id") else {
            unreachable!("Message ID is required")
        };
        let message_id = MessageId::new(
            id.parse()
                .map_err(|_| Error::InvalidMessageId(id.to_string()))?,
        );

        Manager::delete_row(
            pool,
            guild_id,
            channel_id,
            message_id,
            &reaction.to_string(),
        )
        .await
        .unwrap();

        channel_id
            .delete_reaction_emoji(ctx, message_id, reaction)
            .await
            .unwrap();

        Ok(())
    }
}
