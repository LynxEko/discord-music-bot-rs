use serenity::{all::CreateCommand, async_trait};

pub enum CommandResponse {
    #[allow(dead_code)]
    InternallyHandled,
    String(String),
}

#[async_trait]
pub trait BaseCommand {
    fn command_name(&self) -> &'static str;

    fn generate_create_command(&self) -> CreateCommand;

    async fn run(
        &self,
        _command: &serenity::all::CommandInteraction,
        _ctx: &serenity::prelude::Context,
    ) -> CommandResponse;
}
