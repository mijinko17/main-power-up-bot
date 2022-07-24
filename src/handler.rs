use std::future::Future;

use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteraction,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;

// use crate::commands::main_power::command::{
//     interact_main_power_up_command, register_main_power_up_command,
// };
use crate::commands::main_power::constants::MAIN_POWER_UP_COMMAND_NAME;

pub struct Handler;

// #[async_trait]
// impl EventHandler for Handler {
//     async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
//         if let Interaction::ApplicationCommand(command) = interaction {
//             println!("Received command interaction: {:#?}", command);
//             match command.data.name.as_str() {
//                 MAIN_POWER_UP_COMMAND_NAME => interact_main_power_up_command(&ctx, &command).await,
//                 "takashi" => {
//                     if let Err(why) = command
//                         .create_interaction_response(&ctx.http, |response| {
//                             response
//                                 .kind(InteractionResponseType::ChannelMessageWithSource)
//                                 .interaction_response_data(|message| message.content("†TAKASHI†"))
//                         })
//                         .await
//                     {
//                         println!("Cannot respond to slash command: {}", why);
//                     }
//                 }
//                 _ => {
//                     if let Err(why) = command
//                         .create_interaction_response(&ctx.http, |response| {
//                             response
//                                 .kind(InteractionResponseType::ChannelMessageWithSource)
//                                 .interaction_response_data(|message| {
//                                     message.content("not implemented")
//                                 })
//                         })
//                         .await
//                     {
//                         println!("Cannot respond to slash command: {}", why);
//                     }
//                 }
//             }
//         }
//     }

//     async fn ready(&self, ctx: Context, ready: Ready) {
//         println!("{} is connected!", ready.user.name);

//         let takashi_command =
//             ApplicationCommand::create_global_application_command(&ctx.http, |command| {
//                 command.name("takashi").description("†TAKASHI†")
//             })
//             .await;
//         println!(
//             "I created the following global slash command: {:#?}",
//             takashi_command
//         );

//         let main_power_up_command = register_main_power_up_command(&ctx).await;
//         println!(
//             "I created the following global slash command: {:#?}",
//             main_power_up_command
//         );
//     }
// }

pub struct SlashCommandHandler {
    pub commands: &'static Vec<Box<dyn SlashCommand + Sync + Send>>,
}

#[async_trait]
impl EventHandler for SlashCommandHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            let received = command.data.name.as_str();
            for slash_command in self.commands {
                if slash_command.name().eq(received) {
                    slash_command.interact(&ctx, &command).await;
                    break;
                }
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        for slash_command in self.commands {
            let cmd = ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                slash_command.register(command.name(slash_command.name()))
            })
            .await;
            println!("I created the following global slash command: {:#?}", cmd);
        }
    }
}

#[async_trait]
pub trait SlashCommand {
    fn name(&self) -> &'static str;
    async fn interact(&self, ctx: &Context, command: &ApplicationCommandInteraction);
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
    }
}
