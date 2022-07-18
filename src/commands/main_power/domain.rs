use super::constants::MAIN_WEAPON_NAME;

#[derive(Debug, PartialEq, Eq)]
pub enum MainWeaponType {
    SplooshOMatic,
    SplattershotJr,
    SplashOMatic,
    Aerospray,
    Splattershot,
    Dot52Gal,
    NZap,
    SplatCharger,
    // TODO: Add weapon.
}

impl MainWeaponType {
    pub fn from_str(value: &str) -> Option<Self> {
        let a = MAIN_WEAPON_NAME.n_zap;
        match value {
            "N-ZAP" => Some(MainWeaponType::NZap),
            "スプラチャージャー" => Some(MainWeaponType::SplatCharger),
            _ => None,
        }
    }
}

impl ToString for MainWeaponType {
    fn to_string(&self) -> String {
        let result = match self {
            Self::SplashOMatic => "ボールドマーカー",
            Self::NZap => "N-ZAP",
            Self::SplatCharger => "スプラチャージャー",
            _ => "not implemented yet",
        };
        result.to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MainPowerUpType {
    PowerUp(Option<u32>),
    Paint,
    StandRecoil,
}

impl ToString for MainPowerUpType {
    fn to_string(&self) -> String {
        let result = match self {
            Self::PowerUp(_) => "威力アップ",
            Self::Paint => "塗り強化",
            Self::StandRecoil => "立ち撃ち時のブレ軽減",
            _ => "not implemented yet",
        };
        result.to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MainWeapon {
    pub main_weapon_type: MainWeaponType,
    pub main_power_up_specs: &'static [MainPowerUpType],
}
