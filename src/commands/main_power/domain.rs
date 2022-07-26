use std::str::FromStr;

use itertools::Itertools;

use super::constants::{MAIN_WEAPONS, MAIN_WEAPON_NAME};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainWeaponType {
    // Shooter
    SplooshOMatic,
    SplattershotJr,
    SplashOMatic,
    Aerospray,
    Splattershot,
    Dot52Gal,
    NZap,
    SplattershotPro,
    Dot96Gal,
    JetSquelcher,
    L3Nozzlenose,
    H3Nozzlenose,
    Squeezer,
    // Charger
    SplatCharger,
    Bamboozler14,
    // TODO: Add weapon.
}

impl FromStr for MainWeaponType {
    type Err = ();

    fn from_str(value: &str) -> Result<MainWeaponType, Self::Err> {
        MAIN_WEAPONS
            .iter()
            .map(|weapon| weapon.main_weapon_type.clone())
            .find(|weapon_type| weapon_type.to_string().eq(&value))
            .ok_or(())
    }
}

impl ToString for MainWeaponType {
    fn to_string(&self) -> String {
        let result = match self {
            Self::SplooshOMatic => MAIN_WEAPON_NAME.sploosh_o_matic,
            Self::SplattershotJr => MAIN_WEAPON_NAME.splattershot_jr,
            Self::SplashOMatic => MAIN_WEAPON_NAME.splash_o_matic,
            Self::Aerospray => MAIN_WEAPON_NAME.aerospray,
            Self::Splattershot => MAIN_WEAPON_NAME.splattershot,
            Self::Dot52Gal => MAIN_WEAPON_NAME.dot_52_gal,
            Self::NZap => MAIN_WEAPON_NAME.n_zap,
            Self::SplattershotPro => MAIN_WEAPON_NAME.splattershot_pro,
            Self::Dot96Gal => MAIN_WEAPON_NAME.dot_96_gal,
            Self::JetSquelcher => MAIN_WEAPON_NAME.jet_squelcher,
            Self::L3Nozzlenose => MAIN_WEAPON_NAME.l_3_nozzlenose,
            Self::H3Nozzlenose => MAIN_WEAPON_NAME.h_3_nozzlenose,
            Self::Squeezer => MAIN_WEAPON_NAME.squeezer,
            Self::SplatCharger => MAIN_WEAPON_NAME.splat_charger,
            Self::Bamboozler14 => MAIN_WEAPON_NAME.bamboozler14,
        };
        result.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainPowerUpType {
    PowerUp(Option<u32>),
    Paint,
    StandRecoil,
    JumpRecoil,
    ShootRange,
    BulletSpeed,
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
            Self::JumpRecoil => "ジャンプ時のブレ軽減".to_string(),
            Self::ShootRange => "射程延長".to_string(),
            Self::BulletSpeed => "弾速アップ".to_string(),
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
