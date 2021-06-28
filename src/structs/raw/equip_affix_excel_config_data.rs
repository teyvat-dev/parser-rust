use crate::structs::raw::common::AddProp;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub affix_id: usize,
    pub id: usize,
    pub level: Option<usize>,
    pub name_text_map_hash: usize,
    pub desc_text_map_hash: usize,
    pub open_config: String, // Can be enum
    pub add_props: Vec<AddProp>,
    pub param_list: Vec<f64>,
    pub show_only_unlocked: Option<bool>,
}
