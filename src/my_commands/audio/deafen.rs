
use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let guild_id = interaction.guild_id.unwrap();
    
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            return "Not in a voice call".to_string()
        },
    };
    
    let mut handler = handler_lock.lock().await;
    
    if handler.is_deaf() {
        "Already deafened".to_string()
    } else {
        if let Err(e) = handler.deafen(true).await {
            format!("Failed: {:?}", e)
        }
		else {
		    "Deafened".to_string()
		}
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("deafen").description("Deafen the bot")
}