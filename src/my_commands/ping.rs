use serenity::{async_trait, builder::CreateCommand};
use tracing::trace;

use crate::my_commands::base_command::{BaseCommand, CommandResponse};

pub struct Ping;

#[async_trait]
impl BaseCommand for Ping {
    fn command_name(&self) -> &'static str {
        "ping"
    }

    fn generate_create_command(&self) -> CreateCommand {
        CreateCommand::new(self.command_name()).description("A ping command")
    }

    async fn run(
        &self,
        _command: &serenity::all::CommandInteraction,
        _ctx: &serenity::prelude::Context,
    ) -> CommandResponse {
        trace!("Ping received, sending pong...");

        CommandResponse::String("pong".into())
    }
}
