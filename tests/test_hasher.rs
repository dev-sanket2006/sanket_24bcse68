// tests/test_hasher.rs

use intelligent_deduplicator::hasher::{compute_best_hash, compute_sha256};
use std::path::Path;

#[test]
fn test_sha256_hashing() {
    let hash = compute_sha256(Path::new("./sample_files/a.txt")).unwrap();
    assert!(!hash.is_empty());
}

#[test]
fn test_best_hash() {
    let hash = compute_best_hash(Path::new("./sample_files/a.txt")).unwrap();
    assert!(!hash.is_empty());
}
