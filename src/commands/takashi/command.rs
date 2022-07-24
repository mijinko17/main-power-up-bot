use serenity::{
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

use crate::handler::SlashCommandBase;

pub struct Takashi;

impl SlashCommandBase for Takashi {
    type Input = ();

    fn name(&self) -> &'static str {
        "takashi"
    }

    fn extract(&self, _: &ApplicationCommandInteraction) -> Option<Self::Input> {
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
