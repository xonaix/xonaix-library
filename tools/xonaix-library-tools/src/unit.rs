//! Unit management module.
//!
//! Provides commands for validating UNIT.json files, computing audit scope,
//! and verifying dependency graph integrity.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors during unit operations.
#[derive(Debug, Error)]
pub enum UnitError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cannot find specs/ directory")]
    NoSpecsDir,

    #[error("Unit validation failed: {0}")]
    ValidationFailed(String),

    #[error("Graph verification failed: {0}")]
    GraphFailed(String),
}

/// Library UNIT.json schema
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnitJson {
    pub unit_id: String,
    pub unit_type: String,
    pub version: String,
    pub status: String,
    pub description: String,
    pub owner: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub compatibility: Compatibility,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Compatibility {
    pub breaking_changes: String,
    pub backward_compatible: bool,
}

/// UNIT_REGISTRY.json schema
#[derive(Debug, Deserialize, Serialize)]
pub struct UnitRegistry {
    pub registry_version: String,
    pub description: String,
    pub units: HashMap<String, RegistryEntry>,
    #[serde(default)]
    pub reserved: Vec<String>,
    #[serde(default)]
    pub deprecated: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryEntry {
    pub path: String,
    pub domain: String,
    pub status: String,
    pub description: String,
}

/// Find repository root.
fn find_repo_root() -> Result<PathBuf, UnitError> {
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

    Err(UnitError::NoSpecsDir)
}

/// Load the unit registry.
fn load_registry(repo_root: &Path) -> Result<UnitRegistry, UnitError> {
    let registry_path = repo_root.join("specs/_governance/UNIT_REGISTRY.json");
    let content = fs::read_to_string(&registry_path)?;
    let registry: UnitRegistry = serde_json::from_str(&content)?;
    Ok(registry)
}

/// Load a UNIT.json file.
fn load_unit_json(path: &Path) -> Result<UnitJson, UnitError> {
    let content = fs::read_to_string(path)?;
    let unit: UnitJson = serde_json::from_str(&content)?;
    Ok(unit)
}

/// Run unit validate command.
pub fn validate(repo_root_arg: Option<String>, unit_path: Option<String>) -> Result<(), UnitError> {
    let repo_root = match repo_root_arg {
        Some(path) => PathBuf::from(path),
        None => find_repo_root()?,
    };

    println!("=== UNIT VALIDATION ===");
    println!("Repository: {}", repo_root.display());
    println!();

    // Load registry
    println!("[1/3] Loading unit registry...");
    let registry = load_registry(&repo_root)?;
    println!("PASS: Registry loaded ({} units)", registry.units.len());

    let mut errors: Vec<String> = Vec::new();

    // If specific unit path provided, validate only that unit
    let units_to_validate: Vec<(String, String)> = if let Some(path) = unit_path {
        let unit_json_path = repo_root.join(&path).join("UNIT.json");
        if !unit_json_path.exists() {
            return Err(UnitError::ValidationFailed(format!(
                "UNIT.json not found at {}",
                unit_json_path.display()
            )));
        }
        let unit = load_unit_json(&unit_json_path)?;
        vec![(unit.unit_id.clone(), path)]
    } else {
        // Validate all units in registry
        registry.units.iter().map(|(id, entry)| (id.clone(), entry.path.clone())).collect()
    };

    // Validate each unit
    println!();
    println!("[2/3] Validating UNIT.json files...");
    let mut all_units: HashMap<String, UnitJson> = HashMap::new();

    for (unit_id, path) in &units_to_validate {
        let unit_json_path = repo_root.join(path).join("UNIT.json");

        // Check UNIT.json exists
        if !unit_json_path.exists() {
            errors.push(format!("{}: UNIT.json missing", unit_id));
            continue;
        }

        // Load and parse
        let unit = match load_unit_json(&unit_json_path) {
            Ok(u) => u,
            Err(e) => {
                errors.push(format!("{}: Failed to parse UNIT.json: {}", unit_id, e));
                continue;
            }
        };

        // Verify unit_id matches registry
        if &unit.unit_id != unit_id {
            errors.push(format!(
                "{}: unit_id mismatch (registry: {}, file: {})",
                unit_id, unit_id, unit.unit_id
            ));
        }

        // Verify unit_id is in registry
        if !registry.units.contains_key(&unit.unit_id) {
            errors.push(format!("{}: unit_id not in registry", unit.unit_id));
        }

        // Verify unit_type is valid
        if unit.unit_type != "standard" && unit.unit_type != "mini-standard" {
            errors.push(format!("{}: Invalid unit_type: {}", unit_id, unit.unit_type));
        }

        // Verify status is valid
        if unit.status != "active" && unit.status != "deprecated" {
            errors.push(format!("{}: Invalid status: {}", unit_id, unit.status));
        }

        let _ = all_units.insert(unit.unit_id.clone(), unit);
        println!("  {}: OK", unit_id);
    }

    if errors.is_empty() {
        println!("PASS: All UNIT.json files valid");
    } else {
        for err in &errors {
            println!("  ERROR: {}", err);
        }
    }

    // Verify registry path consistency
    println!();
    println!("[3/3] Verifying registry paths...");
    let mut path_errors: Vec<String> = Vec::new();

    for (unit_id, entry) in &registry.units {
        let expected_path = repo_root.join(&entry.path);
        if !expected_path.exists() {
            path_errors.push(format!("{}: Path does not exist: {}", unit_id, entry.path));
        }
    }

    if path_errors.is_empty() {
        println!("PASS: All registry paths exist");
    } else {
        for err in &path_errors {
            println!("  ERROR: {}", err);
        }
        errors.extend(path_errors);
    }

    println!();
    println!("=== VALIDATION COMPLETE ===");
    if errors.is_empty() {
        println!("Result: ALL CHECKS PASSED");
        Ok(())
    } else {
        println!("Result: FAILED - {} error(s)", errors.len());
        Err(UnitError::ValidationFailed(format!("{} validation error(s)", errors.len())))
    }
}

/// Run graph verify command.
pub fn graph_verify(repo_root_arg: Option<String>) -> Result<(), UnitError> {
    let repo_root = match repo_root_arg {
        Some(path) => PathBuf::from(path),
        None => find_repo_root()?,
    };

    println!("=== DEPENDENCY GRAPH VERIFICATION ===");
    println!("Repository: {}", repo_root.display());
    println!();

    // Load registry and all units
    println!("[1/3] Loading units...");
    let registry = load_registry(&repo_root)?;
    let mut all_units: HashMap<String, UnitJson> = HashMap::new();

    for (id, entry) in &registry.units {
        let unit_json_path = repo_root.join(&entry.path).join("UNIT.json");
        if let Ok(unit) = load_unit_json(&unit_json_path) {
            let _ = all_units.insert(id.clone(), unit);
        }
    }
    println!("PASS: Loaded {} units", all_units.len());

    let mut errors: Vec<String> = Vec::new();

    // Build adjacency list
    println!();
    println!("[2/3] Building dependency graph...");
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for (id, unit) in &all_units {
        let deps: Vec<String> = unit.dependencies.clone();
        let _ = graph.insert(id.clone(), deps);
    }

    let edge_count: usize = graph.values().map(|v| v.len()).sum();
    println!("PASS: Graph built ({} nodes, {} edges)", graph.len(), edge_count);

    // Check for cycles using DFS
    println!();
    println!("[3/3] Checking for cycles (DAG enforcement)...");

    fn has_cycle(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        if visiting.contains(node) {
            let cycle_start = path.iter().position(|n| n == node).unwrap_or(0);
            let mut cycle = path[cycle_start..].to_vec();
            cycle.push(node.to_string());
            return Some(cycle);
        }

        if visited.contains(node) {
            return None;
        }

        let _ = visiting.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = graph.get(node) {
            for dep in deps {
                if let Some(cycle) = has_cycle(dep, graph, visiting, visited, path) {
                    return Some(cycle);
                }
            }
        }

        let _ = path.pop();
        let _ = visiting.remove(node);
        let _ = visited.insert(node.to_string());
        None
    }

    let mut visiting: HashSet<String> = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut cycle_found: Option<Vec<String>> = None;

    for node in graph.keys() {
        if !visited.contains(node) {
            let mut path: Vec<String> = Vec::new();
            if let Some(cycle) = has_cycle(node, &graph, &mut visiting, &mut visited, &mut path) {
                cycle_found = Some(cycle);
                break;
            }
        }
    }

    if let Some(cycle) = cycle_found {
        let cycle_str = cycle.join(" -> ");
        errors.push(format!("Cycle detected: {}", cycle_str));
        println!("FAIL: Cycle detected: {}", cycle_str);
    } else {
        println!("PASS: No cycles detected (valid DAG)");
    }

    println!();
    println!("=== GRAPH VERIFICATION COMPLETE ===");
    if errors.is_empty() {
        println!("Result: ALL CHECKS PASSED");
        Ok(())
    } else {
        println!("Result: FAILED - {} error(s)", errors.len());
        Err(UnitError::GraphFailed(format!("{} graph error(s)", errors.len())))
    }
}
