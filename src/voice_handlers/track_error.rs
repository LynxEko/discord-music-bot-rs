use serenity::async_trait;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
// use tokio::sync::Mutex;
use tracing::error;

pub struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(track_list) => {
                for (state, handle) in *track_list {
                    error!(
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
