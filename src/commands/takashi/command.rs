use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

use crate::handler::SlashCommandBase;

pub struct Takashi;

#[async_trait]
impl SlashCommandBase for Takashi {
    type Input = ();
    type Item = ();

    fn name(&self) -> &'static str {
        "takashi"
    }

    fn extract(&self, _: &ApplicationCommandInteraction) -> Option<Self::Input> {
        Some(())
    }

    async fn convert(&self, _: Self::Input) -> Option<Self::Item> {
        Some(())
    }

    fn interaction<'a, 'b>(
        &self,
        _: Self::Input,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("†TAKASHI†"))
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.description("†TAKASHI†")
    }
}
