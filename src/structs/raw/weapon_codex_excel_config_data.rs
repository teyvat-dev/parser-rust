use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub id: usize,
    pub weapon_id: usize,
    pub sort_order: usize,
    pub show_only_unlocked: Option<bool>,
}
