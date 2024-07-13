use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let guild_id = interaction.guild_id.unwrap();
    
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();
    
    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            format!("Failed: {:?}", e)
        } else {
        	format!("Left voice channel")            
        }
    } else {
        format!("Not in a voice channel")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave").description("Leave the current voice channel")
}

