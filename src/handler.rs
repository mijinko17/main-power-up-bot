use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::commands::main_power::command::main_power_up_response;
use crate::commands::main_power::constants::MainWeaponType;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "takashi" => "†TAKASHI†".to_string(),
                "main_power" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("error")
                        .resolved
                        .as_ref()
                        .expect("error");
                    if let ApplicationCommandInteractionDataOptionValue::String(str) = options {
                        let main_weapon_type = MainWeaponType::from_str(str);
                        if let Some(weapon) = main_weapon_type {
                            main_power_up_response(weapon)
                        } else {
                            "ccc".to_string()
                        }
                    } else {
                        "bbb".to_string()
                    }
                }
                _ => command.data.name.clone(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
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

        let main_power_up_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("main_power")
                    .description("メイン性能の効果を応答します.")
                    .create_option(|option| {
                        option
                            .name("weapon")
                            .description("メインウェポン")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                            .add_string_choice("スプラチャージャー", "スプラチャージャー")
                            .add_string_choice("N-ZAP", "N-ZAP")
                    })
            })
            .await;
        println!(
            "I created the following global slash command: {:#?}",
            main_power_up_command
        );
    }
}
