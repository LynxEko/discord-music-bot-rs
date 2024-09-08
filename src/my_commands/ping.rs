use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use tracing::warn;

pub fn run(_options: &[ResolvedOption]) -> String {
    warn!("Ping received, sending pong...");

    "pong".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
