use serenity::async_trait;
use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteraction,
};
use serenity::model::interactions::Interaction;
use serenity::prelude::*;

#[derive(Default)]
pub struct SlashCommandContainer {
    pub commands: Vec<Box<dyn SlashCommand + Sync + Send>>,
}

impl SlashCommandContainer {
    pub fn add_command<T: 'static + SlashCommand + Send + Sync>(mut self, command: T) -> Self {
        self.commands.push(Box::new(command));
        self
    }
    #[allow(clippy::borrowed_box)]
    pub fn get(&self, name: &str) -> Option<&Box<dyn SlashCommand + Sync + Send>> {
        self.commands.iter().find(|command| command.name().eq(name))
    }
    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn SlashCommand + Sync + Send>> {
        self.commands.iter()
    }
}

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

#[async_trait]
pub trait SlashCommandBase {
    type Input;
    type Item;
    fn name(&self) -> &'static str;
    fn extract(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Option<Self::Input>;
    fn extract_failed_response<'a, 'b>(
        &self,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
    }
    async fn convert(&self, input: Self::Input) -> Option<Self::Item>;
    fn interaction<'a, 'b>(
        &self,
        value: Self::Item,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b>;
    fn convert_failed_response<'a, 'b>(
        &self,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
    }
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
        if let Some(value) = self.extract(ctx, command) {
            if let Some(item) = self.convert(value).await {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        self.interaction(item, response)
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            } else if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    self.convert_failed_response(response)
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        } else if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                self.extract_failed_response(response)
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        }
    }
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        self.register(command)
    }
}
