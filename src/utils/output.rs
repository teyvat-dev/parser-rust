use std::fs::File;
use serde_json;
use serde::Serialize;

pub fn save<T: Serialize>(json: T, filename: &str) {
    serde_json::to_writer(&File::create(format!("output/{}.json", filename)).expect("Could not create output file"), &json).expect("Failed to serialize output file");
}