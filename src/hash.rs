use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex::encode;

type HmacSha256 = Hmac<Sha256>;

pub fn hash_path<P: AsRef<Path>>(path: P, hmac_key: Option<&str>, exclude: &[String], _incremental: bool) -> anyhow::Result<HashMap<String, String>> {
    let mut hashes = HashMap::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path_str = entry.path().to_string_lossy();
            if exclude.iter().any(|x| path_str.contains(x)) {
                continue;
            }

            let content = fs::read(entry.path())?;
            let hash = if let Some(key) = hmac_key {
                let mut mac = HmacSha256::new_from_slice(key.as_bytes())?;
                mac.update(&content);
                encode(mac.finalize().into_bytes())
            } else {
                encode(Sha256::digest(&content))
            };
            hashes.insert(path_str.to_string(), hash);
        }
    }
    Ok(hashes)
}