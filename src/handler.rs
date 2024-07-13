use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use crate::config::Config;
use crate::my_commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            // env::var("GUILD_ID")
            //     .expect("Expected GUILD_ID in environment")
            Config::get()
                .guild_id
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands_to_add = vec![
            vec![my_commands::ping::register()],
            my_commands::audio::register_all(),
            my_commands::manager::register_all(),
        ]
        .concat();

        let commands = guild_id.set_commands(&ctx.http, commands_to_add).await;

        println!("I now have the following guild slash commands: {commands:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                // "ping" => Some(my_commands::ping::run(&command.data.options())),
                // "join" => Some(my_commands::audio::join::run(&ctx, &command).await),
                "leave" => Some(my_commands::audio::leave::run(&ctx, &command).await),
                "play" => Some(
                    my_commands::audio::play::run(&command.data.options(), &ctx, &command).await,
                ),
                "shuffle" => Some(my_commands::audio::shuffle::run(&ctx, &command).await),
                "skip" => Some(my_commands::audio::skip::run(&ctx, &command).await),
                // "playlist" => Some(
                //     my_commands::manager::playlist::run(&command.data.options(), &ctx, &command)
                //         .await,
                // ),
                // "id" => Some(commands::id::run(&command.data.options())),
                // "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                // "modal" => {
                //     commands::modal::run(&ctx, &command).await.unwrap();
                //     None
                // },
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}
