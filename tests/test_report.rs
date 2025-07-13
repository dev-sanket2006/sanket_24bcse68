// tests/test_report.rs

use intelligent_deduplicator::report::{FileReport, save_report_as_html, save_report_as_json};
use std::path::Path;
use std::fs;

#[test]
fn test_report_generation() {
    let report = vec![
        FileReport::new("hash123", Path::new("sample_files/a.txt"), "unique"),
        FileReport::new("hash456", Path::new("sample_files/b.txt"), "duplicate"),
    ];

    save_report_as_json(&report);
    save_report_as_html(&report);

    assert!(Path::new("report.json").exists());
    assert!(Path::new("report.html").exists());

    // Clean up
    fs::remove_file("report.json").unwrap();
    fs::remove_file("report.html").unwrap();
}
