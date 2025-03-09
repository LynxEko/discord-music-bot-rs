use std::{path::Path, process::Command};

use tracing::{error, info};

pub async fn get_video_youtube(
    video_id: String,
) -> songbird::input::File<impl AsRef<Path> + Clone> {
    let file_path = format!("music/youtube/{video_id}.mp3");

    let path = Path::new(&file_path);

    if !path.exists() {
        download_song(&video_id, "youtube").await;
    }

    songbird::input::File::new(file_path)
}

async fn download_song(video_id: &str, provider: &str) {
    if provider == "youtube" {
        let Err(err) = youtube_dl_3(video_id).await else {
            return;
        };
        error!("{err}");
        error!("[yt-dlp] ERRORED downloading id {video_id}");

        let Err(err) = youtube_dl_1(video_id).await else {
            return;
        };
        error!("{err}");
        error!("[rusty_ytdl] ERRORED downloading id {video_id}");

        let Err(err) = youtube_dl_2(video_id).await else {
            return;
        };
        error!("{err}");
        error!("[rustube] ERRORED downloading id {video_id}");
    } else {
        error!("unkown mp3 provider [{provider}]");
    }
}

async fn youtube_dl_1(video_id: &str) -> Result<(), String> {
    info!("Downloading using [rusty_ytdl]");
    let video = rusty_ytdl::Video::new(video_id).unwrap();
    video
        .download(format!("music/youtube/{video_id}.mp3"))
        .await
        .map_err(|err| format!("{err}"))?;
    Ok(())
}

async fn youtube_dl_2(video_id: &str) -> Result<(), String> {
    info!("Downloading using [rustube]");
    let id = rustube::Id::from_string(video_id.to_owned())?;
    let video = rustube::Video::from_id(id)
        .await
        .map_err(|err| format!("{err}"))?;

    video
        .best_audio()
        .unwrap()
        .download_to(format!("music/youtube/{video_id}.mp3"))
        .await
        .map_err(|err| format!("{err}"))?;

    Ok(())
}

async fn youtube_dl_3(video_id: &str) -> Result<(), String> {
    info!("Downloading using [yt-dlp]");

    let mut child = Command::new("yt-dlp")
        .arg(format!("https://www.youtube.com/watch?v={video_id}"))
        .arg("--embed-thumbnail")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(format!("music/youtube/{video_id}.mp3"))
        .spawn()
        .unwrap();

    child.wait().unwrap();
    Ok(())
}
