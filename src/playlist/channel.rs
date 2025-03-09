use std::sync::Arc;

use serenity::all::{ChannelId, Context, GuildId};
use songbird::{tracks::TrackHandle, Call, Songbird, TrackEvent};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::{
    voice_handlers::{
        auto_disconnection::AutoDisconnectionHandler, playlist_queue::PlaylistQueueHandler,
        track_error::TrackErrorNotifier,
    },
    youtube::download::get_video_youtube,
};

use super::key::PlaylistKey;

pub async fn join_channel(
    ctx: &Context,
    manager: Arc<Songbird>,
    guild_id: GuildId,
    channel_id: ChannelId,
) -> Option<Arc<Mutex<Call>>> {
    let mut manager_get = manager.get(guild_id);
    if manager_get.is_none() {
        let _ = manager.join(guild_id, channel_id).await;
        manager_get = manager.get(guild_id);
        if let Some(handler_lock) = manager_get.clone() {
            let mut handler = handler_lock.lock().await;
            if !handler.is_deaf() {
                handler.deafen(true).await.unwrap();
            }
            handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);

            let track_handler = AutoDisconnectionHandler {
                manager: manager.clone(),
                // handler_lock: handler_lock.clone(),
                guild_id,
                cache: ctx.cache.clone(),
            };
            handler.add_global_event(
                songbird::CoreEvent::ClientDisconnect.into(),
                track_handler.clone(),
            );

            let playlist_lock = {
                let data_read = ctx.data.read().await;
                data_read
                    .get::<PlaylistKey>()
                    .expect("Expected Playlist in TypeMap")
                    .clone()
            };

            handler.add_global_event(
                TrackEvent::End.into(),
                PlaylistQueueHandler {
                    manager: manager.clone(),
                    guild_id,
                    playlist_lock,
                },
            );
        }
    }

    manager_get
}

pub async fn play_song(handler_lock: Arc<Mutex<Call>>, video_id: String) -> TrackHandle {
    info!("playing song with id: {video_id}");
    let src = get_video_youtube(video_id).await;

    let mut handler = handler_lock.lock().await;

    // let _ = handler.enqueue_input(src.into()).await;
    handler.play_only_input(src.into())
}

pub async fn leave(manager: Arc<Songbird>, guild_id: GuildId) {
    if let Err(e) = manager.remove(guild_id).await {
        // its awaiting for ever (manager seems to be wrong)
        error!("Failed: {:?}", e);
    } else {
        info!("Left voice channel");
    }
}
