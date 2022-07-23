use itertools::Itertools;
use serenity::{
    client::Context,
    model::interactions::{
        application_command::{
            ApplicationCommand, ApplicationCommandInteraction,
            ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
        },
        InteractionResponseType,
    },
};

use crate::commands::main_power::constants::MAIN_WEAPONS;

use super::{
    constants::{MAIN_POWER_UP_COMMAND_NAME, MAIN_WEAPON_NAME},
    domain::{MainWeapon, MainWeaponType},
};

pub async fn register_main_power_up_command(
    ctx: &Context,
) -> Result<ApplicationCommand, serenity::Error> {
    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command
            .name(MAIN_POWER_UP_COMMAND_NAME)
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
    })
    .await
}

pub fn main_power_up_response(main_weapon_type: MainWeaponType) -> String {
    let weapon = MAIN_WEAPONS
        .into_iter()
        .find(|weapon| weapon.main_weapon_type.eq(&main_weapon_type));
    if let Some(target_weapon) = weapon {
        target_weapon
            .main_power_up_specs
            .iter()
            .fold(target_weapon.main_weapon_type.to_string(), |acc, cur| {
                acc + "\n" + &cur.to_string()
            })
    } else {
        "not found".to_string()
    }
}

pub fn to_display_string(main_weapon: MainWeapon) -> String {
    main_weapon
        .main_power_up_specs
        .iter()
        .map(|spec| spec.to_string())
        .map(|spec_str| format!("• {}", spec_str))
        .join("\n")
}

pub async fn interact_main_power_up_command(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) {
    let selected_weapon = match command.data.name.as_str() {
        MAIN_POWER_UP_COMMAND_NAME => {
            let options = command
                .data
                .options
                .get(0)
                .expect("error")
                .resolved
                .as_ref()
                .expect("error");
            if let ApplicationCommandInteractionDataOptionValue::String(str) = options {
                MainWeaponType::from_str(str).map(MainWeapon::from)
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(weapon) = selected_weapon {
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|emb| {
                            emb.title(weapon.main_weapon_type.to_string())
                                .description(to_display_string(weapon))
                                .colour(serenity::utils::Colour::MAGENTA)
                        })
                    })
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        }
    }
}
