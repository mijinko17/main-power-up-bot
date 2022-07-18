use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "takashi" => "†TAKASHI†".to_string(),
                "main_power" => "hoge".to_string(),
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
                            .add_string_choice("zap", "zap")
                    })
            })
            .await;
        println!(
            "I created the following global slash command: {:#?}",
            main_power_up_command
        );
    }
}
