pub mod play;
pub mod playlist;

use serenity::all::CreateCommand;
pub fn register_all() -> Vec<CreateCommand> {
    return vec![playlist::register()];
}
