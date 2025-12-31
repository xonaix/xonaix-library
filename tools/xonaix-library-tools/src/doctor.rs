//! Environment doctor module.
//!
//! Performs comprehensive environment verification for the Library.

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

/// Errors during doctor checks.
#[derive(Debug, Error)]
pub enum DoctorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cannot find specs/ directory")]
    NoSpecsDir,

    #[error("Doctor checks failed: {0} check(s) failed")]
    DoctorFailed(u32),
}

/// UNIT_REGISTRY.json structure
#[derive(Debug, Deserialize)]
struct UnitRegistry {
    #[allow(dead_code)]
    registry_version: String,
    #[allow(dead_code)]
    description: String,
    units: std::collections::HashMap<String, RegistryEntry>,
}

#[derive(Debug, Deserialize)]
struct RegistryEntry {
    path: String,
    #[allow(dead_code)]
    domain: String,
    #[allow(dead_code)]
    status: String,
    #[allow(dead_code)]
    description: String,
}

/// Find repository root.
fn find_repo_root() -> Result<PathBuf, DoctorError> {
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

    Err(DoctorError::NoSpecsDir)
}

/// Run all doctor checks.
pub fn run(repo_root_arg: Option<String>) -> Result<(), DoctorError> {
    let repo_root = match repo_root_arg {
        Some(path) => PathBuf::from(path),
        None => find_repo_root()?,
    };

    let specs_dir = repo_root.join("specs");
    let governance_dir = specs_dir.join("_governance");

    println!("=== XONAIX LIBRARY ENVIRONMENT DOCTOR ===");
    println!("Repository: {}", repo_root.display());
    println!();

    let mut failed_checks = 0u32;

    // Check 1: Required governance files exist
    println!("[1/5] Checking required governance files...");
    let required_files = [
        "AUDIT_CONTRACT.md",
        "NO_DEBT_RULES.md",
        "DISTRIBUTION_EXCLUSIONS.md",
        "LIBRARY_SEALING_CONTRACT.md",
        "LIBRARY_STANDARD_HEADER_CONTRACT.md",
        "XONAIX_SELF_GOVERNANCE_CONTRACT.md",
        "UNIT_REGISTRY.json",
    ];

    let mut missing_files: Vec<String> = Vec::new();
    for file in &required_files {
        let file_path = governance_dir.join(file);
        if !file_path.exists() {
            missing_files.push(file.to_string());
        }
    }

    if !missing_files.is_empty() {
        for f in &missing_files {
            println!("  Missing: {}", f);
        }
        println!("FAIL: Required governance files missing");
        failed_checks += 1;
    } else {
        println!("PASS: All required governance files exist");
    }

    // Check 2: UNIT_REGISTRY.json is valid
    println!();
    println!("[2/5] Validating UNIT_REGISTRY.json...");
    let registry_path = governance_dir.join("UNIT_REGISTRY.json");
    let registry: Option<UnitRegistry> = if registry_path.exists() {
        match fs::read_to_string(&registry_path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(reg) => {
                    println!("PASS: UNIT_REGISTRY.json is valid");
                    Some(reg)
                }
                Err(e) => {
                    println!("FAIL: UNIT_REGISTRY.json parse error: {}", e);
                    failed_checks += 1;
                    None
                }
            },
            Err(e) => {
                println!("FAIL: Cannot read UNIT_REGISTRY.json: {}", e);
                failed_checks += 1;
                None
            }
        }
    } else {
        println!("FAIL: UNIT_REGISTRY.json does not exist");
        failed_checks += 1;
        None
    };

    // Check 3: Required directories exist
    println!();
    println!("[3/5] Checking required directories...");
    let required_dirs = ["standards", "meta"];
    let mut missing_dirs: Vec<String> = Vec::new();

    for dir in &required_dirs {
        let dir_path = specs_dir.join(dir);
        if !dir_path.exists() {
            missing_dirs.push(dir.to_string());
        }
    }

    if missing_dirs.is_empty() {
        println!("PASS: All required directories exist (standards, meta)");
    } else {
        for d in &missing_dirs {
            println!("  Missing: specs/{}", d);
        }
        println!("FAIL: Required directories missing");
        failed_checks += 1;
    }

    // Check 4: Unit paths exist
    println!();
    println!("[4/5] Checking unit paths...");
    if let Some(ref reg) = registry {
        let mut unit_errors: Vec<String> = Vec::new();
        for (unit_id, entry) in &reg.units {
            let unit_path = repo_root.join(&entry.path);
            if !unit_path.exists() {
                unit_errors.push(format!("{}: Path does not exist: {}", unit_id, entry.path));
            }
        }

        if unit_errors.is_empty() {
            println!("PASS: All {} unit paths exist", reg.units.len());
        } else {
            for err in &unit_errors {
                println!("  FAIL: {}", err);
            }
            failed_checks += 1;
        }
    } else {
        println!("SKIP: Cannot check unit paths (UNIT_REGISTRY.json invalid)");
        failed_checks += 1;
    }

    // Check 5: Hash computation is functional
    println!();
    println!("[5/5] Verifying hash computation...");
    let test_data = b"xonaix-library-tools doctor test";
    let mut hasher = Sha256::new();
    hasher.update(test_data);
    let result = hasher.finalize();
    let computed_hash = format!("{:x}", result);

    if computed_hash.len() == 64 {
        println!("PASS: Hash computation functional (SHA-256)");
    } else {
        println!("FAIL: Hash computation returned unexpected result");
        failed_checks += 1;
    }

    println!();
    println!("=== DOCTOR COMPLETE ===");
    if failed_checks == 0 {
        println!("Result: ALL CHECKS PASSED");
        println!("Environment is ready for Xonaix Library development.");
        Ok(())
    } else {
        println!("Result: FAILED - {} check(s) failed", failed_checks);
        Err(DoctorError::DoctorFailed(failed_checks))
    }
}
