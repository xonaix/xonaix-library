//! Manifest generation module.
//!
//! Generates deterministic SHA-256 manifests for governance and units.

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

/// Errors that can occur during manifest generation.
#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid arguments: {0}")]
    InvalidArgs(String),

    #[error("Cannot find specs/ directory in {0}")]
    NoSpecsDir(PathBuf),

    #[error("Manifest drift detected: {0}")]
    ManifestDrift(String),
}

/// File entry in the manifest.
#[derive(Debug, Serialize)]
struct FileEntry {
    path: String,
    sha256: String,
    size: u64,
    #[serde(rename = "type")]
    file_type: String,
}

/// Complete manifest structure.
#[derive(Debug, Serialize)]
struct Manifest {
    baseline: String,
    domain: String,
    file_count: usize,
    files: Vec<FileEntry>,
    generated_at: String,
    generator: String,
    manifest_version: String,
}

/// Compute SHA-256 hash of a file using chunked reading.
fn compute_file_sha256(path: &Path) -> Result<String, ManifestError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];
    let mut hasher = Sha256::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Check if a file should be excluded from the manifest.
fn should_exclude(path: &Path, is_governance: bool) -> bool {
    let path_str = path.to_string_lossy();

    // Exclude .git artifacts
    if path_str.contains(".git") {
        return true;
    }

    // Exclude _reference/ directories
    if path_str.contains("_reference/") || path_str.contains("_reference\\") {
        return true;
    }

    // Exclude common non-content files
    if let Some(name) = path.file_name() {
        let name_str = name.to_string_lossy();
        if name_str == ".gitignore" || name_str == ".gitattributes" || name_str == "CODEOWNERS" {
            return true;
        }
    }

    // For governance manifest: exclude the manifests/ subdirectory to avoid self-reference
    if is_governance && (path_str.contains("manifests/") || path_str.contains("manifests\\")) {
        return true;
    }

    false
}

/// Get file info for a single file.
fn get_file_info(filepath: &Path, base_path: &Path) -> Result<FileEntry, ManifestError> {
    let metadata = fs::metadata(filepath)?;
    let relative = filepath
        .strip_prefix(base_path)
        .unwrap_or(filepath)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(FileEntry {
        path: relative,
        sha256: compute_file_sha256(filepath)?,
        size: metadata.len(),
        file_type: "file".to_string(),
    })
}

/// Generate manifest for governance.
fn generate_governance_manifest(repo_root: &Path) -> Result<Manifest, ManifestError> {
    let target_path = repo_root.join("specs").join("_governance");

    if !target_path.exists() {
        return Err(ManifestError::PathNotFound(target_path));
    }

    let mut files: Vec<FileEntry> = Vec::new();

    for entry in WalkDir::new(&target_path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && !should_exclude(path, true) {
            files.push(get_file_info(path, repo_root)?);
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));

    let now: DateTime<Utc> = Utc::now();

    Ok(Manifest {
        manifest_version: "2.0.0".to_string(),
        domain: "_governance".to_string(),
        baseline: "global".to_string(),
        generated_at: now.format("%Y-%m-%dT%H:%M:%S%.6f+00:00").to_string(),
        generator: "tools/xonaix-library-tools".to_string(),
        file_count: files.len(),
        files,
    })
}

/// Generate manifest for a unit.
fn generate_unit_manifest(unit_path: &Path, repo_root: &Path) -> Result<Manifest, ManifestError> {
    if !unit_path.exists() {
        return Err(ManifestError::PathNotFound(unit_path.to_path_buf()));
    }

    let mut files: Vec<FileEntry> = Vec::new();

    for entry in WalkDir::new(unit_path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        // Exclude the manifests subdirectory
        let path_str = path.to_string_lossy();
        if path_str.contains("manifests/") || path_str.contains("manifests\\") {
            continue;
        }
        if path.is_file() && !should_exclude(path, false) {
            files.push(get_file_info(path, repo_root)?);
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));

    let now: DateTime<Utc> = Utc::now();

    // Extract unit_id from path
    let relative = unit_path
        .strip_prefix(repo_root)
        .unwrap_or(unit_path)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(Manifest {
        manifest_version: "2.0.0".to_string(),
        domain: "unit".to_string(),
        baseline: relative,
        generated_at: now.format("%Y-%m-%dT%H:%M:%S%.6f+00:00").to_string(),
        generator: "tools/xonaix-library-tools".to_string(),
        file_count: files.len(),
        files,
    })
}

/// Find repository root by looking for specs/ directory.
fn find_repo_root() -> Result<PathBuf, ManifestError> {
    let mut current = std::env::current_dir()?;

    if current.join("specs").exists() {
        return Ok(current);
    }

    while let Some(parent) = current.parent() {
        if parent.join("specs").exists() {
            return Ok(parent.to_path_buf());
        }
        current = parent.to_path_buf();
    }

    Err(ManifestError::NoSpecsDir(std::env::current_dir()?))
}

/// Main entry point for manifest generation.
pub fn run(
    governance: bool,
    unit: Option<String>,
    output: Option<String>,
    repo_root_arg: Option<String>,
    check: bool,
) -> Result<(), ManifestError> {
    let repo_root = match repo_root_arg {
        Some(path) => PathBuf::from(path),
        None => find_repo_root()?,
    };

    if !repo_root.join("specs").exists() {
        return Err(ManifestError::NoSpecsDir(repo_root));
    }

    let (manifest, output_name) = if governance {
        let m = generate_governance_manifest(&repo_root)?;
        (m, "MANIFEST_governance.sha256.json".to_string())
    } else if let Some(unit_id) = unit {
        // Find unit path from registry
        let registry_path = repo_root.join("specs").join("_governance").join("UNIT_REGISTRY.json");
        let registry_content = fs::read_to_string(&registry_path)?;
        let registry: serde_json::Value = serde_json::from_str(&registry_content)?;

        let unit_entry = registry["units"][&unit_id].as_object()
            .ok_or_else(|| ManifestError::InvalidArgs(format!("Unit not found: {}", unit_id)))?;

        let unit_path_str = unit_entry["path"].as_str()
            .ok_or_else(|| ManifestError::InvalidArgs("Unit has no path".to_string()))?;

        let unit_path = repo_root.join(unit_path_str);
        let m = generate_unit_manifest(&unit_path, &repo_root)?;
        let safe_name = unit_id.replace('/', "_");
        (m, format!("UNIT_MANIFEST_{}.sha256.json", safe_name))
    } else {
        return Err(ManifestError::InvalidArgs(
            "Either --governance or --unit is required".to_string(),
        ));
    };

    let output_path = match output {
        Some(path) => PathBuf::from(path),
        None => {
            let manifests_dir = repo_root.join("specs").join("_governance").join("manifests");
            if !manifests_dir.exists() {
                fs::create_dir_all(&manifests_dir)?;
            }
            manifests_dir.join(&output_name)
        }
    };

    let json = serde_json::to_string_pretty(&manifest)?;
    let new_content = format!("{}\n", json);

    if check {
        if !output_path.exists() {
            return Err(ManifestError::ManifestDrift(format!(
                "Manifest file does not exist: {}",
                output_path.display()
            )));
        }

        let existing_content = fs::read_to_string(&output_path)?;

        fn strip_generated_at(s: &str) -> String {
            s.lines()
                .filter(|line| !line.trim_start().starts_with("\"generated_at\""))
                .collect::<Vec<_>>()
                .join("\n")
        }

        let new_stripped = strip_generated_at(&new_content);
        let existing_stripped = strip_generated_at(&existing_content);

        if new_stripped != existing_stripped {
            println!("FAIL: Manifest drift detected!");
            println!("Path: {}", output_path.display());
            return Err(ManifestError::ManifestDrift(
                output_path.display().to_string()
            ));
        }

        println!("PASS: Manifest up-to-date: {}", output_path.display());
        println!("Files: {}", manifest.file_count);
        return Ok(());
    }

    let mut file = File::create(&output_path)?;
    file.write_all(new_content.as_bytes())?;

    println!("Generated: {}", output_path.display());
    println!("Files: {}", manifest.file_count);

    Ok(())
}
