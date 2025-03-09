//! Requires the "client", "standard_framework", and "voice" features be enabled in your
//! Cargo.toml, like so:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["client", "standard_framework", "voice"]
//! ```
// use std::env;

use std::{collections::HashMap, sync::Arc};

use playlist::key::PlaylistKey;
// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::SerenityInit;

// Event related imports to detect track creation failures.
// use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};

// To turn user URLs into playable audio, we'll use yt-dlp.
// use songbird::input::YoutubeDl;

// YtDl requests need an HTTP client to operate -- we'll create and store our own.
use reqwest::Client as HttpClient;

// Import the `Context` to handle commands.
// use serenity::client::Context;

// use serenity::{
//     async_trait,
//     client::{Client, EventHandler},
//     framework::{
//         standard::{
//             macros::{command, group},
//             Args,
//             CommandResult,
//             Configuration,
//         },
//         StandardFramework,
//     },
//     model::{channel::Message, gateway::Ready},
//     prelude::{GatewayIntents, TypeMapKey},
//     Result as SerenityResult,
// };

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

// #[group]
// #[commands(deafen, join, leave, mute, play, ping, undeafen, unmute)]
// struct General;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Configure the client with your Discord bot token in the environment.
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let token = Config::get().discord_token.clone();

    // let framework = StandardFramework::new().group(&GENERAL_GROUP);
    // framework.configure(Configuration::new().prefix("~"));

    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    // let intents = GatewayIntents::all();

    YouTubeAuth::init().await;

    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        // .framework(framework)
        .register_songbird()
        // We insert our own HTTP client here to make use of in
        // `~play`. If we wanted, we could supply cookies and auth
        // details ahead of time.
        //
        // Generally, we don't want to make a new Client for every request!
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

// #[command]
// async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
//     check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);
//     Ok(())
// }

// /// Checks that a message successfully sent; if not, then logs why to stdout.
// fn check_msg(result: SerenityResult<Message>) {
//     if let Err(why) = result {
//         error!("Error sending message: {:?}", why);
//     }
// }
