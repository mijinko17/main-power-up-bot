use super::domain::{MainPowerUpType, MainWeapon, MainWeaponType};

pub const MAIN_POWER_UP_COMMAND_NAME: &str = "main_power";

pub struct MainWeaponName {
    pub n_zap: &'static str,
    pub splat_charger: &'static str,
}

pub const MAIN_WEAPON_NAME: MainWeaponName = MainWeaponName {
    splat_charger: "スプラチャージャー",
    n_zap: "N-ZAP",
};

pub const MAIN_WEAPONS: [MainWeapon; 3] = [
    MainWeapon {
        main_weapon_type: MainWeaponType::SplooshOMatic,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(5))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::NZap,
        main_power_up_specs: &[MainPowerUpType::Paint, MainPowerUpType::StandRecoil],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplatCharger,
        main_power_up_specs: &[MainPowerUpType::PowerUp(None)],
    },
];
