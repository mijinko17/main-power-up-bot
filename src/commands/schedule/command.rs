use std::{
    sync::Mutex,
    time::{Duration, SystemTime},
};

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

use crate::handler::slash_command::SlashCommandBase;

static SCHEDULE_CACHE: Mutex<Option<Response>> = Mutex::new(None);

#[async_trait]
pub trait ScheduleRepository {
    async fn schedule(&self) -> Option<Response>;
}

pub struct CachingScheduleRepository;

#[async_trait]
impl ScheduleRepository for CachingScheduleRepository {
    async fn schedule(&self) -> Option<Response> {
        let cache = SCHEDULE_CACHE.lock().unwrap().clone();
        if let Some(c) = cache {
            let now = SystemTime::now();
            let next_schedule = SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_secs(c.result[0].end_t))
                .unwrap();
            if now < next_schedule {
                println!("Use cache");
                return Some(c);
            }
        }
        let res = reqwest::Client::new()
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
        *SCHEDULE_CACHE.lock().unwrap() = Some(res.clone());
        println!("Fetch");
        Some(res)
    }
}

pub struct Schedule<T>
where
    T: ScheduleRepository,
{
    repository: T,
}

impl Default for Schedule<CachingScheduleRepository> {
    fn default() -> Self {
        Self {
            repository: CachingScheduleRepository,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub result: Vec<ScheduleResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleResponse {
    pub rule: String,
    pub maps: Vec<String>,
    pub end_t: u64,
}

#[async_trait]
impl<T> SlashCommandBase for Schedule<T>
where
    T: ScheduleRepository + Send + Sync,
{
    type Input = ();
    type Item = Response;

    fn name(&self) -> &'static str {
        "schedule"
    }

    fn extract(&self, _: &Context, _: &ApplicationCommandInteraction) -> Option<Self::Input> {
        Some(())
    }

    async fn convert(&self, _: Self::Input) -> Option<Self::Item> {
        self.repository.schedule().await
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
