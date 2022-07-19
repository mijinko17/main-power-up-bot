use itertools::Itertools;

use super::constants::{MAIN_WEAPONS, MAIN_WEAPON_NAME};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainWeaponType {
    SplooshOMatic,
    SplattershotJr,
    SplashOMatic,
    Aerospray,
    Splattershot,
    Dot52Gal,
    NZap,
    SplatCharger,
    Bamboozler14,
    // TODO: Add weapon.
}

impl MainWeaponType {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "N-ZAP" => Some(MainWeaponType::NZap),
            "スプラチャージャー" => Some(MainWeaponType::SplatCharger),
            "シャープマーカー" => Some(MainWeaponType::SplashOMatic),
            "14式竹筒銃" => Some(MainWeaponType::Bamboozler14),
            "わかばシューター" => Some(MainWeaponType::SplattershotJr),
            _ => None,
        }
    }
}

impl ToString for MainWeaponType {
    fn to_string(&self) -> String {
        let result = match self {
            Self::SplooshOMatic => "ボールドマーカー",
            Self::NZap => MAIN_WEAPON_NAME.n_zap,
            Self::SplatCharger => MAIN_WEAPON_NAME.splat_charger,
            Self::SplashOMatic => MAIN_WEAPON_NAME.splash_o_matic,
            Self::Bamboozler14 => MAIN_WEAPON_NAME.bamboozler14,
            Self::SplattershotJr => MAIN_WEAPON_NAME.splattershot_jr,
            _ => "not implemented yet",
        };
        result.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainPowerUpType {
    PowerUp(Option<u32>),
    Paint,
    StandRecoil,
}

impl ToString for MainPowerUpType {
    fn to_string(&self) -> String {
        match self {
            Self::PowerUp(gear_power) => match gear_power {
                Some(power) => {
                    let gijikaku = [0, 1, 2, 3]
                        .iter()
                        .map(|main_gear| {
                            let remain: i32 = *power as i32 - main_gear * 10;
                            if remain < 0 {
                                None
                            } else {
                                Some((*main_gear, (remain + 2) / 3))
                            }
                        })
                        .filter_map(|xx| {
                            if let Some((_, sub)) = xx {
                                if sub <= 9 {
                                    xx
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .map(|(main, sub)| format!("{}.{}", main, sub))
                        .join(" / ");
                    format!("威力アップ ( 疑似確: {} )", gijikaku)
                }
                None => "威力アップ".to_string(),
            },
            Self::Paint => "塗り強化".to_string(),
            Self::StandRecoil => "立ち撃ち時のブレ軽減".to_string(),
            _ => "not implemented yet".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MainWeapon {
    pub main_weapon_type: MainWeaponType,
    pub main_power_up_specs: &'static [MainPowerUpType],
}

impl From<MainWeaponType> for MainWeapon {
    fn from(value: MainWeaponType) -> Self {
        MAIN_WEAPONS
            .iter()
            .find(|weapon| weapon.main_weapon_type.eq(&value))
            .unwrap()
            .clone()
    }
}
