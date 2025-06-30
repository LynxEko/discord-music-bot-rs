use std::{collections::HashMap, sync::Arc};

use playlist::key::PlaylistKey;
use songbird::SerenityInit;

use reqwest::Client as HttpClient;

use serenity::prelude::*;
use tracing::{error, info, Level};

use crate::handler::Handler;

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

mod config;
use config::Config;
mod handler;
mod my_commands;
mod playlist;
mod voice_handlers;
mod youtube;

use youtube::auth::YouTubeAuth;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let discord_token = Config::get().discord_token.clone();

    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    YouTubeAuth::init().await;

    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(Handler)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<PlaylistKey>(Arc::new(RwLock::new(HashMap::new())))
    }

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| error!("Client ended: {:?}", why));
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    info!("Received Ctrl-C, shutting down.");
}
