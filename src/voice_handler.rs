use std::sync::Arc;

use serenity::{
    all::{Cache, ChannelId, GuildId},
    async_trait,
};
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler},
    Call,
};
use tokio::sync::Mutex;

pub struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(track_list) => {
                for (state, handle) in *track_list {
                    println!(
                        "Track {:?} encountered an error: {:?}",
                        handle.uuid(),
                        state.playing
                    );
                }
            }
            _ => {}
        }

        None
    }
}

#[derive(Clone)]
pub struct TrackHandler {
    pub handler_lock: Arc<Mutex<Call>>,
    pub guild_id: GuildId,
    pub cache: Arc<Cache>,
}

#[async_trait]
impl VoiceEventHandler for TrackHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(track_list) => {
                for (state, handle) in *track_list {
                    println!("Track {:?} ended: {:?}", handle.uuid(), state.playing);
                }
            }
            EventContext::ClientDisconnect(client_disconnect) => {
                println!("Client disconnected {}", client_disconnect.user_id.0);
                self.check_for_clients(client_disconnect.user_id.0).await;
            }
            _ => {}
        }
        None
    }
}

impl TrackHandler {
    pub async fn check_for_clients(&self, user_id_just_disconnected: u64) {
        let mut handler = self.handler_lock.lock().await;
        let channel_id = ChannelId::new(handler.current_channel().unwrap().0.into());

        let mut user_amount = 0;
        {
            let guild = self.guild_id.to_guild_cached(&self.cache).unwrap();
            for us in guild.voice_states.iter() {
                let vs = us.1;
                if vs.channel_id.is_some() && vs.channel_id.unwrap() == channel_id {
                    if us.0.get() != user_id_just_disconnected {
                        println!("user in channel {}", us.0.get());
                        user_amount += 1;
                    } else {
                        println!(
                            "user in channel {}, just disconnected not counting",
                            us.0.get()
                        );
                    }
                }
            }
        }
        println!("USER AMOUNT {user_amount}");

        if user_amount == 1 {
            handler.leave().await.unwrap();
        }
    }
}
