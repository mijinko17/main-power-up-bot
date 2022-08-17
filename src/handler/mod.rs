use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::model::prelude::application_command::ApplicationCommand;
use serenity::prelude::*;

use self::slash_command::container::SlashCommandContainer;

pub mod slash_command;

pub struct SlashCommandHandler {
    container: SlashCommandContainer,
}

impl SlashCommandHandler {
    pub fn new(container: SlashCommandContainer) -> Self {
        Self { container }
    }
}

#[async_trait]
impl EventHandler for SlashCommandHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            if let Some(slash_command) = self.container.get(command.data.name.as_str()) {
                slash_command.interact(&ctx, &command).await;
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        for slash_command in self.container.iter() {
            let slash_command_result =
                ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                    slash_command.register(command.name(slash_command.name()))
                })
                .await;
            println!(
                "I created the following global slash command: {:#?}",
                slash_command_result
            );
        }
    }
}
