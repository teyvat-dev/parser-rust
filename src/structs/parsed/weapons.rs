use crate::structs::raw::common::AddProp;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackLevel {
    pub level: usize,
    pub value: f64,
    pub upgraded: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubStatLevel {
    pub level: usize,
    pub value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PassiveLevel {
    pub level: usize,
    pub description: String,
    pub additional_properties: Vec<AddProp>,
    pub params: Vec<f64>,
    pub extra_id: usize,
    pub extra_config: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    pub name: String,
    pub levels: Vec<PassiveLevel>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub r#type: String,
    pub name: String,
    pub description: String,
    pub passive: Option<Passive>,
    pub lore: String,
    pub rarity: usize,
    pub attack_levels: Vec<AttackLevel>,
    pub sub_stat_levels: Vec<SubStatLevel>,
    pub sub_stat_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AscensionRecipe {
    pub name: String,
    pub count: usize,
    pub game_id: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ascension {
    pub level: usize,
    pub recipe: Vec<AscensionRecipe>,
    pub cost: usize,
    pub max_level: usize,
    pub required_player_level: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub weapon_id: usize,
    pub weapon: Weapon,
    pub ascensions: Vec<Ascension>,
}
