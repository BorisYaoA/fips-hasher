use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
pub struct HashReport(pub HashMap<String, String>);

pub fn save_report<P: AsRef<Path>>(path: P, data: &HashMap<String, String>) -> anyhow::Result<()> {
    let report = HashReport(data.clone());
    let json = serde_json::to_string_pretty(&report)?;
    fs::write(path, json)?;
    Ok(())
}