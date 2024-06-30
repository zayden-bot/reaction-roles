use serenity::all::{Context, Reaction, RoleId};
use sqlx::Pool;

use crate::{Error, ReactionRolesManager, Result};

pub struct ReactionRoleReaction;

impl ReactionRoleReaction {
    pub async fn reaction_add<Db, Manager>(
        ctx: &Context,
        reaction: &Reaction,
        pool: &Pool<Db>,
    ) -> Result<()>
    where
        Db: sqlx::Database,
        Manager: ReactionRolesManager<Db>,
    {
        let emoji_string = reaction.emoji.to_string();
        let reaction_role = Manager::get_row(pool, reaction.message_id, &emoji_string).await?;

        if let Some(reaction_role) = reaction_role {
            let member = reaction
                .member
                .as_ref()
                .ok_or_else(|| Error::MemberNotFound(reaction.clone()))?;

            member.add_role(ctx, reaction_role.role_id as u64).await?;
        }

        Ok(())
    }

    pub async fn reaction_remove<Db, Manager>(
        ctx: &Context,
        reaction: &Reaction,
        pool: &Pool<Db>,
    ) -> Result<()>
    where
        Db: sqlx::Database,
        Manager: ReactionRolesManager<Db>,
    {
        let guild_id = reaction
            .guild_id
            .ok_or_else(|| Error::GuildNotFound(reaction.clone()))?;
        let user_id = reaction
            .user_id
            .ok_or_else(|| Error::UserNotFound(reaction.clone()))?;

        let reaction_role =
            Manager::get_row(pool, reaction.message_id, &reaction.emoji.to_string()).await?;

        if let Some(reaction_role) = reaction_role {
            let role_id = RoleId::new(reaction_role.role_id as u64);

            ctx.http
                .remove_member_role(guild_id, user_id, role_id, Some("Reaction role removed"))
                .await?;
        }

        Ok(())
    }
}
