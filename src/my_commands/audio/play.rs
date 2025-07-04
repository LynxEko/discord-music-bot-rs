use std::time::Duration;

use serenity::all::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;

use crate::my_commands::base_command::{BaseCommand, CommandResponse};
use crate::playlist::channel::join_channel;
use crate::playlist::key::{add_now, add_to_playlist, get_playlist_lock};
use crate::youtube;

pub struct Play;

#[async_trait]
impl BaseCommand for Play {
    fn command_name(&self) -> &'static str {
        "play"
    }

    fn generate_create_command(&self) -> CreateCommand {
        CreateCommand::new(self.command_name())
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

    async fn run(&self, command: &CommandInteraction, ctx: &Context) -> CommandResponse {
        let options = command.data.options();

        let Some(ResolvedOption {
            value: ResolvedValue::String(url_value),
            ..
        }) = options.get(0)
        else {
            return CommandResponse::String("Must provide a URL to a video or audio".into());
        };

        let Ok(url_value) = url::Url::parse(url_value) else {
            return CommandResponse::String(
                "Must provide a valid URL to a video, audio or playlist".into(),
            );
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
            let guild = command
                .guild_id
                .unwrap()
                .to_guild_cached(&ctx.cache)
                .unwrap();
            let channel_id = guild
                .voice_states
                .get(&command.user.id)
                .and_then(|voice_state| voice_state.channel_id)
                .unwrap();

            (guild.id, channel_id)
        };

        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();
        let manager_get = join_channel(ctx, manager, guild_id, channel_id).await;
        let handler_lock = manager_get.unwrap();

        let response = if let Some(playlist_id) = playlist_id {
            // https://github.com/serenity-rs/serenity/blob/current/examples/e13_parallel_loops/src/main.rs
            if let Ok(playlist) = youtube::playlist::get_playlist(&playlist_id).await {
                let first_snippet = playlist.first().unwrap().snippet.clone().unwrap();
                let playlist_length = playlist.len();

                let video_id = first_snippet.resource_id.unwrap().video_id.unwrap();

                let playlist_lock = get_playlist_lock(ctx).await;
                add_to_playlist(
                    playlist_lock.clone(),
                    guild_id,
                    handler_lock.clone(),
                    video_id,
                )
                .await;
                {
                    let guild_id = guild_id.clone();
                    let handler_lock = handler_lock;
                    tokio::spawn(async move {
                        let mut playlist = playlist;
                        playlist.remove(0);
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        for song in playlist {
                            add_to_playlist(
                                playlist_lock.clone(),
                                guild_id,
                                handler_lock.clone(),
                                song.snippet.unwrap().resource_id.unwrap().video_id.unwrap(),
                            )
                            .await;
                        }
                    });
                }

                format!("Playing {} songs from playist", playlist_length)
            } else {
                "Could not load the playlist".to_string()
            }
        } else if let Some(video_id) = video_id {
            let playlist_lock = get_playlist_lock(ctx).await;
            add_now(
                playlist_lock.clone(),
                guild_id,
                handler_lock.clone(),
                video_id,
            )
            .await;

            "Added new song, will play next".to_string()
        } else {
            "Other sources not implemented".to_string()
        };

        CommandResponse::String(response)
    }
}
