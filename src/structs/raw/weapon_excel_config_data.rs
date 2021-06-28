use crate::structs::raw::common::SubStatPropType;
use crate::structs::raw::weapon_shared::WeaponType;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeaponProp {
    pub prop_type: Option<SubStatPropType>,
    pub init_value: Option<f64>,
    pub r#type: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub weapon_type: WeaponType,
    pub rank_level: usize,
    pub weapon_base_exp: usize,
    pub skill_affix: Vec<usize>,
    pub weapon_prop: Vec<WeaponProp>,
    pub awaken_texture: String,
    pub awaken_light_map_texture: String,
    pub awaken_icon: String,
    pub weapon_promote_id: usize,
    pub story_id: Option<usize>,
    pub awaken_costs: Vec<usize>,
    pub gacha_card_name_hash_suffix: usize,
    pub gacha_card_name_hash_pre: usize,
    pub destroy_rule: Option<String>,
    pub destroy_return_material: Vec<usize>,
    pub destroy_return_material_count: Vec<usize>,
    pub id: usize,
    pub name_text_map_hash: usize,
    pub desc_text_map_hash: usize,
    pub icon: String,
    pub item_type: String,
    // Duplicate identifier breaks parsing
    // pub weight: usize,
    pub rank: usize,
    pub gadget_id: usize,
}
