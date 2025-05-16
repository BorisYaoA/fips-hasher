use std::time::Instant;
use crate::hash;

pub fn run() {
    let start = Instant::now();
    let _ = hash::hash_path(".", None, &[], false);
    println!("Benchmark completed in: {:?}", start.elapsed());
}