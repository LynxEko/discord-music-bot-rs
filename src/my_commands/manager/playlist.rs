use google_youtube3::api::PlaylistItem;
use indicatif::ProgressBar;
use rusty_ytdl::Video;
use serenity::all::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use songbird::input::YoutubeDl;

use crate::config::Config;
use crate::youtube;

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &CommandInteraction,
) -> String {
    // if let Some(ResolvedOption {
    //     value: ResolvedValue::String(url),
    //     ..
    // }) = options.first()
    // {}
    let playlist_id = "PL7YlFpbAjlTM9_9ZxgVSmw92vUaxTfzUX".to_owned();

    let res = youtube::playlist::get_playlist(&playlist_id).await;

    match res {
        Err(e) => {
            format!("{e}")
        }
        Ok(res) => {
            println!("got {} videos from the playlist", res.len());

            println!("{:#?}", res[0]);

            download_playlist(&res).await;

            format!("Success!")
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("playlist")
        .description("manage the playlist")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "url",
                "the link where the youtube playlist is",
            )
            .required(true),
        )
}

async fn download_playlist(playlist: &Vec<PlaylistItem>) {
    let bar = ProgressBar::new(playlist.len() as u64);
    for video in playlist.iter() {
        let snippet = video.snippet.clone().unwrap();
        let video_url = snippet.resource_id.unwrap().video_id.unwrap();
        let video = Video::new(&video_url).unwrap();

        video
            .download(format!("music/{}.mp3", video_url))
            .await
            .unwrap();
        bar.inc(1);
    }
    bar.finish();
}
