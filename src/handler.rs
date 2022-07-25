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
            println!("I created the following global slash command: {:#?}", cmd);
        }
    }
}

#[async_trait]
pub trait SlashCommandBase {
    type Input;
    type Item;
    fn name(&self) -> &'static str;
    fn extract(&self, command: &ApplicationCommandInteraction) -> Option<Self::Input>;
    async fn convert(&self, input: Self::Input) -> Option<Self::Item>;
    fn interaction<'a, 'b>(
        &self,
        value: Self::Item,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b>;
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand;
}

#[async_trait]
pub trait SlashCommand {
    fn name(&self) -> &'static str;
    async fn interact(&self, ctx: &Context, command: &ApplicationCommandInteraction);
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand;
}

#[async_trait]
impl<T, INPUT, ITEM> SlashCommand for T
where
    T: SlashCommandBase<Input = INPUT, Item = ITEM> + Send + Sync,
    INPUT: Send + Sync,
    ITEM: Send + Sync,
{
    fn name(&self) -> &'static str {
        self.name()
    }
    async fn interact(&self, ctx: &Context, command: &ApplicationCommandInteraction) {
        if let Some(value) = self.extract(command) {
            if let Some(item) = self.convert(value).await {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        self.interaction(item, response)
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        } else {
            println!("Invalid input.");
        }
    }
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        self.register(command)
    }
}
