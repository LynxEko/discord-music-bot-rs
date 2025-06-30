use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use serenity::client::Context;
use serenity::{all::GuildId, prelude::TypeMapKey};
use songbird::Call;
use tokio::sync::{Mutex, RwLock};

use crate::playlist::internal::PlayMode;

use super::{channel::play_song, internal::Playlist};

pub struct PlaylistKey;
pub type PlaylistValue = Arc<RwLock<HashMap<GuildId, Playlist>>>;

impl TypeMapKey for PlaylistKey {
    type Value = PlaylistValue;
}

pub async fn get_playlist_lock(ctx: &Context) -> PlaylistValue {
    let data_read = ctx.data.read().await;
    data_read
        .get::<PlaylistKey>()
        .expect("Expected Playlist in TypeMap")
        .clone()
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

pub async fn switch_shuffle(
    playlist_lock: PlaylistValue,
    guild_id: GuildId,
) -> Result<String, String> {
    let mut playlist_map = playlist_lock.write().await;

    let playlist = match playlist_map.entry(guild_id) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(_) => return Err("Missing playlist".into()),
    };

    let playstate = playlist.switch_playstate();
    let playstate_string = match playstate {
        PlayMode::Playing => "Playing in order",
        PlayMode::Shuffle => "Playing random shuffle",
    };
    Ok(playstate_string.to_string())
}
