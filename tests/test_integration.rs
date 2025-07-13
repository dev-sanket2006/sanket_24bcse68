// tests/test_integration.rs

use intelligent_deduplicator::hasher::compute_best_hash;
use walkdir::WalkDir;

#[test]
fn integration_deduplication_flow() {
    let entries: Vec<_> = WalkDir::new("./sample_files")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    for entry in entries {
        let hash = compute_best_hash(entry.path()).unwrap();
        assert!(!hash.is_empty());
    }
}
