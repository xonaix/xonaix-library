//! Header validation module.
//!
//! Validates document headers against the v2.1 schema specification.

use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

/// Errors during header validation.
#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Cannot find specs/ directory")]
    NoSpecsDir,

    #[error("Header validation failed: {0} error(s), {1} warning(s)")]
    ValidationFailed(u32, u32),
}

/// Valid status values for v2.1 schema.
const VALID_STATUSES: &[&str] = &[
    "draft",
    "internal_review",
    "proposed",
    "approved",
    "sealed",
    "deprecated",
    "superseded",
];

/// Valid trust classes (including compound values for multi-level languages).
const VALID_TRUST_CLASSES: &[&str] = &["L0", "L1", "L2", "L3", "L4", "L1/L2", "L3/L4"];

/// Valid authority tiers.
const VALID_AUTHORITY_TIERS: &[&str] = &["T0", "T1", "T2", "T3"];

/// Valid document types.
const VALID_DOC_TYPES: &[&str] = &["standard", "mini-standard", "template", "contract"];

/// Valid classifications.
const VALID_CLASSIFICATIONS: &[&str] = &["public", "internal", "confidential", "restricted"];


/// Document header structure (partial, for validation).
#[derive(Debug, Deserialize)]
struct DocumentHeader {
    schema: Option<String>,
    schema_version: Option<String>,
    repo: Option<String>,
    path: Option<String>,
    unit_id: Option<String>,
    title: Option<String>,
    document_type: Option<String>,
    language: Option<String>,
    version: Option<String>,
    baseline: Option<serde_yaml::Value>,
    status: Option<String>,
    trust_class: Option<serde_yaml::Value>,
    classification: Option<String>,
    compliance: Option<Vec<String>>,
    owner: Option<String>,
    approved_by: Option<serde_yaml::Value>,
    authority_tier: Option<String>,
    authority: Option<Authority>,
    depends_on: Option<Vec<serde_yaml::Value>>,
    supersedes: Option<serde_yaml::Value>,
    superseded_by: Option<serde_yaml::Value>,
    implements: Option<Vec<serde_yaml::Value>>,
    integrity: Option<Integrity>,
    constitutional_conformance: Option<ConstitutionalConformance>,
    created: Option<String>,
    last_updated: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Authority {
    repo: Option<String>,
    #[serde(rename = "ref")]
    ref_: Option<String>,
    version: Option<serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
struct Integrity {
    hash_alg: Option<serde_yaml::Value>,
    content_hash: Option<serde_yaml::Value>,
    signature: Option<serde_yaml::Value>,
    signed_by: Option<serde_yaml::Value>,
    signed_at: Option<serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
struct ConstitutionalConformance {
    constitution_version: Option<serde_yaml::Value>,
    constitution_hash: Option<serde_yaml::Value>,
    zero_point_version: Option<serde_yaml::Value>,
    zero_point_hash: Option<serde_yaml::Value>,
    deviations: Option<Vec<String>>,
    last_verified: Option<serde_yaml::Value>,
    verified_by: Option<serde_yaml::Value>,
}

/// Validation result for a single file.
struct ValidationResult {
    errors: Vec<String>,
    warnings: Vec<String>,
}

/// Find repository root.
fn find_repo_root() -> Result<PathBuf, HeaderError> {
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

    Err(HeaderError::NoSpecsDir)
}


/// Check if path should be excluded from validation.
fn should_exclude(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    if path_str.contains("_deprecated/") || path_str.contains("_deprecated\\") {
        return true;
    }
    if path_str.contains("_reference/") || path_str.contains("_reference\\") {
        return true;
    }
    if path_str.contains(".git/") || path_str.contains(".git\\") {
        return true;
    }
    if path_str.contains("/target/") || path_str.contains("\target\\") {
        return true;
    }
    if path_str.contains("manifests/") || path_str.contains("manifests\\") {
        return true;
    }
    if path_str.contains("_roadmap/") || path_str.contains("_roadmap\\") {
        return true;
    }

    false
}

/// Extract YAML frontmatter from markdown content.
fn extract_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---") {
        return None;
    }

    let rest = &content[3..];
    if let Some(end_idx) = rest.find("\n---") {
        Some(&rest[..end_idx])
    } else {
        None
    }
}

/// Check if a serde_yaml::Value is null or represents null.
fn is_yaml_null(v: &Option<serde_yaml::Value>) -> bool {
    match v {
        None => true,
        Some(serde_yaml::Value::Null) => true,
        _ => false,
    }
}

/// Get string value from serde_yaml::Value if it's a string.
fn get_yaml_string(v: &Option<serde_yaml::Value>) -> Option<&str> {
    match v {
        Some(serde_yaml::Value::String(s)) => Some(s.as_str()),
        _ => None,
    }
}


/// Validate a single document header.
fn validate_header(_path: &Path, content: &str) -> ValidationResult {
    let mut result = ValidationResult {
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    // Extract frontmatter
    let frontmatter = match extract_frontmatter(content) {
        Some(fm) => fm,
        None => {
            result.errors.push("Missing YAML frontmatter".to_string());
            return result;
        }
    };

    // Parse YAML
    let header: DocumentHeader = match serde_yaml::from_str(frontmatter) {
        Ok(h) => h,
        Err(e) => {
            result.errors.push(format!("Invalid YAML: {}", e));
            return result;
        }
    };

    // Required fields
    if header.schema.as_deref() != Some("xonaix-document-header") {
        result.errors.push("schema must be \"xonaix-document-header\"".to_string());
    }

    match header.schema_version.as_deref() {
        Some("2.0") => {
            result.warnings.push("schema_version is 2.0, should migrate to 2.1".to_string());
        }
        Some("2.1") => {}
        Some(v) => {
            result.errors.push(format!("Unknown schema_version: {}", v));
        }
        None => {
            result.errors.push("Missing schema_version".to_string());
        }
    }

    if header.repo.is_none() {
        result.errors.push("Missing repo".to_string());
    }

    if header.path.is_none() {
        result.errors.push("Missing path".to_string());
    }

    if header.unit_id.is_none() {
        result.errors.push("Missing unit_id".to_string());
    }

    if header.title.is_none() {
        result.errors.push("Missing title".to_string());
    }

    if let Some(doc_type) = &header.document_type {
        if !VALID_DOC_TYPES.contains(&doc_type.as_str()) {
            result.errors.push(format!("Invalid document_type: {}", doc_type));
        }
    } else {
        result.errors.push("Missing document_type".to_string());
    }

    if header.language.is_none() {
        result.errors.push("Missing language".to_string());
    }

    // Version validation
    if let Some(version) = &header.version {
        let valid_prefix = version.starts_with("XZERO-") ||
            version.starts_with("XCORT-") ||
            version.starts_with("XCODE-") ||
            version.starts_with("XNEX-") ||
            version.starts_with("XBLADE-") ||
            version.starts_with("XINFRA-") ||
            version.starts_with("XLIB-") ||
            version.starts_with("XGOV-") ||
            version.starts_with("XUX-");
        if !valid_prefix {
            result.errors.push(format!("Invalid version prefix: {}", version));
        }
    } else {
        result.errors.push("Missing version".to_string());
    }


    // Status validation
    if let Some(status) = &header.status {
        if !VALID_STATUSES.contains(&status.as_str()) {
            // Check for legacy "active" status
            if status == "active" {
                result.warnings.push("status 'active' is deprecated in v2.1, use 'approved' or 'sealed'".to_string());
            } else {
                result.errors.push(format!("Invalid status: {}", status));
            }
        }

        // Check integrity requirements based on status
        let has_content_hash = header.integrity.as_ref()
            .map(|i| !is_yaml_null(&i.content_hash))
            .unwrap_or(false);
        let has_signature = header.integrity.as_ref()
            .map(|i| !is_yaml_null(&i.signature))
            .unwrap_or(false);

        if status == "approved" && !has_content_hash {
            result.warnings.push("status is 'approved' but content_hash is missing (governance debt)".to_string());
        }

        if status == "sealed" || status == "deprecated" || status == "superseded" {
            if !has_content_hash {
                result.errors.push(format!("status is '{}' but content_hash is missing", status));
            }
            if !has_signature {
                result.errors.push(format!("status is '{}' but signature is missing", status));
            }
        }
    } else {
        result.errors.push("Missing status".to_string());
    }

    // Trust class validation
    if let Some(tc_val) = &header.trust_class {
        if let Some(tc) = get_yaml_string(&Some(tc_val.clone())) {
            if !VALID_TRUST_CLASSES.contains(&tc) {
                result.errors.push(format!("Invalid trust_class: {}", tc));
            }
        }
    }

    // Classification validation
    if let Some(class) = &header.classification {
        if !VALID_CLASSIFICATIONS.contains(&class.as_str()) {
            result.errors.push(format!("Invalid classification: {}", class));
        }
    } else {
        result.errors.push("Missing classification".to_string());
    }

    // Authority tier validation
    if let Some(tier) = &header.authority_tier {
        if !VALID_AUTHORITY_TIERS.contains(&tier.as_str()) {
            result.errors.push(format!("Invalid authority_tier: {}", tier));
        }
    } else {
        result.errors.push("Missing authority_tier".to_string());
    }

    // Authority section validation
    if let Some(auth) = &header.authority {
        if auth.repo.is_none() {
            result.errors.push("Missing authority.repo".to_string());
        }
        if auth.ref_.is_none() {
            result.errors.push("Missing authority.ref".to_string());
        }
    } else {
        result.errors.push("Missing authority section".to_string());
    }

    // Owner validation
    if header.owner.is_none() {
        result.errors.push("Missing owner".to_string());
    }

    // Timestamp validation
    if header.created.is_none() {
        result.errors.push("Missing created".to_string());
    }
    if header.last_updated.is_none() {
        result.errors.push("Missing last_updated".to_string());
    }

    result
}


/// Run header validation on all markdown files.
pub fn run(repo_root_arg: Option<String>, file_path: Option<String>) -> Result<(), HeaderError> {
    let repo_root = match repo_root_arg {
        Some(path) => PathBuf::from(path),
        None => find_repo_root()?,
    };

    let specs_dir = repo_root.join("specs");

    println!("=== XONAIX LIBRARY HEADER VALIDATION ===");
    println!("Repository: {}", repo_root.display());
    println!("Schema Version: 2.1");
    println!();

    let mut total_errors = 0u32;
    let mut total_warnings = 0u32;
    let mut files_checked = 0u32;

    // If specific file provided, validate just that one
    if let Some(file) = file_path {
        let path = PathBuf::from(&file);
        if !path.exists() {
            return Err(HeaderError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", file),
            )));
        }

        let content = fs::read_to_string(&path)?;
        let result = validate_header(&path, &content);

        files_checked = 1;
        total_errors = result.errors.len() as u32;
        total_warnings = result.warnings.len() as u32;

        if !result.errors.is_empty() || !result.warnings.is_empty() {
            println!("{}:", path.display());
            for e in &result.errors {
                println!("  ERROR: {}", e);
            }
            for w in &result.warnings {
                println!("  WARN: {}", w);
            }
        }
    } else {
        // Validate all markdown files in specs/
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

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let result = validate_header(path, &content);
            files_checked += 1;

            if !result.errors.is_empty() || !result.warnings.is_empty() {
                println!("{}:", path.display());
                for e in &result.errors {
                    println!("  ERROR: {}", e);
                    total_errors += 1;
                }
                for w in &result.warnings {
                    println!("  WARN: {}", w);
                    total_warnings += 1;
                }
                println!();
            }
        }
    }

    println!("=== VALIDATION COMPLETE ===");
    println!("Files checked: {}", files_checked);
    println!("Errors: {}", total_errors);
    println!("Warnings: {}", total_warnings);

    if total_errors > 0 {
        Err(HeaderError::ValidationFailed(total_errors, total_warnings))
    } else {
        if total_warnings > 0 {
            println!("\nResult: PASSED with {} warning(s)", total_warnings);
        } else {
            println!("\nResult: ALL HEADERS VALID");
        }
        Ok(())
    }
}
