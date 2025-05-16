use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::hash;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HashReport(pub HashMap<String, String>);

pub struct VerifyResult {
    pub all_matched: bool,
}

pub fn verify_path<P: AsRef<Path>>(path: P, report_path: &Path) -> anyhow::Result<VerifyResult> {
    let content = fs::read_to_string(report_path)?;
    let report: HashReport = serde_json::from_str(&content)?;
    let current = hash::hash_path(path, None, &[], false)?;

    let all_matched = report.0.iter().all(|(k, v)| current.get(k) == Some(v));
    Ok(VerifyResult { all_matched })
}