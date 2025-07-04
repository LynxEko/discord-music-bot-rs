use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::{all::CommandInteraction, async_trait};

use crate::my_commands::base_command::{BaseCommand, CommandResponse};

pub struct Leave;

#[async_trait]
impl BaseCommand for Leave {
    fn command_name(&self) -> &'static str {
        "leave"
    }

    fn generate_create_command(&self) -> CreateCommand {
        CreateCommand::new(self.command_name()).description("Leave the current voice channel")
    }

    async fn run(
        &self,
        command: &CommandInteraction,
        ctx: &Context,
    ) -> crate::my_commands::base_command::CommandResponse {
        let guild_id = command.guild_id.unwrap();

        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();
        let has_handler = manager.get(guild_id).is_some();

        let result = if has_handler {
            if let Err(e) = manager.remove(guild_id).await {
                format!("Failed: {:?}", e)
            } else {
                format!("Left voice channel")
            }
        } else {
            format!("Not in a voice channel")
        };

        CommandResponse::String(result)
    }
}
