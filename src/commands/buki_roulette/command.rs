use rand::{seq::SliceRandom, thread_rng};
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::{
        interactions::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::{ChannelId, Guild},
    },
    prelude::Context,
};

use crate::handler::slash_command::SlashCommandBase;

use super::constants::WEAPONS;

pub struct BukiRoulette;

#[async_trait]
impl SlashCommandBase for BukiRoulette {
    type Input = (Guild, ChannelId);
    type Item = Vec<(String, String, bool)>;

    fn name(&self) -> &'static str {
        "buki_roulette"
    }

    fn extract(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Option<Self::Input> {
        let guild = command.guild_id?.to_guild_cached(&ctx.cache)?;
        let channel_id = guild.voice_states.get(&command.user.id)?.channel_id?;
        Some((guild, channel_id))
    }

    fn extract_failed_response<'a, 'b>(
        &self,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.content("ボイスチャンネル, サーバの取得に失敗しました")
            })
    }

    async fn convert(&self, (guild, channel_id): Self::Input) -> Option<Self::Item> {
        let mut rng = thread_rng();
        let channel_members = guild
            .voice_states
            .values()
            .filter(|voice_state| voice_state.channel_id.eq(&Some(channel_id)))
            .map(|voice_state| voice_state.user_id)
            .filter_map(|uid| guild.members.get(&uid))
            .map(|member| {
                member
                    .nick
                    .clone()
                    .unwrap_or_else(|| member.user.name.clone())
            })
            .map(|member_name| {
                (
                    member_name,
                    WEAPONS.choose(&mut rng).unwrap().to_string(),
                    false,
                )
            })
            .collect();
        Some(channel_members)
    }

    fn interaction<'a, 'b>(
        &self,
        item: Self::Item,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.embed(|embed| {
                    embed
                        .title("ブキルーレット")
                        .fields(item)
                        .colour(serenity::utils::Colour::KERBAL)
                })
            })
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.description("同じボイスチャンネルにいる各メンバーについてブキルーレットを回します")
    }
}
