use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub id: usize,
    pub name: String,
    pub dungeon_list: Vec<usize>,
    pub jump_list: Vec<Value>,
    pub text_list: Vec<usize>,
}
