use serenity::async_trait;
use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

pub mod container;

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
