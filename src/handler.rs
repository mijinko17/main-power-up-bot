use serenity::async_trait;
use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteraction,
};
use serenity::model::interactions::Interaction;
use serenity::prelude::*;

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
            println!("{}", slash_command.name());
            println!("I created the following global slash command: {:#?}", cmd);
        }
    }
}

#[async_trait]
pub trait SlashCommandBase {
    type Value;
    fn name(&self) -> &'static str;
    fn extract(&self, command: &ApplicationCommandInteraction) -> Option<Self::Value>;
    fn interaction<'a, 'b>(
        &self,
        value: Self::Value,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b>;
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
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

#[async_trait]
impl<U, T> SlashCommand for U
where
    U: SlashCommandBase<Value = T> + Send + Sync,
    T: Send + Sync,
{
    fn name(&self) -> &'static str {
        self.name()
    }
    async fn interact(&self, ctx: &Context, command: &ApplicationCommandInteraction) {
        if let Some(value) = self.extract(command) {
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    self.interaction(value, response)
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        } else {
            println!("Invalid input.");
        }
    }
}
