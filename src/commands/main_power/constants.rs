use super::domain::{MainPowerUpType, MainWeapon, MainWeaponType};

pub const MAIN_POWER_UP_COMMAND_NAME: &str = "main_power";

pub struct MainWeaponName {
    // Shooter
    pub sploosh_o_matic: &'static str,
    pub splattershot_jr: &'static str,
    pub splash_o_matic: &'static str,
    pub aerospray: &'static str,
    pub splattershot: &'static str,
    pub dot_52_gal: &'static str,
    pub n_zap: &'static str,
    pub splattershot_pro: &'static str,
    pub dot_96_gal: &'static str,
    pub jet_squelcher: &'static str,
    pub l_3_nozzlenose: &'static str,
    pub h_3_nozzlenose: &'static str,
    pub squeezer: &'static str,
    //Charger
    pub splat_charger: &'static str,
    pub bamboozler14: &'static str,
}

pub const MAIN_WEAPON_NAME: MainWeaponName = MainWeaponName {
    sploosh_o_matic: "ボールドマーカー",
    splattershot_jr: "わかばシューター",
    splash_o_matic: "シャープマーカー",
    aerospray: "プロモデラー",
    splattershot: "スプラシューター",
    dot_52_gal: ".52ガロン",
    n_zap: "N-ZAP",
    splattershot_pro: "プライムシューター",
    dot_96_gal: ".96ガロン",
    jet_squelcher: "ジェットスイーパー",
    l_3_nozzlenose: "L3リールガン",
    h_3_nozzlenose: "H3リールガン",
    squeezer: "ボトルガイザー",
    splat_charger: "スプラチャージャー",
    bamboozler14: "14式竹筒銃",
};

pub const MAIN_WEAPONS: [MainWeapon; 15] = [
    MainWeapon {
        main_weapon_type: MainWeaponType::SplooshOMatic,
        main_power_up_specs: &[MainPowerUpType::PowerUp(None)],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplattershotJr,
        main_power_up_specs: &[MainPowerUpType::Paint],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplashOMatic,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(31))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::Aerospray,
        main_power_up_specs: &[MainPowerUpType::Paint],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::Splattershot,
        main_power_up_specs: &[MainPowerUpType::StandRecoil, MainPowerUpType::JumpRecoil],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::Dot52Gal,
        main_power_up_specs: &[MainPowerUpType::StandRecoil, MainPowerUpType::JumpRecoil],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::NZap,
        main_power_up_specs: &[MainPowerUpType::Paint, MainPowerUpType::StandRecoil],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplattershotPro,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(42))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::Dot96Gal,
        main_power_up_specs: &[MainPowerUpType::PowerUp(None)],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::JetSquelcher,
        main_power_up_specs: &[
            MainPowerUpType::ShootRange,
            MainPowerUpType::BulletSpeed,
            MainPowerUpType::JumpRecoil,
        ],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::L3Nozzlenose,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(24))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::H3Nozzlenose,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(42))],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::Squeezer,
        main_power_up_specs: &[MainPowerUpType::PowerUp(None)],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::SplatCharger,
        main_power_up_specs: &[MainPowerUpType::PowerUp(None)],
    },
    MainWeapon {
        main_weapon_type: MainWeaponType::Bamboozler14,
        main_power_up_specs: &[MainPowerUpType::PowerUp(Some(44))],
    },
];
