use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::{all::CommandInteraction, async_trait};

use crate::{
    my_commands::base_command::{BaseCommand, CommandResponse},
    playlist::key::{get_playlist_lock, switch_shuffle},
};

pub struct Shuffle;

#[async_trait]
impl BaseCommand for Shuffle {
    fn command_name(&self) -> &'static str {
        "shuffle"
    }

    fn generate_create_command(&self) -> CreateCommand {
        CreateCommand::new(self.command_name()).description("shuffle the queue")
    }

    async fn run(&self, command: &CommandInteraction, ctx: &Context) -> CommandResponse {
        let guild_id = command
            .guild_id
            .unwrap()
            .to_guild_cached(&ctx.cache)
            .unwrap()
            .id;

        let playlist_lock = get_playlist_lock(ctx).await;

        let return_value = switch_shuffle(playlist_lock, guild_id).await;
        let response = match return_value {
            Ok(message) => message,
            Err(err) => err,
        };
        CommandResponse::String(response)
    }
}
