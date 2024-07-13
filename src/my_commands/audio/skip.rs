use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;

pub fn register() -> CreateCommand {
    CreateCommand::new("skip").description("Skip the current song")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        if let Err(e) = handler.queue().skip() {
            format!("Failed to skip a song: {:?}", e)
        } else {
            format!("Skipped a song")
        }
    } else {
        format!("Not in a voice channel")
    }
}
