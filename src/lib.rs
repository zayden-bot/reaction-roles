use reaction_role_row::ReactionRoleRow;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, ReactionType, ResolvedValue,
};
use slash_command_core::parse_options;
use sqlx::PgPool;

mod add;
mod error;
mod reaction_role_row;
mod remove;

use add::add;
pub(crate) use error::{Error, Result};
use remove::remove;

struct ReactionRoles;

impl ReactionRoles {
    async fn run<Row: ReactionRoleRow>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        let _ = interaction.defer(ctx).await;

        let guild_id = interaction
            .guild_id
            .ok_or_else(|| Error::CommandNotInGuild)?;

        let command = &interaction.data.options()[0];

        let options = match &command.value {
            ResolvedValue::SubCommand(options) => options,
            _ => unreachable!("Subcommand is required"),
        };
        let options = parse_options(options);

        let channel = match options.get("channel") {
            Some(ResolvedValue::Channel(channel)) => *channel,
            _ => unreachable!("Channel is required"),
        };

        let reaction = match options.get("emoji") {
            Some(ResolvedValue::String(emoji)) => ReactionType::try_from(*emoji)?,
            _ => unreachable!("Emoji is required"),
        };

        match command.name {
            "add" => {
                add::<Row>(
                    ctx,
                    interaction,
                    pool,
                    &guild_id,
                    &channel.id,
                    reaction,
                    &options,
                )
                .await?
            }
            "remove" => {
                remove::<Row>(
                    ctx,
                    interaction,
                    pool,
                    &channel.id,
                    &guild_id,
                    reaction,
                    &options,
                )
                .await?;
            }
            _ => unreachable!("Invalid subcommand name"),
        };

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("reaction_role")
            .description("Adds or removes a reaction role")
            .default_member_permissions(Permissions::MANAGE_MESSAGES)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "add",
                    "Adds a reaction role",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Channel,
                        "channel",
                        "The channel the message is in",
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
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Role,
                        "role",
                        "The role to add when the emoji is reacted to",
                    )
                    .required(true),
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::String,
                    "message_id",
                    "The message id of the reaction role message",
                )),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "remove",
                    "Removes a reaction role",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Channel,
                        "channel",
                        "The channel the message is in",
                    )
                    .required(true),
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
                ),
            )
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }