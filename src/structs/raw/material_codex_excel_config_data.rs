use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub id: usize,
    pub material_id: usize,
    pub sort_order: usize,
    pub name_text_map_hash: usize,
    pub desc_text_map_hash: usize,
    pub icon: String,
}
