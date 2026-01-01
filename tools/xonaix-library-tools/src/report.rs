//! Governance Report module.
//!
//! Generates comprehensive governance reports with metrics for dashboards,
//! audits, and monitoring systems.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

/// Errors during report generation.
#[derive(Debug, Error)]
pub enum ReportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cannot find specs/ directory")]
    NoSpecsDir,
}

/// Document header for parsing.
#[derive(Debug, Deserialize)]
struct DocumentHeader {
    schema: Option<String>,
    schema_version: Option<String>,
    repo: Option<String>,
    path: Option<String>,
    unit_id: Option<String>,
    title: Option<String>,
    document_type: Option<String>,
    status: Option<String>,
    trust_class: Option<serde_yaml::Value>,
    classification: Option<String>,
    owner: Option<String>,
    approved_by: Option<serde_yaml::Value>,
    authority_tier: Option<String>,
    integrity: Option<Integrity>,
    created: Option<String>,
    last_updated: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Integrity {
    hash_alg: Option<serde_yaml::Value>,
    content_hash: Option<serde_yaml::Value>,
    signature: Option<serde_yaml::Value>,
    signed_by: Option<serde_yaml::Value>,
    signed_at: Option<serde_yaml::Value>,
}

/// Full governance report output.
#[derive(Debug, Serialize)]
pub struct GovernanceReport {
    /// Report metadata
    pub metadata: ReportMetadata,
    /// Summary metrics
    pub summary: ReportSummary,
    /// Per-document details
    pub documents: Vec<DocumentReport>,
    /// Governance debt details
    pub governance_debt: GovernanceDebt,
}

#[derive(Debug, Serialize)]
pub struct ReportMetadata {
    pub generated_at: DateTime<Utc>,
    pub generator: String,
    pub generator_version: String,
    pub repository: String,
    pub report_version: String,
}

#[derive(Debug, Serialize)]
pub struct ReportSummary {
    pub total_documents: u32,
    pub by_status: HashMap<String, u32>,
    pub by_document_type: HashMap<String, u32>,
    pub by_trust_class: HashMap<String, u32>,
    pub by_classification: HashMap<String, u32>,
    pub by_authority_tier: HashMap<String, u32>,
    pub integrity_metrics: IntegrityMetrics,
    pub schema_metrics: SchemaMetrics,
}

#[derive(Debug, Serialize)]
pub struct IntegrityMetrics {
    pub with_content_hash: u32,
    pub without_content_hash: u32,
    pub with_signature: u32,
    pub without_signature: u32,
    pub fully_sealed: u32,
    pub content_hash_coverage_percent: f64,
    pub signature_coverage_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct SchemaMetrics {
    pub v2_1_documents: u32,
    pub v2_0_documents: u32,
    pub other_version_documents: u32,
    pub missing_schema: u32,
}

#[derive(Debug, Serialize)]
pub struct DocumentReport {
    pub path: String,
    pub title: Option<String>,
    pub unit_id: Option<String>,
    pub document_type: Option<String>,
    pub status: Option<String>,
    pub trust_class: Option<String>,
    pub classification: Option<String>,
    pub authority_tier: Option<String>,
    pub owner: Option<String>,
    pub schema_version: Option<String>,
    pub integrity: DocumentIntegrity,
    pub governance_debt: Vec<String>,
    pub created: Option<String>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DocumentIntegrity {
    pub has_content_hash: bool,
    pub has_signature: bool,
    pub hash_algorithm: Option<String>,
    pub signed_by: Option<String>,
    pub signed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GovernanceDebt {
    pub total_debt_items: u32,
    pub documents_with_debt: u32,
    pub debt_by_type: HashMap<String, u32>,
    pub debt_items: Vec<DebtItem>,
}

#[derive(Debug, Serialize)]
pub struct DebtItem {
    pub path: String,
    pub debt_type: String,
    pub description: String,
    pub severity: String,
}

/// Find repository root.
fn find_repo_root() -> Result<PathBuf, ReportError> {
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

    Err(ReportError::NoSpecsDir)
}

/// Check if path should be excluded.
fn should_exclude(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    path_str.contains("_deprecated")
        || path_str.contains(".git")
        || path_str.contains("target")
        || path_str.contains("manifests")
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

/// Check if a serde_yaml::Value is null.
fn is_yaml_null(v: &Option<serde_yaml::Value>) -> bool {
    match v {
        None => true,
        Some(serde_yaml::Value::Null) => true,
        _ => false,
    }
}

/// Get string from yaml value.
fn yaml_to_string(v: &Option<serde_yaml::Value>) -> Option<String> {
    match v {
        Some(serde_yaml::Value::String(s)) => Some(s.clone()),
        _ => None,
    }
}

/// Parse a single document and generate its report.
fn parse_document(path: &Path, content: &str) -> Option<DocumentReport> {
    let frontmatter = extract_frontmatter(content)?;
    let header: DocumentHeader = serde_yaml::from_str(frontmatter).ok()?;

    let has_content_hash = header
        .integrity
        .as_ref()
        .map(|i| !is_yaml_null(&i.content_hash))
        .unwrap_or(false);

    let has_signature = header
        .integrity
        .as_ref()
        .map(|i| !is_yaml_null(&i.signature))
        .unwrap_or(false);

    let hash_alg = header
        .integrity
        .as_ref()
        .and_then(|i| yaml_to_string(&i.hash_alg));

    let signed_by = header
        .integrity
        .as_ref()
        .and_then(|i| yaml_to_string(&i.signed_by));

    let signed_at = header
        .integrity
        .as_ref()
        .and_then(|i| yaml_to_string(&i.signed_at));

    let trust_class = match &header.trust_class {
        Some(serde_yaml::Value::String(s)) => Some(s.clone()),
        _ => None,
    };

    // Calculate governance debt
    let mut debt = Vec::new();
    if let Some(status) = &header.status {
        if status == "approved" && !has_content_hash {
            debt.push("content_hash missing (approved status)".to_string());
        }
        if (status == "sealed" || status == "deprecated" || status == "superseded")
            && !has_content_hash
        {
            debt.push("content_hash missing (sealed/deprecated/superseded)".to_string());
        }
        if (status == "sealed" || status == "deprecated" || status == "superseded")
            && !has_signature
        {
            debt.push("signature missing (sealed/deprecated/superseded)".to_string());
        }
    }

    if header.schema_version.as_deref() == Some("2.0") {
        debt.push("schema_version 2.0 (should migrate to 2.1)".to_string());
    }

    Some(DocumentReport {
        path: path.to_string_lossy().to_string(),
        title: header.title,
        unit_id: header.unit_id,
        document_type: header.document_type,
        status: header.status,
        trust_class,
        classification: header.classification,
        authority_tier: header.authority_tier,
        owner: header.owner,
        schema_version: header.schema_version,
        integrity: DocumentIntegrity {
            has_content_hash,
            has_signature,
            hash_algorithm: hash_alg,
            signed_by,
            signed_at,
        },
        governance_debt: debt,
        created: header.created,
        last_updated: header.last_updated,
    })
}

/// Generate the full governance report.
pub fn generate_report(repo_root_arg: Option<String>) -> Result<GovernanceReport, ReportError> {
    let repo_root = match repo_root_arg {
        Some(path) => PathBuf::from(path),
        None => find_repo_root()?,
    };

    let specs_dir = repo_root.join("specs");
    let mut documents = Vec::new();

    // Scan all markdown files
    for entry in WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_exclude(path) {
            continue;
        }

        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if let Some(doc) = parse_document(path, &content) {
            documents.push(doc);
        }
    }

    // Build summary metrics
    let mut by_status: HashMap<String, u32> = HashMap::new();
    let mut by_document_type: HashMap<String, u32> = HashMap::new();
    let mut by_trust_class: HashMap<String, u32> = HashMap::new();
    let mut by_classification: HashMap<String, u32> = HashMap::new();
    let mut by_authority_tier: HashMap<String, u32> = HashMap::new();

    let mut with_content_hash = 0u32;
    let mut with_signature = 0u32;
    let mut v2_1_docs = 0u32;
    let mut v2_0_docs = 0u32;
    let mut other_version = 0u32;
    let mut missing_schema = 0u32;

    let mut debt_items = Vec::new();
    let mut debt_by_type: HashMap<String, u32> = HashMap::new();
    let mut docs_with_debt = 0u32;

    for doc in &documents {
        // Status counts
        if let Some(status) = &doc.status {
            *by_status.entry(status.clone()).or_insert(0) += 1;
        }

        // Document type counts
        if let Some(dt) = &doc.document_type {
            *by_document_type.entry(dt.clone()).or_insert(0) += 1;
        }

        // Trust class counts
        if let Some(tc) = &doc.trust_class {
            *by_trust_class.entry(tc.clone()).or_insert(0) += 1;
        } else {
            *by_trust_class.entry("unspecified".to_string()).or_insert(0) += 1;
        }

        // Classification counts
        if let Some(c) = &doc.classification {
            *by_classification.entry(c.clone()).or_insert(0) += 1;
        }

        // Authority tier counts
        if let Some(at) = &doc.authority_tier {
            *by_authority_tier.entry(at.clone()).or_insert(0) += 1;
        }

        // Integrity metrics
        if doc.integrity.has_content_hash {
            with_content_hash += 1;
        }
        if doc.integrity.has_signature {
            with_signature += 1;
        }

        // Schema version metrics
        match doc.schema_version.as_deref() {
            Some("2.1") => v2_1_docs += 1,
            Some("2.0") => v2_0_docs += 1,
            Some(_) => other_version += 1,
            None => missing_schema += 1,
        }

        // Governance debt
        if !doc.governance_debt.is_empty() {
            docs_with_debt += 1;
            for debt_desc in &doc.governance_debt {
                let debt_type = if debt_desc.contains("content_hash") {
                    "missing_content_hash"
                } else if debt_desc.contains("signature") {
                    "missing_signature"
                } else if debt_desc.contains("schema_version") {
                    "schema_migration"
                } else {
                    "other"
                };

                *debt_by_type.entry(debt_type.to_string()).or_insert(0) += 1;

                let severity = if debt_desc.contains("sealed") || debt_desc.contains("deprecated") {
                    "error"
                } else {
                    "warning"
                };

                debt_items.push(DebtItem {
                    path: doc.path.clone(),
                    debt_type: debt_type.to_string(),
                    description: debt_desc.clone(),
                    severity: severity.to_string(),
                });
            }
        }
    }

    let total = documents.len() as u32;
    let total_f64 = if total > 0 { total as f64 } else { 1.0 };

    let fully_sealed = documents
        .iter()
        .filter(|d| d.integrity.has_content_hash && d.integrity.has_signature)
        .count() as u32;

    let report = GovernanceReport {
        metadata: ReportMetadata {
            generated_at: Utc::now(),
            generator: "xonaix-library-tools".to_string(),
            generator_version: env!("CARGO_PKG_VERSION").to_string(),
            repository: repo_root.to_string_lossy().to_string(),
            report_version: "1.0.0".to_string(),
        },
        summary: ReportSummary {
            total_documents: total,
            by_status,
            by_document_type,
            by_trust_class,
            by_classification,
            by_authority_tier,
            integrity_metrics: IntegrityMetrics {
                with_content_hash,
                without_content_hash: total.saturating_sub(with_content_hash),
                with_signature,
                without_signature: total.saturating_sub(with_signature),
                fully_sealed,
                content_hash_coverage_percent: (with_content_hash as f64 / total_f64) * 100.0,
                signature_coverage_percent: (with_signature as f64 / total_f64) * 100.0,
            },
            schema_metrics: SchemaMetrics {
                v2_1_documents: v2_1_docs,
                v2_0_documents: v2_0_docs,
                other_version_documents: other_version,
                missing_schema,
            },
        },
        documents,
        governance_debt: GovernanceDebt {
            total_debt_items: debt_items.len() as u32,
            documents_with_debt: docs_with_debt,
            debt_by_type,
            debt_items,
        },
    };

    Ok(report)
}

/// Output format for the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    JsonPretty,
    Table,
    Summary,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "json-pretty" => Ok(OutputFormat::JsonPretty),
            "table" => Ok(OutputFormat::Table),
            "summary" => Ok(OutputFormat::Summary),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

/// Print report in table format.
fn print_table(report: &GovernanceReport) {
    println!("=== XONAIX GOVERNANCE REPORT ===");
    println!("Generated: {}", report.metadata.generated_at);
    println!("Repository: {}", report.metadata.repository);
    println!();

    println!("=== SUMMARY ===");
    println!("Total Documents: {}", report.summary.total_documents);
    println!();

    println!("By Status:");
    let mut statuses: Vec<_> = report.summary.by_status.iter().collect();
    statuses.sort_by_key(|(k, _)| *k);
    for (status, count) in statuses {
        println!("  {:20} {}", status, count);
    }
    println!();

    println!("By Document Type:");
    let mut types: Vec<_> = report.summary.by_document_type.iter().collect();
    types.sort_by_key(|(k, _)| *k);
    for (doc_type, count) in types {
        println!("  {:20} {}", doc_type, count);
    }
    println!();

    println!("Integrity Metrics:");
    println!(
        "  Content Hash Coverage: {:.1}% ({}/{})",
        report.summary.integrity_metrics.content_hash_coverage_percent,
        report.summary.integrity_metrics.with_content_hash,
        report.summary.total_documents
    );
    println!(
        "  Signature Coverage:    {:.1}% ({}/{})",
        report.summary.integrity_metrics.signature_coverage_percent,
        report.summary.integrity_metrics.with_signature,
        report.summary.total_documents
    );
    println!(
        "  Fully Sealed:          {}",
        report.summary.integrity_metrics.fully_sealed
    );
    println!();

    println!("Schema Versions:");
    println!(
        "  v2.1: {}  v2.0: {}  Other: {}  Missing: {}",
        report.summary.schema_metrics.v2_1_documents,
        report.summary.schema_metrics.v2_0_documents,
        report.summary.schema_metrics.other_version_documents,
        report.summary.schema_metrics.missing_schema
    );
    println!();

    println!("=== GOVERNANCE DEBT ===");
    println!(
        "Documents with Debt: {}",
        report.governance_debt.documents_with_debt
    );
    println!("Total Debt Items: {}", report.governance_debt.total_debt_items);
    println!();

    if !report.governance_debt.debt_by_type.is_empty() {
        println!("Debt by Type:");
        let mut debts: Vec<_> = report.governance_debt.debt_by_type.iter().collect();
        debts.sort_by_key(|(k, _)| *k);
        for (debt_type, count) in debts {
            println!("  {:25} {}", debt_type, count);
        }
        println!();
    }

    if !report.governance_debt.debt_items.is_empty() {
        println!("Debt Items:");
        for item in &report.governance_debt.debt_items {
            let severity_marker = if item.severity == "error" { "!" } else { "?" };
            println!("  [{}] {}", severity_marker, item.path);
            println!("      {}", item.description);
        }
    }
}

/// Print summary only.
fn print_summary(report: &GovernanceReport) {
    println!("=== GOVERNANCE SUMMARY ===");
    println!("Documents: {}", report.summary.total_documents);
    println!(
        "Content Hash Coverage: {:.1}%",
        report.summary.integrity_metrics.content_hash_coverage_percent
    );
    println!(
        "Signature Coverage: {:.1}%",
        report.summary.integrity_metrics.signature_coverage_percent
    );
    println!(
        "Governance Debt: {} items in {} documents",
        report.governance_debt.total_debt_items, report.governance_debt.documents_with_debt
    );

    // Status summary on one line
    let statuses: Vec<String> = report
        .summary
        .by_status
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect();
    println!("Status: {}", statuses.join(" | "));
}

/// Run the governance report command.
pub fn run(
    repo_root: Option<String>,
    format: OutputFormat,
    output_file: Option<String>,
) -> Result<(), ReportError> {
    let report = generate_report(repo_root)?;

    let output = match format {
        OutputFormat::Json => serde_json::to_string(&report)?,
        OutputFormat::JsonPretty => serde_json::to_string_pretty(&report)?,
        OutputFormat::Table => {
            print_table(&report);
            return Ok(());
        }
        OutputFormat::Summary => {
            print_summary(&report);
            return Ok(());
        }
    };

    if let Some(file_path) = output_file {
        fs::write(&file_path, &output)?;
        println!("Report written to: {}", file_path);
    } else {
        println!("{}", output);
    }

    Ok(())
}
