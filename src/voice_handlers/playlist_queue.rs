use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use serenity::{all::GuildId, async_trait};
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler},
    Songbird,
};
use tokio::sync::RwLock;
// use tokio::sync::Mutex;
use tracing::{error, info};

use crate::playlist::{
    channel::{leave, play_song},
    internal::Playlist,
};

#[derive(Clone)]
pub struct PlaylistQueueHandler {
    pub manager: Arc<Songbird>,
    // pub handler_lock: Arc<Mutex<Call>>,
    pub guild_id: GuildId,
    pub playlist_lock: Arc<RwLock<HashMap<GuildId, Playlist>>>,
}

#[async_trait]
impl VoiceEventHandler for PlaylistQueueHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(track_list) => {
                for (state, handle) in *track_list {
                    info!("Track {:?} ended: {:?}", handle.uuid(), state.playing);
                    self.next_song().await
                }
            }
            _ => {}
        }
        None
    }
}

impl PlaylistQueueHandler {
    async fn next_song(&self) {
        let Some(handler_lock) = self.manager.get(self.guild_id) else {
            error!("Not in a call (?? this should not happen)");
            return;
        };

        let mut playlist_map = self.playlist_lock.write().await;

        let playlist = match playlist_map.entry(self.guild_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Playlist::default()),
        };

        let Some(song) = playlist.next_song() else {
            info!("No more songs to play, exiting");
            leave(self.manager.clone(), self.guild_id).await;
            return;
        };

        // play the song
        let handle = play_song(handler_lock, song).await;

        playlist.set_current_track(handle);
    }
}
