use rand::seq::SliceRandom;
use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;

pub fn register() -> CreateCommand {
    CreateCommand::new("shuffle").description("shuffle the queue")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    return "NOT IMPLEMENTED".into();

    let guild_id = interaction.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        // handler.queue().modify_queue(|queued_songs| {
        //     let mut to_shuffle = queued_songs.split_off(1);
        //     let mut rng = rand::thread_rng();
        //     to_shuffle.make_contiguous().shuffle(&mut rng);
        //     for song in to_shuffle {
        //         queued_songs.push_back(song);
        //     }
        // });

        "Shuffled the queue".into()
    } else {
        "Error shuffling the queue".into()
    }
}
