use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use songbird::events::TrackEvent;

use crate::voice_handler::TrackErrorNotifier;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let (guild_id, channel_id) = {
        let guild = interaction
            .guild_id
            .unwrap()
            .to_guild_cached(&ctx.cache)
            .unwrap();
        let channel_id = guild
            .voice_states
            .get(&interaction.user.id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => return "Not in a voice channel".to_string(),
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    }

    "Joined the channel".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join your current voice channel")
}
