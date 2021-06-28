use serde::Deserialize;

impl ToString for WeaponType {
    fn to_string(&self) -> String {
        match self {
            WeaponType::WeaponBow => {
                "Bow".to_owned()
            }
            WeaponType::WeaponCatalyst => {
                "Catalyst".to_owned()
            }
            WeaponType::WeaponClaymore => {
                "Claymore".to_owned()
            }
            WeaponType::WeaponPole => {
                "Polearm".to_owned()
            }
            WeaponType::WeaponSwordOneHand => {
                "Sword".to_owned()
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeaponType {
    WeaponBow,
    WeaponCatalyst,
    WeaponClaymore,
    WeaponPole,
    WeaponSwordOneHand,
}