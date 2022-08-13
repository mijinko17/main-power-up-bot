use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
    utils::Colour,
};

use crate::handler::SlashCommandBase;

pub struct Schedule;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub result: Vec<ScheduleResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleResponse {
    pub rule: String,
    pub maps: Vec<String>,
}

#[async_trait]
impl SlashCommandBase for Schedule {
    type Input = ();
    type Item = Response;

    fn name(&self) -> &'static str {
        "schedule"
    }

    fn extract(&self, _: &Context, _: &ApplicationCommandInteraction) -> Option<Self::Input> {
        Some(())
    }

    async fn convert(&self, _: Self::Input) -> Option<Self::Item> {
        let stage = reqwest::Client::new()
            .get("https://spla2.yuu26.com/gachi/now")
            .header(
                reqwest::header::USER_AGENT,
                "main-power-up-bot/0.1 (twitter @mijinko_cpp)",
            )
            .send()
            .await
            .ok()?
            .json::<Response>()
            .await
            .ok()?;
        Some(stage)
    }

    fn interaction<'a, 'b>(
        &self,
        stage: Self::Item,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.embed(|emb| {
                    emb.title(stage.result[0].rule.clone())
                        .description(stage.result[0].maps.iter().join(", "))
                        .colour(Colour::BLITZ_BLUE)
                })
            })
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.description("ステージスケジュール")
    }
}
