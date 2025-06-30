use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;

use crate::playlist::key::{get_playlist_lock, switch_shuffle};

pub fn register() -> CreateCommand {
    CreateCommand::new("shuffle").description("shuffle the queue")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let guild_id = interaction
        .guild_id
        .unwrap()
        .to_guild_cached(&ctx.cache)
        .unwrap()
        .id;

    let playlist_lock = get_playlist_lock(ctx).await;

    let return_value = switch_shuffle(playlist_lock, guild_id).await;
    match return_value {
        Ok(message) => message,
        Err(err) => err,
    }
}
