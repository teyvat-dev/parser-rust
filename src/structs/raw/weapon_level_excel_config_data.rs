use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub level: usize,
    pub required_exps: Vec<usize>,
}
