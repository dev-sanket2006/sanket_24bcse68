// src/filter.rs

use walkdir::DirEntry;
use chrono::{DateTime, Utc};
use std::fs;
use regex::Regex;

// Allowed file extensions
const ALLOWED_EXTENSIONS: [&str; 6] = ["txt", "rs", "jpg", "png", "pdf", "docx"];

/// Minimum and maximum file size in bytes (e.g., 1 KB to 10 MB)
const MIN_SIZE: u64 = 1;         // 1 KB
const MAX_SIZE: u64 = 10_000_000;    // 10 MB

/// Optional regex pattern to match filenames
const FILE_NAME_PATTERN: &str = r".*"; // match all

pub fn is_valid_file(entry: &DirEntry) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }

    let path = entry.path();

    // Extension filtering
    let ext_valid = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ALLOWED_EXTENSIONS.contains(&ext))
        .unwrap_or(false);

    if !ext_valid {
        return false;
    }

    // Size filtering
    if let Ok(metadata) = fs::metadata(path) {
        let file_size = metadata.len();
        if file_size < MIN_SIZE || file_size > MAX_SIZE {
            return false;
        }

        // Date filtering (optional, currently allowing all)
        if let Ok(modified) = metadata.modified() {
            let datetime: DateTime<Utc> = modified.into();
            let _ts = datetime.timestamp(); // You can filter based on timestamp here if needed
        }
    } else {
        return false;
    }

    // Filename pattern filtering
    let regex = Regex::new(FILE_NAME_PATTERN).unwrap();
    if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
        return regex.is_match(file_name);
    }

    false
}
