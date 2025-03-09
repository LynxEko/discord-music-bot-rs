use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use serenity::{all::GuildId, prelude::TypeMapKey};
use songbird::Call;
use tokio::sync::{Mutex, RwLock};

use super::{channel::play_song, internal::Playlist};

pub struct PlaylistKey;

impl TypeMapKey for PlaylistKey {
    type Value = Arc<RwLock<HashMap<GuildId, Playlist>>>;
}

pub async fn add_to_playlist(
    playlist_lock: Arc<RwLock<HashMap<GuildId, Playlist>>>,
    guild_id: GuildId,
    handler_lock: Arc<Mutex<Call>>,
    song: String,
) {
    add_song(playlist_lock, guild_id, handler_lock, song, false).await
}

pub async fn add_now(
    playlist_lock: Arc<RwLock<HashMap<GuildId, Playlist>>>,
    guild_id: GuildId,
    handler_lock: Arc<Mutex<Call>>,
    song: String,
) {
    add_song(playlist_lock, guild_id, handler_lock, song, true).await
}

async fn add_song(
    playlist_lock: Arc<RwLock<HashMap<GuildId, Playlist>>>,
    guild_id: GuildId,
    handler_lock: Arc<Mutex<Call>>,
    song: String,
    immediate: bool,
) {
    {
        let mut playlist_map = playlist_lock.write().await;

        let playlist = match playlist_map.entry(guild_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Playlist::default()),
        };

        if playlist.get_current_track().await.is_some() {
            if immediate {
                playlist.add_now(song);
            } else {
                playlist.add_to_playlist(song);
            }
        } else {
            if immediate {
                let track_handle = play_song(handler_lock, song).await;
                playlist.set_current_track(track_handle);
            } else {
                let track_handle = play_song(handler_lock, song.clone()).await;
                playlist.set_current_track(track_handle);
                playlist.add_to_playlist(song);
            }
        }
    }
}
