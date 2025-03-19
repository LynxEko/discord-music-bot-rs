use google_youtube3::api::PlaylistItem;

use crate::youtube::auth::YouTubeAuth;

pub async fn get_playlist(playlist_id: &String) -> Result<Vec<PlaylistItem>, String> {
    let youtube_auth = YouTubeAuth::get();

    let hub = google_youtube3::YouTube::new(
        google_youtube3::hyper_util::client::legacy::Client::builder(
            google_youtube3::hyper_util::rt::TokioExecutor::new(),
        )
        .build(
            google_youtube3::hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        youtube_auth.auth.clone(),
    );

    let res = hub
        .playlist_items()
        .list(&vec!["snippet".into()])
        .playlist_id(&playlist_id)
        .max_results(2)
        .doit()
        .await;

    match res {
        Err(e) => Err(format!("{e}")),
        Ok(res) => {
            let mut playlist_items = res.1.items.clone().unwrap_or(vec![]);
            let mut current_response = res;
            while let Some(next_page_token) = current_response.1.next_page_token.clone() {
                let res = hub
                    .playlist_items()
                    .list(&vec!["snippet".into()])
                    .playlist_id(&playlist_id)
                    .max_results(50)
                    .page_token(&next_page_token)
                    .doit()
                    .await;
                match res {
                    Err(e) => return Err(format!("{e}")),
                    Ok(res) => {
                        playlist_items.append(&mut res.1.items.clone().unwrap_or(vec![]));
                        current_response = res;
                    }
                }
            }
            Ok(playlist_items)
        }
    }
}
