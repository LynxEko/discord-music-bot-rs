pub mod leave;
pub mod mute;
pub mod play;
pub mod shuffle;
pub mod skip;

use serenity::all::CreateCommand;
pub fn register_all() -> Vec<CreateCommand> {
    return vec![
        // deafen::register(),
        // join::register(),
        leave::register(),
        // mute::register(),
        play::register(),
        // undeafen::register(),
        // unmute::register(),
        shuffle::register(),
        skip::register(),
    ];
}
