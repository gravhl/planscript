use crate::ast::{ClearanceSide, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
                CatalogError::new(format!("Failed to read catalog directory {path:?}: {error}"))
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
    let mut anchors = HashMap::new();
    anchors.insert("origin".to_string(), Point { x: 0.0, y: 0.0 });
    CatalogItem {
        id,
        name: "Imported IFC Candidate".to_string(),
        category,
        source: Some(CatalogSource {
            format: "IFC".to_string(),
            provider: None,
            source_url,
            license: Some("review-required".to_string()),
            redistributable: false,
        }),
        bim: Some(CatalogBim {
            ifc_class: None,
            ifc_predefined_type: None,
        }),
        size: CatalogSize {
            width: 1.0,
            depth: 1.0,
            height: None,
        },
        footprint: rectangle_footprint(1.0, 1.0),
        clearances: HashMap::new(),
        anchors,
        assets: CatalogAssets {
            ifc: Some(ifc_path),
            ..Default::default()
        },
        status: Some("candidate".to_string()),
        needs_review: vec![
            "dimensions".to_string(),
            "footprint".to_string(),
            "facing".to_string(),
            "clearances".to_string(),
            "license".to_string(),
        ],
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
