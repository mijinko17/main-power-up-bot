use crate::commands::main_power::constants::MAIN_WEAPONS;

use super::constants::MainWeaponType;

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
