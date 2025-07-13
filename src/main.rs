mod filter;
mod hasher;
mod report;
mod db;

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use tokio;
use mongodb::{Client, bson::{doc, Document}};
use serde::Serialize;
use filter::is_valid_file;
use hasher::compute_best_hash;
use report::{FileReport, save_report_as_json, save_report_as_html};
use db::connect_to_mongo;

const SOURCE_DIR: &str = "./sample_files";
const CLEAN_DIR: &str = "./clean";
const QUARANTINE_DIR: &str = "./quarantine";

#[tokio::main]
async fn main() {
    println!("🚀 Intelligent File Deduplicator Starting...");
    println!("📂 Scanning directory: {}", SOURCE_DIR);

    let files: Vec<PathBuf> = WalkDir::new(SOURCE_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
        .inspect(|e| println!("📄 Found: {:?}", e.path()))
        .filter(|e| is_valid_file(e))
        .map(|e| e.into_path())
        .collect();

    println!("✅ Total valid files found: {}", files.len());

    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for path in &files {
        if let Ok(hash) = compute_best_hash(path) {
            hash_map.entry(hash).or_default().push(path.to_path_buf());
        }
    }

    fs::create_dir_all(CLEAN_DIR).unwrap();
    fs::create_dir_all(QUARANTINE_DIR).unwrap();

    let client = connect_to_mongo().await;
    let db = client.database("file_deduplicator");
    let coll = db.collection::<Document>("file_metadata");

    let mut report_data: Vec<FileReport> = Vec::new();

    for (hash, paths) in hash_map {
        if paths.len() > 1 {
            for path in &paths {
                let dest = Path::new(QUARANTINE_DIR).join(path.file_name().unwrap());
                fs::copy(path, &dest).unwrap();

                let entry = FileReport::new(&hash, path, "duplicate");
                report_data.push(entry.clone());

                let doc = doc! {
                    "hash": &entry.hash,
                    "path": &entry.path,
                    "status": &entry.status
                };
                coll.insert_one(doc, None).await.unwrap();
            }
        } else {
            let path = &paths[0];
            let dest = Path::new(CLEAN_DIR).join(path.file_name().unwrap());
            fs::copy(path, &dest).unwrap();

            let entry = FileReport::new(&hash, path, "unique");
            report_data.push(entry.clone());

            let doc = doc! {
                "hash": &entry.hash,
                "path": &entry.path,
                "status": &entry.status
            };
            coll.insert_one(doc, None).await.unwrap();
        }
    }

    save_report_as_json(&report_data);
    save_report_as_html(&report_data);

    println!("✅ Done. Reports generated.");
}
