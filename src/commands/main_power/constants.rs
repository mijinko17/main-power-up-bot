use super::domain::{MainPowerUpType, MainWeapon, MainWeaponType};

pub const MAIN_POWER_UP_COMMAND_NAME: &str = "main_power";

pub struct MainWeaponName {
    pub n_zap: &'static str,
    pub splat_charger: &'static str,
    pub splash_o_matic: &'static str,
    pub bamboozler14: &'static str,
    pub splattershot_jr: &'static str,
}

pub const MAIN_WEAPON_NAME: MainWeaponName = MainWeaponName {
    splat_charger: "スプラチャージャー",
    n_zap: "N-ZAP",
    splash_o_matic: "シャープマーカー",
    bamboozler14: "14式竹筒銃",
    splattershot_jr: "わかばシューター",
};

pub const MAIN_WEAPONS: [MainWeapon; 6] = [
    MainWeapon {
        main_weapon_type: MainWeaponType::SplooshOMatic,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(5))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplashOMatic,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(31))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::NZap,
        main_power_up_specs: &[MainPowerUpType::Paint, MainPowerUpType::StandRecoil],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplatCharger,
        main_power_up_specs: &[MainPowerUpType::PowerUp(None)],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::Bamboozler14,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(44))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplattershotJr,
        main_power_up_specs: &[MainPowerUpType::Paint],
    },
];
