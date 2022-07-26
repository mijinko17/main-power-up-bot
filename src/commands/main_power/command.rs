use std::str::FromStr;

use itertools::Itertools;
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::interactions::{
        application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
            ApplicationCommandOptionType,
        },
        InteractionResponseType,
    },
};

use crate::handler::SlashCommandBase;

use super::{
    constants::{MAIN_POWER_UP_COMMAND_NAME, MAIN_WEAPON_NAME},
    domain::{MainWeapon, MainWeaponType},
};

pub struct MainPowerUp;

#[async_trait]
impl SlashCommandBase for MainPowerUp {
    type Input = String;
    type Item = MainWeapon;

    fn name(&self) -> &'static str {
        MAIN_POWER_UP_COMMAND_NAME
    }

    fn extract(&self, command: &ApplicationCommandInteraction) -> Option<String> {
        let options = command
            .data
            .options
            .get(0)
            .expect("error")
            .resolved
            .as_ref()
            .expect("error");
        if let ApplicationCommandInteractionDataOptionValue::String(str) = options {
            Some(str.clone())
        } else {
            None
        }
    }

    async fn convert(&self, input: String) -> Option<MainWeapon> {
        MainWeaponType::from_str(&input).ok().map(MainWeapon::from)
    }

    fn interaction<'a, 'b>(
        &self,
        weapon: MainWeapon,
        response: &'a mut CreateInteractionResponse<'b>,
    ) -> &'a mut CreateInteractionResponse<'b> {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.embed(|emb| {
                    emb.title(weapon.main_weapon_type.to_string())
                        .description(to_display_string(weapon))
                        .colour(serenity::utils::Colour::MAGENTA)
                })
            })
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .description("メイン性能の効果を応答します.")
            .create_option(|option| {
                option
                    .name("weapon")
                    .description("メインウェポン")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
                    .add_string_choice(
                        MAIN_WEAPON_NAME.sploosh_o_matic,
                        MAIN_WEAPON_NAME.sploosh_o_matic,
                    )
                    .add_string_choice(
                        MAIN_WEAPON_NAME.splat_charger,
                        MAIN_WEAPON_NAME.splat_charger,
                    )
                    .add_string_choice(MAIN_WEAPON_NAME.n_zap, MAIN_WEAPON_NAME.n_zap)
                    .add_string_choice(
                        MAIN_WEAPON_NAME.splash_o_matic,
                        MAIN_WEAPON_NAME.splash_o_matic,
                    )
                    .add_string_choice(MAIN_WEAPON_NAME.bamboozler14, MAIN_WEAPON_NAME.bamboozler14)
                    .add_string_choice(
                        MAIN_WEAPON_NAME.splattershot_jr,
                        MAIN_WEAPON_NAME.splattershot_jr,
                    )
            })
    }
}

fn to_display_string(main_weapon: MainWeapon) -> String {
    main_weapon
        .main_power_up_specs
        .iter()
        .map(|spec| spec.to_string())
        .map(|spec_str| format!("• {}", spec_str))
        .join("\n")
}
