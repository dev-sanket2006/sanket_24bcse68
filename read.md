# 🧠 Intelligent File Deduplicator (Rust)

A powerful and efficient Rust-based CLI tool to detect and quarantine duplicate files using advanced filtering, multiple hashing algorithms, and MongoDB logging. Ideal for organizing your directories and avoiding file clutter.

---

## 📦 Features

- ✅ **Multi-Algorithm Hashing**
  - Uses **SHA-256**, **Blake3**, and **xxHash** for different file types
  - Automatic algorithm selection based on file type (text, binary, image)

- ✅ **Advanced Filtering**
  - Filters files based on:
    - Allowed extensions (`txt`, `rs`, `jpg`, `png`, `pdf`, `docx`)
    - File **size range** (min–max in bytes)
    - File **modified date range**
    - Filename matching with **Regex patterns**

- ✅ **Parallel Processing**
  - Uses **Rayon** to process files concurrently for faster hashing and scanning.

- ✅ **MongoDB Integration**
  - Stores hash, path, and status (`unique` or `duplicate`) in a MongoDB database.

- ✅ **Safe Operations**
  - **Quarantine System**: Moves duplicates to a quarantine folder instead of deleting them.
  - Keeps originals in a clean folder.

- ✅ **Reports**
  - Generates:
    - `report.json`: Machine-readable report
    - `report.html`: User-friendly, color-coded HTML report for easy viewing

---

## 📂 Project Structure

intelligent_deduplicator/
├── src/
│ ├── main.rs
│ ├── filter.rs
│ ├── hasher.rs
│ ├── report.rs
│ └── db.rs
├── sample_files/ # Input directory (source files)
├── clean/ # Output directory for unique files
├── quarantine/ # Output directory for duplicates
├── report.json # JSON report
├── report.html # HTML report
├── tests/ # Unit & integration tests
│ ├── test_filter.rs
│ ├── test_hasher.rs
│ ├── test_report.rs
│ ├── test_db.rs
│ └── test_integration.rs
├── Cargo.toml
└── README.md

---

## 🚀 How to Run

### ✅ Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://www.mongodb.com/try/download/community) running locally at `localhost:27017`

### ✅ Clone and Run

```bash
git clone https://github.com/your-username/intelligent_deduplicator.git
cd intelligent_deduplicator
cargo run
