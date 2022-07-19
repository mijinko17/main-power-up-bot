use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::commands::main_power::command::{
    interact_main_power_up_command, register_main_power_up_command,
};
use crate::commands::main_power::constants::MAIN_POWER_UP_COMMAND_NAME;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            match command.data.name.as_str() {
                MAIN_POWER_UP_COMMAND_NAME => interact_main_power_up_command(&ctx, &command).await,
                "takashi" => {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content("†TAKASHI†"))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
                _ => {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.content("not implemented")
                                })
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let takashi_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command.name("takashi").description("†TAKASHI†")
            })
            .await;
        println!(
            "I created the following global slash command: {:#?}",
            takashi_command
        );

        let main_power_up_command = register_main_power_up_command(&ctx).await;
        println!(
            "I created the following global slash command: {:#?}",
            main_power_up_command
        );
    }
}
