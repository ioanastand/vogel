use crate::models::PortInfo;
use std::fs;

pub fn export(data: &[PortInfo]) {

    let json = serde_json::to_string_pretty(data).unwrap();

    fs::write("data/scan_result.json", json).unwrap();
}
