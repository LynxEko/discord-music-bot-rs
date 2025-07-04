use std::collections::HashMap;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use tracing::{error, info, trace};

use crate::config::Config;
use crate::my_commands::audio::leave::Leave;
use crate::my_commands::audio::play::Play;
use crate::my_commands::audio::shuffle::Shuffle;
use crate::my_commands::audio::skip::Skip;
use crate::my_commands::base_command::{BaseCommand, CommandResponse};
use crate::my_commands::ping::Ping;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            Config::get()
                .guild_id
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands_to_add: Vec<_> = self
            .command_list()
            .iter()
            .map(|c| c.generate_create_command())
            .collect();

        let _commands = guild_id.set_commands(&ctx.http, commands_to_add).await;

        let command_names: Vec<_> = self
            .command_list()
            .iter()
            .map(|c| c.command_name())
            .collect();
        info!("I now have the following guild slash commands: {command_names:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            trace!("Received command interaction: {command:#?}");
            let command_map = self.command_map();

            let response = if let Some(cmd) = command_map.get(command.data.name.as_str()) {
                cmd.run(&command, &ctx).await
            } else {
                CommandResponse::String("not implemented :(".to_string())
            };

            let CommandResponse::String(response) = response else {
                return;
            };

            let data = CreateInteractionResponseMessage::new().content(response);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                error!("Cannot respond to slash command: {why}");
            }
        }
    }
}
impl Handler {
    pub fn new() -> Self {
        Self
    }

    fn command_list(&self) -> Vec<Box<dyn BaseCommand + Send>> {
        vec![
            Box::new(Ping),
            Box::new(Leave),
            Box::new(Play),
            Box::new(Shuffle),
            Box::new(Skip),
        ]
    }

    fn command_map(&self) -> HashMap<&str, Box<dyn BaseCommand + Send>> {
        let commands = self.command_list();

        commands
            .into_iter()
            .map(|c| (c.command_name(), c))
            .collect()
    }
}
