use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct FileReport {
    pub hash: String,
    pub path: String,
    pub status: String,
}

impl FileReport {
    pub fn new(hash: &str, path: &Path, status: &str) -> Self {
        FileReport {
            hash: hash.to_string(),
            path: path.display().to_string(),
            status: status.to_string(),
        }
    }
}

pub fn save_report_as_json(report: &[FileReport]) {
    let json = serde_json::to_string_pretty(report).unwrap();
    fs::write("report.json", json).unwrap();
}

pub fn save_report_as_html(report: &[FileReport]) {
    let mut html = String::from("<html><head><title>Report</title></head><body>");
    html.push_str("<h1>Deduplication Report</h1><table border='1'><tr><th>Hash</th><th>Path</th><th>Status</th></tr>");

    for entry in report {
        let color = if entry.status == "unique" {
            "green"
        } else {
            "red"
        };
        html.push_str(&format!(
            "<tr style='color:{}'><td>{}</td><td>{}</td><td>{}</td></tr>",
            color, entry.hash, entry.path, entry.status
        ));
    }

    html.push_str("</table></body></html>");
    let mut file = File::create("report.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
}
