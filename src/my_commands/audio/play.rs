use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use google_youtube3::api::PlaylistItem;
use serenity::all::{
    Cache, ChannelId, CommandInteraction, CommandOptionType, GuildId, ResolvedOption, ResolvedValue,
};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use songbird::{Call, Songbird, TrackEvent};
use tokio::sync::Mutex;
use tracing::error;

use crate::voice_handler::{TrackErrorNotifier, TrackHandler};
use crate::youtube;

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("Play a track")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "url",
                "The link where the video, audio or playlist is located",
            )
            .required(true),
        )
}

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &CommandInteraction,
) -> String {
    let Some(ResolvedOption {
        value: ResolvedValue::String(url_value),
        ..
    }) = options.get(0)
    else {
        return "Must provide a URL to a video or audio".into();
    };

    let Ok(url_value) = url::Url::parse(url_value) else {
        return "Must provide a valid URL to a video, audio or playlist".into();
    };

    let query_pairs = url_value.query_pairs();
    let mut video_id = None;
    let mut playlist_id = None;

    for (query_id, query_value) in query_pairs {
        match query_id.into_owned().as_str() {
            "v" => video_id = Some(query_value.into_owned()),
            "list" => playlist_id = Some(query_value.into_owned()),
            _ => {}
        }
    }

    let (guild_id, channel_id) = {
        let guild = interaction
            .guild_id
            .unwrap()
            .to_guild_cached(&ctx.cache)
            .unwrap();
        let channel_id = guild
            .voice_states
            .get(&interaction.user.id)
            .and_then(|voice_state| voice_state.channel_id)
            .unwrap();

        (guild.id, channel_id)
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(playlist_id) = playlist_id {
        // https://github.com/serenity-rs/serenity/blob/current/examples/e13_parallel_loops/src/main.rs
        if let Ok(playlist) = youtube::playlist::get_playlist(&playlist_id).await {
            let first_snippet = playlist.first().unwrap().snippet.clone().unwrap();
            let playlist_length = playlist.len();
            {
                let cache = ctx.cache.clone();
                let manager = manager.clone();
                let guild_id = guild_id.clone();
                let channel_id = channel_id.clone();
                tokio::spawn(async move {
                    let mut playlist = playlist;
                    playlist.remove(0);
                    tokio::time::sleep(Duration::from_secs(2)).await;

                    queue_playlist(cache.clone(), playlist, manager, guild_id, channel_id).await;
                });
            }
            let video_id = first_snippet.resource_id.unwrap().video_id.unwrap();
            let manager_get = join_channel(ctx.cache.clone(), manager, guild_id, channel_id).await;

            match join_and_play(manager_get, video_id).await {
                Ok(_) => format!("Playing {} songs from playist", playlist_length),
                Err(_) => "Could not join channel".to_string(),
            }
        } else {
            "Could not load the playlist".to_string()
        }
    } else if let Some(video_id) = video_id {
        let manager_get = join_channel(ctx.cache.clone(), manager, guild_id, channel_id).await;

        match join_and_play(manager_get, video_id).await {
            Ok(_) => "Playing song".to_string(),
            Err(_) => "Could not join channel".to_string(),
        }
    } else {
        "Other sources not implemented".to_string()
    }
}

async fn join_channel(
    cache: Arc<Cache>,
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
            let track_handler = TrackHandler {
                manager,
                // handler_lock: handler_lock.clone(),
                guild_id,
                cache,
            };
            handler.add_global_event(TrackEvent::End.into(), track_handler.clone());
            handler.add_global_event(
                songbird::CoreEvent::ClientDisconnect.into(),
                track_handler.clone(),
            );
        }
    }

    manager_get
}

async fn get_video_youtube(video_id: String) -> songbird::input::File<impl AsRef<Path> + Clone> {
    let file_path = format!("music/youtube/{video_id}.mp3");

    let path = Path::new(&file_path);

    if !path.exists() {
        download_song(&video_id, "youtube").await;
    }

    songbird::input::File::new(file_path)
}

async fn download_song(video_id: &str, provider: &str) {
    let video = rusty_ytdl::Video::new(video_id).unwrap();

    match video
        .download(format!("music/{provider}/{video_id}.mp3"))
        .await
    {
        Ok(_) => {}
        Err(err) => {
            error!("{err}");
            error!("ERRORED id {video_id}");
        }
    }
}

async fn join_and_play(manager_get: Option<Arc<Mutex<Call>>>, video_id: String) -> Result<(), ()> {
    let src = get_video_youtube(video_id).await;

    if let Some(handler_lock) = manager_get {
        let mut handler = handler_lock.lock().await;

        let _ = handler.enqueue_input(src.into()).await;
        Ok(())
    } else {
        Err(())
    }
}

async fn queue_playlist(
    cache: Arc<Cache>,
    playlist: Vec<PlaylistItem>,
    manager: Arc<Songbird>,
    guild_id: GuildId,
    channel_id: ChannelId,
) {
    let mut videos = vec![];

    for vid in playlist {
        videos.push(
            get_video_youtube(vid.snippet.unwrap().resource_id.unwrap().video_id.unwrap()).await,
        );
    }

    let manager_get = join_channel(cache, manager, guild_id, channel_id).await;
    if let Some(handler_lock) = manager_get {
        let mut handler = handler_lock.lock().await;
        for video in videos {
            let _ = handler.enqueue_input(video.into()).await;
        }
    }

    // for video in playlist {
    //     let src = get_video_youtube(
    //         video
    //             .snippet
    //             .unwrap()
    //             .resource_id
    //             .unwrap()
    //             .video_id
    //             .unwrap(),
    //     )
    //     .await;
    // }
}
