//! Enforcement module for no-debt rules.
//!
//! Checks all files in current-only scope against governance rules.

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

/// Errors during enforcement.
#[derive(Debug, Error)]
pub enum EnforceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Cannot find specs/ directory")]
    NoSpecsDir,

    #[error("Enforcement failed: {0} check(s) failed")]
    EnforcementFailed(u32),
}

/// Find repository root.
fn find_repo_root() -> Result<PathBuf, EnforceError> {
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

    Err(EnforceError::NoSpecsDir)
}

/// Check if path should be excluded from enforcement.
fn should_exclude(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Exclude _deprecated/
    if path_str.contains("_deprecated/") || path_str.contains("_deprecated\\") {
        return true;
    }

    // Exclude _reference/
    if path_str.contains("_reference/") || path_str.contains("_reference\\") {
        return true;
    }

    // Exclude .git/
    if path_str.contains(".git/") || path_str.contains(".git\\") {
        return true;
    }

    // Exclude target/
    if path_str.contains("/target/") || path_str.contains("\\target\\") {
        return true;
    }

    // Exclude manifests/ (generated files)
    if path_str.contains("manifests/") || path_str.contains("manifests\\") {
        return true;
    }

    false
}

/// Check if path is a governance file (excluded from token checks only).
/// Governance files describe rules and necessarily mention forbidden tokens.
fn is_governance_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains("_governance/") || path_str.contains("_governance\\")
}

/// Run all enforcement checks.
pub fn run(repo_root_arg: Option<String>, _current_only: bool) -> Result<(), EnforceError> {
    let repo_root = match repo_root_arg {
        Some(path) => PathBuf::from(path),
        None => find_repo_root()?,
    };

    let specs_dir = repo_root.join("specs");

    println!("=== XONAIX LIBRARY NO-DEBT ENFORCEMENT ===");
    println!("Repository: {}", repo_root.display());
    println!();

    let mut failed_checks = 0u32;

    // Check 1: Forbidden tokens
    println!("[1/8] Checking for forbidden tokens...");
    let forbidden_tokens = ["TODO", "TBD", "FIXME", "CHANGEME", "PLACEHOLDER", "INTENTIONALLY LEFT BLANK"];
    let mut token_violations: Vec<String> = Vec::new();

    for entry in WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_exclude(path) {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext != "md" && ext != "json" {
                continue;
            }
        } else {
            continue;
        }

        // Governance files describe rules and necessarily mention forbidden tokens
        if is_governance_file(path) {
            continue;
        }

        if let Ok(content) = fs::read_to_string(path) {
            for token in &forbidden_tokens {
                if content.contains(token) {
                    token_violations.push(format!("{}: Contains {}", path.display(), token));
                }
            }
        }
    }

    if token_violations.is_empty() {
        println!("PASS: No forbidden tokens found");
    } else {
        for v in &token_violations {
            println!("  FAIL: {}", v);
        }
        failed_checks += 1;
    }

    // Check 2: Ellipsis patterns
    println!();
    println!("[2/8] Checking for ellipsis patterns...");
    let ellipsis_re = Regex::new(r"^\s*\.\.\.\s*$")?;
    let mut ellipsis_violations: Vec<String> = Vec::new();

    for entry in WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_exclude(path) {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext != "md" {
                continue;
            }
        } else {
            continue;
        }

        if let Ok(content) = fs::read_to_string(path) {
            for (line_num, line) in content.lines().enumerate() {
                if ellipsis_re.is_match(line) {
                    ellipsis_violations.push(format!("{}:{}: Standalone ellipsis", path.display(), line_num + 1));
                }
            }
        }
    }

    if ellipsis_violations.is_empty() {
        println!("PASS: No ellipsis patterns found");
    } else {
        for v in &ellipsis_violations {
            println!("  FAIL: {}", v);
        }
        failed_checks += 1;
    }

    // Check 3: Pre-seal signature files
    println!();
    println!("[3/8] Checking for pre-seal signature files...");
    let mut sig_violations: Vec<String> = Vec::new();

    for entry in WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_exclude(path) {
            continue;
        }

        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            if ext_str == "asc" || ext_str == "sig" {
                sig_violations.push(format!("{}: Pre-seal signature file", path.display()));
            }
        }
    }

    if sig_violations.is_empty() {
        println!("PASS: No pre-seal signature files found");
    } else {
        for v in &sig_violations {
            println!("  FAIL: {}", v);
        }
        failed_checks += 1;
    }

    // Check 4: Emoji detection
    println!();
    println!("[4/8] Checking for emojis...");
    let emoji_re = Regex::new(r"[\x{1F300}-\x{1F9FF}\x{2600}-\x{26FF}\x{2700}-\x{27BF}\x{1F600}-\x{1F64F}\x{1F680}-\x{1F6FF}]")?;
    let mut emoji_violations: Vec<String> = Vec::new();

    for entry in WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_exclude(path) {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext != "md" {
                continue;
            }
        } else {
            continue;
        }

        if let Ok(content) = fs::read_to_string(path) {
            if emoji_re.is_match(&content) {
                emoji_violations.push(format!("{}: Contains emoji", path.display()));
            }
        }
    }

    if emoji_violations.is_empty() {
        println!("PASS: No emojis found");
    } else {
        for v in &emoji_violations {
            println!("  FAIL: {}", v);
        }
        failed_checks += 1;
    }

    // Check 5: Forbidden paths
    println!();
    println!("[5/8] Checking for forbidden paths...");
    let forbidden_paths = ["drafts/", "quarantine/", "tools/legacy/"];
    let mut path_violations: Vec<String> = Vec::new();

    for forbidden in &forbidden_paths {
        let check_path = specs_dir.join(forbidden);
        if check_path.exists() {
            path_violations.push(format!("Forbidden path exists: {}", forbidden));
        }
    }

    if path_violations.is_empty() {
        println!("PASS: No forbidden paths found");
    } else {
        for v in &path_violations {
            println!("  FAIL: {}", v);
        }
        failed_checks += 1;
    }

    // Check 6: Required governance files
    println!();
    println!("[6/8] Checking required governance files...");
    let required_governance_files = [
        "specs/_governance/AUDIT_CONTRACT.md",
        "specs/_governance/NO_DEBT_RULES.md",
        "specs/_governance/DISTRIBUTION_EXCLUSIONS.md",
        "specs/_governance/LIBRARY_SEALING_CONTRACT.md",
        "specs/_governance/LIBRARY_STANDARD_HEADER_CONTRACT.md",
        "specs/_governance/XONAIX_SELF_GOVERNANCE_CONTRACT.md",
        "specs/_governance/UNIT_REGISTRY.json",
    ];
    let mut missing_files: Vec<String> = Vec::new();

    for file in &required_governance_files {
        let file_path = repo_root.join(file);
        if !file_path.exists() {
            missing_files.push(file.to_string());
        }
    }

    if missing_files.is_empty() {
        println!("PASS: All required governance files present");
    } else {
        for f in &missing_files {
            println!("  FAIL: Missing: {}", f);
        }
        failed_checks += 1;
    }

    // Check 7: CRLF line endings
    println!();
    println!("[7/8] Checking for CRLF line endings...");
    let mut crlf_violations: Vec<String> = Vec::new();

    for entry in WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_exclude(path) {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext != "md" && ext != "json" {
                continue;
            }
        } else {
            continue;
        }

        if let Ok(content) = fs::read_to_string(path) {
            if content.contains("\r\n") {
                crlf_violations.push(format!("{}: Contains CRLF", path.display()));
            }
        }
    }

    if crlf_violations.is_empty() {
        println!("PASS: No CRLF line endings found");
    } else {
        for v in &crlf_violations {
            println!("  FAIL: {}", v);
        }
        failed_checks += 1;
    }

    // Check 8: Soft language patterns
    println!();
    println!("[8/8] Checking for soft/advisory language...");
    let soft_patterns = [
        r"\bshould consider\b",
        r"\bmight want to\b",
        r"\bperhaps\b",
        r"\bideally\b",
        r"\bhopefully\b",
    ];
    let mut soft_violations: Vec<String> = Vec::new();

    for entry in WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_exclude(path) {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext != "md" {
                continue;
            }
        } else {
            continue;
        }

        if let Ok(content) = fs::read_to_string(path) {
            for pattern in &soft_patterns {
                let re = Regex::new(pattern)?;
                if re.is_match(&content.to_lowercase()) {
                    soft_violations.push(format!("{}: Contains soft language pattern '{}'", path.display(), pattern));
                }
            }
        }
    }

    if soft_violations.is_empty() {
        println!("PASS: No soft language patterns found");
    } else {
        for v in &soft_violations {
            println!("  WARN: {}", v);
        }
        // Note: soft language is a warning, not a failure for library
    }

    println!();
    println!("=== ENFORCEMENT COMPLETE ===");
    if failed_checks == 0 {
        println!("Result: ALL CHECKS PASSED");
        Ok(())
    } else {
        println!("Result: FAILED - {} check(s) failed", failed_checks);
        Err(EnforceError::EnforcementFailed(failed_checks))
    }
}
