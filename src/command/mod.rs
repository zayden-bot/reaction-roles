use serenity::all::{CommandInteraction, Context, CreateCommand, ReactionType, ResolvedValue};
use sqlx::Pool;

mod add;
mod remove;

pub use crate::error::{Error, Result};
pub use crate::reaction_roles_manager::ReactionRolesManager;
use crate::utils::parse_options;

pub struct ReactionRoleCommand;

impl ReactionRoleCommand {
    pub async fn run<Db, Row>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<()>
    where
        Db: sqlx::Database,
        Row: ReactionRolesManager<Db>,
    {
        let guild_id = interaction
            .guild_id
            .ok_or_else(|| Error::CommandNotInGuild)?;

        let command = &interaction.data.options()[0];

        let options = match &command.value {
            ResolvedValue::SubCommand(options) => options,
            _ => unreachable!("Subcommand is required"),
        };
        let options = parse_options(options);

        let channel_id = match options.get("channel") {
            Some(ResolvedValue::Channel(channel)) => channel.id,
            _ => interaction.channel_id,
        };

        let reaction = match options.get("emoji") {
            Some(ResolvedValue::String(emoji)) => ReactionType::try_from(*emoji)
                .map_err(|_| Error::InvalidEmoji(emoji.to_string()))?,
            _ => unreachable!("Emoji is required"),
        };

        match command.name {
            "add" => {
                add::run::<Db, Row>(ctx, pool, guild_id, channel_id, reaction, &options).await?
            }
            "remove" => {
                remove::run::<Db, Row>(ctx, pool, channel_id, guild_id, reaction, &options).await?;
            }
            _ => unreachable!("Invalid subcommand name"),
        };

        Ok(())
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("reaction_role")
            .description("Adds or removes a reaction role")
            .add_option(add::register())
            .add_option(remove::register())
    }
}
