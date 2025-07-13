use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use sha2::{Sha256, Digest};
use blake3;
use xxhash_rust::xxh3::Xxh3;

/// Automatically selects the best hashing algorithm based on file extension
pub fn compute_best_hash<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let ext = path.as_ref().extension().and_then(|s| s.to_str()).unwrap_or("");

    match ext {
        "txt" | "rs" | "docx" => compute_sha256(path),
        "pdf" => compute_blake3(path),
        "jpg" | "png" => compute_xxhash(path),
        _ => compute_sha256(path),
    }
}

/// SHA-256 hash for text-based files
pub fn compute_sha256<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut file = BufReader::new(File::open(path)?);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Blake3 hash for PDF files
pub fn compute_blake3<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut file = BufReader::new(File::open(path)?);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0; 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

/// xxHash for images
pub fn compute_xxhash<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut file = BufReader::new(File::open(path)?);
    let mut hasher = Xxh3::new();
    let mut buffer = [0; 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.digest()))
}
