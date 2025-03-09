use std::sync::Arc;

use serenity::{
    all::{Cache, ChannelId, GuildId},
    async_trait,
};
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler},
    Songbird,
};
// use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::playlist::channel::leave;

#[derive(Clone)]
pub struct AutoDisconnectionHandler {
    pub manager: Arc<Songbird>,
    // pub handler_lock: Arc<Mutex<Call>>,
    pub guild_id: GuildId,
    pub cache: Arc<Cache>,
}

#[async_trait]
impl VoiceEventHandler for AutoDisconnectionHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::ClientDisconnect(client_disconnect) => {
                info!("Client disconnected {}", client_disconnect.user_id.0);
                self.check_for_clients(client_disconnect.user_id.0).await;
            }
            _ => {}
        }
        None
    }
}

impl AutoDisconnectionHandler {
    async fn check_for_clients(&self, user_id_just_disconnected: u64) {
        let Some(handler_lock) = self.manager.get(self.guild_id) else {
            error!("Not in a call (?? this should not happen)");
            return;
        };

        let handler = handler_lock.lock().await;
        let channel_id = ChannelId::new(handler.current_channel().unwrap().0.into());
        drop(handler);

        let mut user_amount = 0;
        {
            let guild = self.guild_id.to_guild_cached(&self.cache).unwrap();
            for us in guild.voice_states.iter() {
                let vs = us.1;
                if vs.channel_id.is_some() && vs.channel_id.unwrap() == channel_id {
                    if us.0.get() != user_id_just_disconnected {
                        info!("user in channel {}", us.0.get());
                        user_amount += 1;
                    } else {
                        info!(
                            "user in channel {}, just disconnected not counting",
                            us.0.get()
                        );
                    }
                }
            }
        }
        info!("USER AMOUNT {user_amount}");

        if user_amount == 1 {
            leave(self.manager.clone(), self.guild_id).await;
        } else {
            warn!("{} users still in call, wont leave yet", user_amount - 1);
        }
    }
}
