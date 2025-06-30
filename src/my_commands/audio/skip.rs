use std::collections::hash_map::Entry;

use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;

use crate::playlist::internal::Playlist;
use crate::playlist::key::PlaylistKey;

pub fn register() -> CreateCommand {
    CreateCommand::new("skip").description("Skip the current song")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(_) = manager.get(guild_id) {
        let playlist_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<PlaylistKey>()
                .expect("Expected Playlist in TypeMap")
                .clone()
        };

        let mut playlist_map = playlist_lock.write().await;

        let playlist = match playlist_map.entry(guild_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Playlist::default()),
        };

        playlist
            .get_current_track()
            .await
            .and_then(|track| match track.stop() {
                Ok(_) => Some("Skipped a song".into()),
                Err(err) => Some(format!("Error skipping track: {err}")),
            })
            .unwrap_or("Error getting the track".into())
    } else {
        format!("Not in a voice channel")
    }
}
