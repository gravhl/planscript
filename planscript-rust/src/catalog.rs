use crate::ast::{ClearanceSide, Point};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogSource {
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    pub redistributable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogBim {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifc_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifc_predefined_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CatalogSize {
    pub width: f64,
    pub depth: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub id: String,
    pub name: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CatalogSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bim: Option<CatalogBim>,
    pub size: CatalogSize,
    pub footprint: Vec<Point>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub clearances: HashMap<ClearanceSide, f64>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub anchors: HashMap<String, Point>,
    #[serde(default, skip_serializing_if = "CatalogAssets::is_empty")]
    pub assets: CatalogAssets,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub needs_review: Vec<String>,
}

impl CatalogAssets {
    fn is_empty(&self) -> bool {
        self.svg.is_none() && self.glb.is_none() && self.ifc.is_none()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    items: HashMap<String, CatalogItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatalogError {
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CatalogLintSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogLintIssue {
    pub severity: CatalogLintSeverity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogImportManifest {
    pub items: Vec<CatalogImportSpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogImportSpec {
    pub id: String,
    pub category: String,
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redistributable: Option<bool>,
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CatalogError {}

impl CatalogError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Catalog {
    pub fn builtins() -> Self {
        let mut catalog = Self::default();
        for item in builtin_items() {
            catalog.items.insert(item.id.clone(), item);
        }
        catalog
    }

    pub fn get(&self, id: &str) -> Option<&CatalogItem> {
        self.items.get(id)
    }

    pub fn ids(&self) -> Vec<String> {
        let mut ids = self.items.keys().cloned().collect::<Vec<_>>();
        ids.sort();
        ids
    }

    pub fn insert(&mut self, item: CatalogItem) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn load_path(&mut self, path: impl AsRef<Path>) -> Result<(), CatalogError> {
        let path = path.as_ref();
        if path.is_dir() {
            let entries = fs::read_dir(path).map_err(|error| {
                CatalogError::new(format!(
                    "Failed to read catalog directory {path:?}: {error}"
                ))
            })?;
            for entry in entries {
                let entry = entry.map_err(|error| {
                    CatalogError::new(format!("Failed to read catalog entry in {path:?}: {error}"))
                })?;
                let entry_path = entry.path();
                if is_catalog_file(&entry_path) {
                    self.load_file(&entry_path)?;
                }
            }
            Ok(())
        } else {
            self.load_file(path)
        }
    }

    fn load_file(&mut self, path: &Path) -> Result<(), CatalogError> {
        let text = fs::read_to_string(path).map_err(|error| {
            CatalogError::new(format!("Failed to read catalog file {path:?}: {error}"))
        })?;
        let item = serde_json::from_str::<CatalogItem>(&text).map_err(|error| {
            CatalogError::new(format!("Failed to parse catalog file {path:?}: {error}"))
        })?;
        self.insert(item);
        Ok(())
    }
}

pub fn is_wall_backed_category(category: &str) -> bool {
    matches!(
        category.trim().to_ascii_lowercase().as_str(),
        "sanitary" | "kitchen" | "laundry" | "plumbing" | "appliance" | "fixture" | "fixtures"
    )
}

pub fn prefers_wall_placement_for(category: &str, id: &str) -> bool {
    is_wall_backed_category(category)
        || id.starts_with("builtin.sanitary.")
        || id.starts_with("builtin.kitchen.")
        || id.starts_with("builtin.laundry.")
}

pub fn prefers_wall_placement(item: &CatalogItem) -> bool {
    prefers_wall_placement_for(&item.category, &item.id)
}

pub fn resolve_catalog_path(path: &str, base_dir: Option<&Path>) -> PathBuf {
    let raw = PathBuf::from(path);
    if raw.is_absolute() {
        raw
    } else if let Some(base_dir) = base_dir {
        base_dir.join(raw)
    } else {
        raw
    }
}

pub fn candidate_ifc_item(
    id: String,
    category: String,
    ifc_path: String,
    source_url: Option<String>,
) -> CatalogItem {
    candidate_ifc_item_with_metadata(id, category, ifc_path, source_url, IfcExtract::default())
}

pub fn candidate_ifc_item_from_file(
    id: String,
    category: String,
    ifc_path: impl AsRef<Path>,
    source_url: Option<String>,
) -> Result<CatalogItem, CatalogError> {
    let ifc_path = ifc_path.as_ref();
    let text = fs::read_to_string(ifc_path).map_err(|error| {
        CatalogError::new(format!("Failed to read IFC file {ifc_path:?}: {error}"))
    })?;
    let metadata = extract_ifc_metadata(&text);
    Ok(candidate_ifc_item_with_metadata(
        id,
        category,
        ifc_path.to_string_lossy().to_string(),
        source_url,
        metadata,
    ))
}

pub fn candidate_ifc_item_from_spec(
    spec: &CatalogImportSpec,
    base_dir: Option<&Path>,
) -> Result<CatalogItem, CatalogError> {
    validate_import_spec(spec)?;
    let resolved_path = resolve_catalog_path(&spec.file, base_dir);
    let mut item = candidate_ifc_item_from_file(
        spec.id.clone(),
        spec.category.clone(),
        resolved_path,
        spec.source_url.clone(),
    )?;
    item.assets.ifc = Some(spec.file.clone());
    if let Some(name) = spec.name.as_ref().filter(|name| !name.trim().is_empty()) {
        item.name = name.clone();
    }
    if let Some(source) = item.source.as_mut() {
        if let Some(provider) = &spec.provider {
            source.provider = Some(provider.clone());
        }
        if let Some(license) = &spec.license {
            source.license = Some(license.clone());
        }
        if let Some(redistributable) = spec.redistributable {
            source.redistributable = redistributable;
        }
    }
    if spec.license.is_some() && spec.redistributable == Some(true) {
        item.needs_review.retain(|need| need != "license");
    }
    Ok(item)
}

pub fn import_ifc_manifest(
    manifest_path: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
) -> Result<Vec<PathBuf>, CatalogError> {
    let manifest_path = manifest_path.as_ref();
    let out_dir = out_dir.as_ref();
    let text = fs::read_to_string(manifest_path).map_err(|error| {
        CatalogError::new(format!(
            "Failed to read import manifest {manifest_path:?}: {error}"
        ))
    })?;
    let manifest: CatalogImportManifest = serde_json::from_str(&text).map_err(|error| {
        CatalogError::new(format!(
            "Failed to parse import manifest {manifest_path:?}: {error}"
        ))
    })?;
    let base_dir = manifest_path.parent();
    fs::create_dir_all(out_dir).map_err(|error| {
        CatalogError::new(format!(
            "Failed to create catalog output directory {out_dir:?}: {error}"
        ))
    })?;

    let mut seen_ids = HashSet::new();
    let mut written = Vec::new();
    for spec in &manifest.items {
        if !seen_ids.insert(spec.id.clone()) {
            return Err(CatalogError::new(format!(
                "Import manifest contains duplicate item id {}",
                spec.id
            )));
        }
        let item = candidate_ifc_item_from_spec(spec, base_dir)?;
        let json = serde_json::to_string_pretty(&item).map_err(|error| {
            CatalogError::new(format!(
                "Failed to serialize catalog item {}: {error}",
                spec.id
            ))
        })?;
        let path = out_dir.join(format!("{}.psobj.json", spec.id));
        fs::write(&path, json).map_err(|error| {
            CatalogError::new(format!("Failed to write catalog item {path:?}: {error}"))
        })?;
        written.push(path);
    }
    Ok(written)
}

fn validate_import_spec(spec: &CatalogImportSpec) -> Result<(), CatalogError> {
    if spec.id.trim().is_empty() {
        return Err(CatalogError::new("Import manifest item id is required"));
    }
    if !is_catalog_id(spec.id.as_str()) {
        return Err(CatalogError::new(format!(
            "Import manifest item id {} is not a valid catalog id",
            spec.id
        )));
    }
    if spec.category.trim().is_empty() {
        return Err(CatalogError::new(format!(
            "Import manifest item {} category is required",
            spec.id
        )));
    }
    if spec.file.trim().is_empty() {
        return Err(CatalogError::new(format!(
            "Import manifest item {} file is required",
            spec.id
        )));
    }
    Ok(())
}

fn is_catalog_id(id: &str) -> bool {
    id.split('.').all(is_identifier)
}

fn is_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

pub fn lint_catalog_item(item: &CatalogItem) -> Vec<CatalogLintIssue> {
    let mut issues = Vec::new();
    if item.id.trim().is_empty() {
        issues.push(lint_error("Catalog item id is required"));
    }
    if item.name.trim().is_empty() {
        issues.push(lint_error("Catalog item name is required"));
    }
    if item.category.trim().is_empty() {
        issues.push(lint_error("Catalog item category is required"));
    }
    if item.size.width <= 0.0 {
        issues.push(lint_error("Catalog item size.width must be greater than 0"));
    }
    if item.size.depth <= 0.0 {
        issues.push(lint_error("Catalog item size.depth must be greater than 0"));
    }
    if item.footprint.len() < 3 {
        issues.push(lint_error(
            "Catalog item footprint must have at least 3 points",
        ));
    }
    if polygon_area(&item.footprint) <= 0.0 {
        issues.push(lint_error("Catalog item footprint must have positive area"));
    }
    if item.status.as_deref() == Some("approved") && !item.needs_review.is_empty() {
        issues.push(lint_error(
            "Approved catalog item cannot have non-empty needsReview",
        ));
    }
    if item.status.as_deref() != Some("approved") {
        issues.push(lint_warning(
            "Catalog item is not approved and should be reviewed before curation",
        ));
    }
    if item.source.as_ref().is_none_or(|source| {
        source.license.as_deref().is_none_or(str::is_empty)
            || (!source.redistributable && item.status.as_deref() == Some("approved"))
    }) {
        issues.push(lint_warning(
            "Catalog item license/redistribution metadata needs review",
        ));
    }
    if let Some(bim) = &item.bim {
        if bim.ifc_class.as_deref().is_none_or(str::is_empty) {
            issues.push(lint_warning("BIM metadata is missing ifcClass"));
        }
    } else {
        issues.push(lint_warning("Catalog item is missing BIM metadata"));
    }
    issues
}

fn lint_error(message: impl Into<String>) -> CatalogLintIssue {
    CatalogLintIssue {
        severity: CatalogLintSeverity::Error,
        message: message.into(),
    }
}

fn lint_warning(message: impl Into<String>) -> CatalogLintIssue {
    CatalogLintIssue {
        severity: CatalogLintSeverity::Warning,
        message: message.into(),
    }
}

fn polygon_area(points: &[Point]) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }
    let mut area = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        area += points[i].x * points[j].y;
        area -= points[j].x * points[i].y;
    }
    area.abs() / 2.0
}

#[derive(Debug, Clone, Default, PartialEq)]
struct IfcExtract {
    name: Option<String>,
    ifc_class: Option<String>,
    predefined_type: Option<String>,
    bounds: Option<IfcBounds>,
    unit_scale: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IfcBounds {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    min_z: f64,
    max_z: f64,
}

impl IfcBounds {
    fn width(self) -> f64 {
        self.max_x - self.min_x
    }

    fn depth(self) -> f64 {
        self.max_y - self.min_y
    }

    fn height(self) -> f64 {
        self.max_z - self.min_z
    }
}

fn candidate_ifc_item_with_metadata(
    id: String,
    category: String,
    ifc_path: String,
    source_url: Option<String>,
    metadata: IfcExtract,
) -> CatalogItem {
    let mut anchors = HashMap::new();
    anchors.insert("origin".to_string(), Point { x: 0.0, y: 0.0 });
    let (size, footprint, mut needs_review) = if let Some(bounds) = metadata.bounds {
        let width = bounds.width().abs();
        let depth = bounds.depth().abs();
        if width > 0.001 && depth > 0.001 {
            (
                CatalogSize {
                    width,
                    depth,
                    height: (bounds.height() > 0.001).then(|| bounds.height()),
                },
                rectangle_footprint(width, depth),
                vec![
                    "origin-anchor".to_string(),
                    "facing".to_string(),
                    "clearances".to_string(),
                    "license".to_string(),
                ],
            )
        } else {
            (
                CatalogSize {
                    width: 1.0,
                    depth: 1.0,
                    height: None,
                },
                rectangle_footprint(1.0, 1.0),
                vec![
                    "dimensions".to_string(),
                    "footprint".to_string(),
                    "facing".to_string(),
                    "clearances".to_string(),
                    "license".to_string(),
                ],
            )
        }
    } else {
        (
            CatalogSize {
                width: 1.0,
                depth: 1.0,
                height: None,
            },
            rectangle_footprint(1.0, 1.0),
            vec![
                "dimensions".to_string(),
                "footprint".to_string(),
                "facing".to_string(),
                "clearances".to_string(),
                "license".to_string(),
            ],
        )
    };
    if (metadata.unit_scale - 1.0).abs() > f64::EPSILON {
        needs_review.push("unit-scale".to_string());
    }
    CatalogItem {
        id,
        name: metadata
            .name
            .unwrap_or_else(|| "Imported IFC Candidate".to_string()),
        category,
        source: Some(CatalogSource {
            format: "IFC".to_string(),
            provider: None,
            source_url,
            license: Some("review-required".to_string()),
            redistributable: false,
        }),
        bim: Some(CatalogBim {
            ifc_class: metadata.ifc_class,
            ifc_predefined_type: metadata.predefined_type,
        }),
        size,
        footprint,
        clearances: HashMap::new(),
        anchors,
        assets: CatalogAssets {
            ifc: Some(ifc_path),
            ..Default::default()
        },
        status: Some("candidate".to_string()),
        needs_review,
    }
}

fn extract_ifc_metadata(text: &str) -> IfcExtract {
    let unit_scale = infer_length_unit_scale(text);
    let mut extract = IfcExtract {
        unit_scale,
        ..Default::default()
    };
    let mut cartesian_bounds: Option<IfcBounds> = None;
    let mut point_list_bounds: Option<IfcBounds> = None;
    let mut product_candidate: Option<(String, Option<String>, Option<String>, i32)> = None;
    let mut type_candidate: Option<(String, Option<String>, Option<String>, i32)> = None;

    for statement in ifc_statements(text) {
        let Some(entity) = entity_name(&statement) else {
            continue;
        };
        if entity == "IFCCARTESIANPOINTLIST2D" || entity == "IFCCARTESIANPOINTLIST3D" {
            for point in parse_cartesian_point_list(&statement, unit_scale) {
                point_list_bounds = Some(update_ifc_bounds(point_list_bounds, point));
            }
        } else if entity == "IFCCARTESIANPOINT" {
            if let Some(point) = parse_cartesian_point(&statement, unit_scale) {
                cartesian_bounds = Some(update_ifc_bounds(cartesian_bounds, point));
            }
        }

        if let Some(priority) = product_priority(&entity) {
            let args = entity_arguments(&statement);
            let name = args.get(2).and_then(|arg| clean_ifc_string(arg));
            let predefined = args.iter().rev().find_map(|arg| enum_value(arg));
            if entity.ends_with("TYPE") {
                let replace_type = type_candidate
                    .as_ref()
                    .is_none_or(|(_, _, _, existing)| priority < *existing);
                if replace_type {
                    type_candidate =
                        Some((entity.clone(), name.clone(), predefined.clone(), priority));
                }
            }
            let replace = product_candidate
                .as_ref()
                .is_none_or(|(_, _, _, existing)| priority < *existing);
            if replace {
                product_candidate = Some((entity, name, predefined, priority));
            }
        }
    }

    if let Some((ifc_class, mut name, mut predefined_type, _)) = product_candidate {
        if let Some((type_class, type_name, type_predefined, _)) = &type_candidate {
            if same_ifc_family(&ifc_class, type_class) {
                if predefined_type.is_none() {
                    predefined_type = type_predefined.clone();
                }
                if name.is_none() && is_useful_ifc_name(type_name.as_deref(), type_class) {
                    name = type_name.clone();
                }
            }
        }
        extract.ifc_class = Some(to_pascal_ifc(&ifc_class));
        extract.name = name;
        extract.predefined_type = predefined_type;
    }
    extract.bounds = point_list_bounds.or(cartesian_bounds);
    extract
}

fn ifc_statements(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        current.push(ch);
        if ch == '\'' {
            if in_string && chars.peek() == Some(&'\'') {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            } else {
                in_string = !in_string;
            }
        } else if ch == ';' && !in_string {
            out.push(current.trim().to_string());
            current.clear();
        }
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

fn entity_name(statement: &str) -> Option<String> {
    let after_equal = statement.split_once('=')?.1.trim_start();
    let end = after_equal.find('(')?;
    Some(after_equal[..end].trim().to_ascii_uppercase())
}

fn entity_arguments(statement: &str) -> Vec<String> {
    let Some(start) = statement.find('(') else {
        return Vec::new();
    };
    let Some(end) = statement.rfind(')') else {
        return Vec::new();
    };
    split_top_level_args(&statement[start + 1..end])
}

fn split_top_level_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\'' {
            current.push(ch);
            if in_string && chars.peek() == Some(&'\'') {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            } else {
                in_string = !in_string;
            }
            continue;
        }
        if !in_string {
            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 0 => {
                    args.push(current.trim().to_string());
                    current.clear();
                    continue;
                }
                _ => {}
            }
        }
        current.push(ch);
    }
    if !current.trim().is_empty() {
        args.push(current.trim().to_string());
    }
    args
}

fn clean_ifc_string(arg: &str) -> Option<String> {
    let trimmed = arg.trim();
    if trimmed == "$" || trimmed == "*" || !trimmed.starts_with('\'') {
        return None;
    }
    let without_quotes = trimmed.strip_prefix('\'')?.strip_suffix('\'')?;
    let value = without_quotes.replace("''", "'");
    (!value.trim().is_empty()).then(|| value)
}

fn enum_value(arg: &str) -> Option<String> {
    let trimmed = arg.trim();
    if !trimmed.starts_with('.') || !trimmed.ends_with('.') || trimmed.len() < 3 {
        return None;
    }
    let value = &trimmed[1..trimmed.len() - 1];
    if matches!(value, "T" | "F" | "U" | "NOTDEFINED") {
        None
    } else {
        Some(value.to_string())
    }
}

fn parse_cartesian_point(statement: &str, scale: f64) -> Option<[f64; 3]> {
    let args = entity_arguments(statement);
    let first = args.first()?;
    let numbers = parse_numbers(first);
    if numbers.len() < 2 {
        return None;
    }
    Some([
        numbers[0] * scale,
        numbers[1] * scale,
        numbers.get(2).copied().unwrap_or(0.0) * scale,
    ])
}

fn parse_cartesian_point_list(statement: &str, scale: f64) -> Vec<[f64; 3]> {
    let Some(entity) = entity_name(statement) else {
        return Vec::new();
    };
    let dimensions = if entity == "IFCCARTESIANPOINTLIST2D" {
        2
    } else {
        3
    };
    let args = entity_arguments(statement);
    let Some(first) = args.first() else {
        return Vec::new();
    };
    parse_numbers(first)
        .chunks_exact(dimensions)
        .map(|chunk| {
            [
                chunk[0] * scale,
                chunk[1] * scale,
                chunk.get(2).copied().unwrap_or(0.0) * scale,
            ]
        })
        .collect()
}

fn parse_numbers(input: &str) -> Vec<f64> {
    let mut numbers = Vec::new();
    let mut current = String::new();
    for ch in input.chars() {
        if ch.is_ascii_digit() || matches!(ch, '-' | '+' | '.' | 'E' | 'e') {
            current.push(ch);
        } else if !current.is_empty() {
            if let Ok(value) = current.parse::<f64>() {
                numbers.push(value);
            }
            current.clear();
        }
    }
    if !current.is_empty() {
        if let Ok(value) = current.parse::<f64>() {
            numbers.push(value);
        }
    }
    numbers
}

fn update_ifc_bounds(bounds: Option<IfcBounds>, point: [f64; 3]) -> IfcBounds {
    if let Some(bounds) = bounds {
        IfcBounds {
            min_x: bounds.min_x.min(point[0]),
            max_x: bounds.max_x.max(point[0]),
            min_y: bounds.min_y.min(point[1]),
            max_y: bounds.max_y.max(point[1]),
            min_z: bounds.min_z.min(point[2]),
            max_z: bounds.max_z.max(point[2]),
        }
    } else {
        IfcBounds {
            min_x: point[0],
            max_x: point[0],
            min_y: point[1],
            max_y: point[1],
            min_z: point[2],
            max_z: point[2],
        }
    }
}

fn infer_length_unit_scale(text: &str) -> f64 {
    for statement in ifc_statements(text) {
        let upper = statement.to_ascii_uppercase();
        if upper.contains("IFCCONVERSIONBASEDUNIT") && upper.contains(".LENGTHUNIT.") {
            if upper.contains("'INCH'") {
                return 0.0254;
            }
            if upper.contains("'FOOT'") || upper.contains("'FEET'") {
                return 0.3048;
            }
            if upper.contains("'YARD'") {
                return 0.9144;
            }
            if upper.contains("'MILLIMETRE'") || upper.contains("'MILLIMETER'") {
                return 0.001;
            }
            if upper.contains("'CENTIMETRE'") || upper.contains("'CENTIMETER'") {
                return 0.01;
            }
        }
    }
    for statement in ifc_statements(text) {
        let upper = statement.to_ascii_uppercase();
        if upper.contains("IFCSIUNIT")
            && upper.contains(".LENGTHUNIT.")
            && upper.contains(".METRE.")
        {
            if upper.contains(".MILLI.") {
                return 0.001;
            }
            if upper.contains(".CENTI.") {
                return 0.01;
            }
            if upper.contains(".DECI.") {
                return 0.1;
            }
            if upper.contains(".KILO.") {
                return 1000.0;
            }
            return 1.0;
        }
    }
    1.0
}

fn product_priority(entity: &str) -> Option<i32> {
    match entity {
        "IFCSANITARYTERMINAL" => Some(0),
        "IFCELECTRICAPPLIANCE" => Some(1),
        "IFCFURNISHINGELEMENT" => Some(2),
        "IFCFURNITURE" => Some(3),
        "IFCDOOR" => Some(4),
        "IFCWINDOW" => Some(5),
        "IFCFLOWTERMINAL" => Some(6),
        "IFCBUILDINGELEMENTPROXY" => Some(7),
        "IFCCOLUMN" => Some(8),
        "IFCBEAM" => Some(9),
        "IFCMEMBER" => Some(10),
        "IFCPLATE" => Some(11),
        "IFCSLAB" => Some(12),
        "IFCWALL" => Some(13),
        "IFCWALLSTANDARDCASE" => Some(14),
        "IFCCOVERING" => Some(15),
        "IFCSANITARYTERMINALTYPE" => Some(20),
        "IFCELECTRICAPPLIANCETYPE" => Some(21),
        "IFCFURNITURETYPE" => Some(22),
        "IFCDOORTYPE" => Some(23),
        "IFCWINDOWTYPE" => Some(24),
        "IFCFLOWTERMINALTYPE" => Some(25),
        "IFCCOLUMNTYPE" => Some(26),
        "IFCBEAMTYPE" => Some(27),
        "IFCMEMBERTYPE" => Some(28),
        "IFCPLATETYPE" => Some(29),
        "IFCSLABTYPE" => Some(30),
        "IFCWALLTYPE" => Some(31),
        "IFCCOVERINGTYPE" => Some(32),
        _ => None,
    }
}

fn same_ifc_family(entity: &str, type_entity: &str) -> bool {
    type_entity
        .strip_suffix("TYPE")
        .is_some_and(|base| base == entity)
}

fn is_useful_ifc_name(name: Option<&str>, entity: &str) -> bool {
    let Some(name) = name else {
        return false;
    };
    let trimmed = name.trim();
    !trimmed.is_empty()
        && !trimmed.eq_ignore_ascii_case(entity)
        && !trimmed.eq_ignore_ascii_case(&to_pascal_ifc(entity))
}

fn to_pascal_ifc(entity: &str) -> String {
    let known = match entity {
        "IFCSANITARYTERMINAL" => Some("IfcSanitaryTerminal"),
        "IFCSANITARYTERMINALTYPE" => Some("IfcSanitaryTerminalType"),
        "IFCELECTRICAPPLIANCE" => Some("IfcElectricAppliance"),
        "IFCELECTRICAPPLIANCETYPE" => Some("IfcElectricApplianceType"),
        "IFCFURNISHINGELEMENT" => Some("IfcFurnishingElement"),
        "IFCFURNITURE" => Some("IfcFurniture"),
        "IFCFURNITURETYPE" => Some("IfcFurnitureType"),
        "IFCDOOR" => Some("IfcDoor"),
        "IFCDOORTYPE" => Some("IfcDoorType"),
        "IFCWINDOW" => Some("IfcWindow"),
        "IFCWINDOWTYPE" => Some("IfcWindowType"),
        "IFCFLOWTERMINAL" => Some("IfcFlowTerminal"),
        "IFCFLOWTERMINALTYPE" => Some("IfcFlowTerminalType"),
        "IFCBUILDINGELEMENTPROXY" => Some("IfcBuildingElementProxy"),
        "IFCCOLUMN" => Some("IfcColumn"),
        "IFCCOLUMNTYPE" => Some("IfcColumnType"),
        "IFCBEAM" => Some("IfcBeam"),
        "IFCBEAMTYPE" => Some("IfcBeamType"),
        "IFCMEMBER" => Some("IfcMember"),
        "IFCMEMBERTYPE" => Some("IfcMemberType"),
        "IFCPLATE" => Some("IfcPlate"),
        "IFCPLATETYPE" => Some("IfcPlateType"),
        "IFCSLAB" => Some("IfcSlab"),
        "IFCSLABTYPE" => Some("IfcSlabType"),
        "IFCWALL" => Some("IfcWall"),
        "IFCWALLSTANDARDCASE" => Some("IfcWallStandardCase"),
        "IFCWALLTYPE" => Some("IfcWallType"),
        "IFCCOVERING" => Some("IfcCovering"),
        "IFCCOVERINGTYPE" => Some("IfcCoveringType"),
        _ => None,
    };
    if let Some(known) = known {
        return known.to_string();
    }
    let lower = entity.to_ascii_lowercase();
    let mut out = String::new();
    let mut capitalize_next = true;
    for ch in lower.chars() {
        if capitalize_next {
            out.extend(ch.to_uppercase());
            capitalize_next = false;
        } else {
            out.push(ch);
        }
        if ch == 'c' && out == "Ifc" {
            capitalize_next = true;
        }
    }
    if out.starts_with("Ifc") {
        out
    } else {
        entity.to_string()
    }
}

fn is_catalog_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".psobj.json"))
}

fn rectangle_footprint(width: f64, depth: f64) -> Vec<Point> {
    let half = width / 2.0;
    vec![
        Point { x: -half, y: 0.0 },
        Point { x: half, y: 0.0 },
        Point { x: half, y: depth },
        Point { x: -half, y: depth },
    ]
}

fn clearances(values: &[(ClearanceSide, f64)]) -> HashMap<ClearanceSide, f64> {
    values.iter().copied().collect()
}

fn builtin_item(
    id: &str,
    name: &str,
    category: &str,
    width: f64,
    depth: f64,
    height: Option<f64>,
    ifc_class: &str,
    ifc_type: Option<&str>,
    clearances_values: &[(ClearanceSide, f64)],
) -> CatalogItem {
    let mut anchors = HashMap::new();
    anchors.insert("wall".to_string(), Point { x: 0.0, y: 0.0 });
    CatalogItem {
        id: id.to_string(),
        name: name.to_string(),
        category: category.to_string(),
        source: Some(CatalogSource {
            format: "PlanScript".to_string(),
            provider: Some("builtin".to_string()),
            source_url: None,
            license: Some("MIT".to_string()),
            redistributable: true,
        }),
        bim: Some(CatalogBim {
            ifc_class: Some(ifc_class.to_string()),
            ifc_predefined_type: ifc_type.map(str::to_string),
        }),
        size: CatalogSize {
            width,
            depth,
            height,
        },
        footprint: rectangle_footprint(width, depth),
        clearances: clearances(clearances_values),
        anchors,
        assets: CatalogAssets::default(),
        status: Some("approved".to_string()),
        needs_review: Vec::new(),
    }
}

pub fn builtin_items() -> Vec<CatalogItem> {
    use ClearanceSide::{Back, Front, Left, Right};
    vec![
        builtin_item(
            "builtin.sanitary.toilet.floor_mounted",
            "Floor Mounted Toilet",
            "sanitary",
            0.38,
            0.68,
            Some(0.78),
            "IfcSanitaryTerminal",
            Some("TOILETPAN"),
            &[(Front, 0.75), (Left, 0.2), (Right, 0.2)],
        ),
        builtin_item(
            "builtin.sanitary.sink.wall_hung",
            "Wall Hung Sink",
            "sanitary",
            0.55,
            0.45,
            Some(0.85),
            "IfcSanitaryTerminal",
            Some("WASHHANDBASIN"),
            &[(Front, 0.7), (Left, 0.15), (Right, 0.15)],
        ),
        builtin_item(
            "builtin.sanitary.shower.size_900x900",
            "900 x 900 Shower",
            "sanitary",
            0.9,
            0.9,
            Some(2.1),
            "IfcSanitaryTerminal",
            Some("SHOWER"),
            &[(Front, 0.7)],
        ),
        builtin_item(
            "builtin.sanitary.tub.size_1700",
            "1700 Bath Tub",
            "sanitary",
            1.7,
            0.75,
            Some(0.55),
            "IfcSanitaryTerminal",
            Some("BATH"),
            &[(Front, 0.7)],
        ),
        builtin_item(
            "builtin.kitchen.sink",
            "Kitchen Sink",
            "kitchen",
            0.8,
            0.6,
            Some(0.9),
            "IfcSanitaryTerminal",
            Some("SINK"),
            &[(Front, 0.75)],
        ),
        builtin_item(
            "builtin.kitchen.range.size_600",
            "600 Range",
            "kitchen",
            0.6,
            0.65,
            Some(0.9),
            "IfcElectricAppliance",
            Some("COOKER"),
            &[(Front, 0.9)],
        ),
        builtin_item(
            "builtin.kitchen.fridge.size_900",
            "900 Refrigerator",
            "kitchen",
            0.9,
            0.75,
            Some(1.9),
            "IfcElectricAppliance",
            Some("REFRIGERATOR"),
            &[(Front, 0.9)],
        ),
        builtin_item(
            "builtin.laundry.washer",
            "Washer",
            "laundry",
            0.65,
            0.65,
            Some(0.9),
            "IfcElectricAppliance",
            Some("WASHINGMACHINE"),
            &[(Front, 0.9)],
        ),
        builtin_item(
            "builtin.laundry.dryer",
            "Dryer",
            "laundry",
            0.65,
            0.65,
            Some(0.9),
            "IfcElectricAppliance",
            Some("DRYER"),
            &[(Front, 0.9)],
        ),
        builtin_item(
            "builtin.furniture.bed.queen",
            "Queen Bed",
            "furniture",
            1.6,
            2.05,
            Some(0.6),
            "IfcFurniture",
            Some("BED"),
            &[(Left, 0.6), (Right, 0.6), (Front, 0.75)],
        ),
        builtin_item(
            "builtin.furniture.sofa.three_seat",
            "Three Seat Sofa",
            "furniture",
            2.1,
            0.9,
            Some(0.85),
            "IfcFurniture",
            Some("SOFA"),
            &[(Front, 0.75)],
        ),
        builtin_item(
            "builtin.furniture.table.dining_6",
            "Dining Table Six",
            "furniture",
            1.8,
            0.9,
            Some(0.75),
            "IfcFurniture",
            Some("TABLE"),
            &[(Front, 0.75), (Back, 0.75), (Left, 0.75), (Right, 0.75)],
        ),
    ]
}
