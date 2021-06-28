use crate::structs::raw::common::AddProp;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CostItem {
    pub id: Option<usize>,
    pub count: Option<usize>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub weapon_promote_id: usize,
    pub promote_level: Option<usize>,
    pub coin_cost: Option<usize>,
    pub cost_items: Vec<CostItem>,
    pub add_props: Vec<AddProp>,
    pub unlock_max_level: usize,
    pub required_player_level: Option<usize>,
}
