// tests/test_filter.rs

use intelligent_deduplicator::filter::is_valid_file;
use walkdir::DirEntry;
use std::fs::File;
use std::path::Path;

fn fake_file_entry(path: &str) -> DirEntry {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .find(|e| e.file_type().is_file())
        .expect("File not found")
}

#[test]
fn test_is_valid_file_txt() {
    let entry = fake_file_entry("./sample_files/a.txt");
    assert!(is_valid_file(&entry));
}
