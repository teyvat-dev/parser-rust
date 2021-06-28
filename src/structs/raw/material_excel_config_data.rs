use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UseParam {
    pub use_op: Option<String>,
    pub use_param: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub interaction_title_text_map_hash: usize,
    pub no_first_get_hint: Option<bool>,
    pub material_type: Option<String>,
    pub stack_limit: Option<usize>,
    pub max_use_count: Option<usize>,
    pub use_param: Vec<UseParam>,
    pub rank_level: Option<usize>,
    pub effect_desc_text_map_hash: usize,
    pub special_desc_text_map_hash: usize,
    pub type_desc_text_map_hash: usize,
    pub effect_icon: String,
    pub effect_name: String,
    pub pic_path: Vec<Value>,
    pub satination_params: Option<Vec<Value>>,
    pub destroy_return_material: Option<Vec<Value>>,
    pub destory_return_material_count: Option<Vec<Value>>,
    pub id: usize,
    pub name_text_map_hash: usize,
    pub desc_text_map_hash: usize,
    pub icon: String,
    pub item_type: String,
    pub weight: Option<usize>,
    pub rank: Option<usize>,
    pub gadget_id: Option<usize>,
}
