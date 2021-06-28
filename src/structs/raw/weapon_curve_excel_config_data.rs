use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurveInfo {
    pub r#type: String,
    pub arith: String,
    pub value: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub level: usize,
    pub curve_infos: Vec<CurveInfo>,
}
