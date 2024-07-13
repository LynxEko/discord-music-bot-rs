use rand::prelude::*;
use rand::seq::SliceRandom;
use std::path::Path;

use rusty_ytdl::Video;
use serenity::all::{Context, GuildId};

pub struct PlaylistManager {
    song_names: Vec<String>,
}

impl PlaylistManager {
    pub async fn add_song(ctx: &Context, guild_id: GuildId, video_id: &str) {
        let file_path = format!("music/{video_id}.mp3");

        let path = Path::new(&file_path);

        if path.exists() {
            let manager = songbird::get(ctx)
                .await
                .expect("Songbird Voice client placed in at initialisation.")
                .clone();

            if let Some(handler_lock) = manager.get(guild_id) {
                let mut handler = handler_lock.lock().await;

                let file_src = songbird::input::File::new(format!("music/{video_id}.mp3"));

                let handle = handler.enqueue_input(file_src.into()).await;
                handler.queue().modify_queue(|queued_songs| {
                    let mut rng = rand::thread_rng();
                    queued_songs.make_contiguous().shuffle(&mut rng);
                });
            } else {
            }
        }
    }

    async fn download_song(video_id: &str) {
        let video = Video::new(video_id).unwrap();

        video
            .download(format!("music/{}.mp3", video_id))
            .await
            .unwrap();
    }
}
