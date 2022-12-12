use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
                InteractionResponseType,
            },
        },
        user::User,
    },
    prelude::Context,
};

use crate::handler::slash_command::SlashCommandBase;

use super::constants::OKITE;

pub struct Okite;

#[async_trait]
impl SlashCommandBase for Okite {
    type Input = User;
    type Item = User;

    fn name(&self) -> &'static str {
        "okite"
    }

    fn extract(&self, _: &Context, command: &ApplicationCommandInteraction) -> Option<Self::Input> {
        let a = command
            .data
            .options
            .get(0)
            .expect("error")
            .resolved
            .as_ref()
            .expect("error");
        if let CommandDataOptionValue::User(user, _) = a {
            Some(user.clone())
        } else {
            None
        }
    }

    async fn convert(&self, user: Self::Input) -> Option<Self::Item> {
        Some(user)
    }

    fn interaction<'a, 'b>(
        &self,
        user: Self::Item,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(mention(user, OKITE)))
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .description("ユーザを起こします。")
            .create_option(|option| {
                option
                    .name("user")
                    .kind(CommandOptionType::User)
                    .description("起こす人")
                    .required(true)
            })
    }
}

fn mention(user: User, content: &str) -> String {
    format!("<@{}> {}", user.id, content)
}
