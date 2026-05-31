#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorMaterialScope {
    Indoor,
    Outdoor,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorPatternKind {
    Solid,
    Planks,
    Grid,
    Diagonal,
    RunningBond,
    Dots,
    Speckles,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloorMaterialSpec {
    pub id: &'static str,
    pub display_name: &'static str,
    pub scope: FloorMaterialScope,
    pub fill: &'static str,
    pub stroke: &'static str,
    pub pattern: FloorPatternKind,
}

impl FloorMaterialSpec {
    pub fn allows_indoor(self) -> bool {
        matches!(
            self.scope,
            FloorMaterialScope::Indoor | FloorMaterialScope::Both
        )
    }

    pub fn allows_outdoor(self) -> bool {
        matches!(
            self.scope,
            FloorMaterialScope::Outdoor | FloorMaterialScope::Both
        )
    }
}

pub fn floor_material_spec(id: &str) -> Option<FloorMaterialSpec> {
    let normalized = normalize_floor_material_id(id);
    FLOOR_MATERIALS
        .iter()
        .copied()
        .find(|material| material.id == normalized)
}

pub fn normalize_floor_material_id(id: &str) -> String {
    id.trim().to_ascii_lowercase().replace('-', "_")
}

pub fn floor_pattern_id(material_id: &str) -> String {
    format!(
        "floor-material-{}",
        normalize_floor_material_id(material_id)
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    ch
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .replace('_', "-")
    )
}

const FLOOR_MATERIALS: &[FloorMaterialSpec] = &[
    FloorMaterialSpec {
        id: "hardwood",
        display_name: "Hardwood",
        scope: FloorMaterialScope::Indoor,
        fill: "#f2dfbf",
        stroke: "#b98d56",
        pattern: FloorPatternKind::Planks,
    },
    FloorMaterialSpec {
        id: "tile",
        display_name: "Tile",
        scope: FloorMaterialScope::Indoor,
        fill: "#eef1f2",
        stroke: "#aeb8bc",
        pattern: FloorPatternKind::Grid,
    },
    FloorMaterialSpec {
        id: "carpet",
        display_name: "Carpet",
        scope: FloorMaterialScope::Indoor,
        fill: "#e7dfd3",
        stroke: "#b9aa98",
        pattern: FloorPatternKind::Dots,
    },
    FloorMaterialSpec {
        id: "concrete",
        display_name: "Concrete",
        scope: FloorMaterialScope::Both,
        fill: "#e4e6e4",
        stroke: "#a5aaa5",
        pattern: FloorPatternKind::Diagonal,
    },
    FloorMaterialSpec {
        id: "polished_concrete",
        display_name: "Polished Concrete",
        scope: FloorMaterialScope::Indoor,
        fill: "#edf0ef",
        stroke: "#b8c0bd",
        pattern: FloorPatternKind::Solid,
    },
    FloorMaterialSpec {
        id: "vinyl",
        display_name: "Vinyl",
        scope: FloorMaterialScope::Indoor,
        fill: "#f0eadc",
        stroke: "#c7b98f",
        pattern: FloorPatternKind::Grid,
    },
    FloorMaterialSpec {
        id: "stone",
        display_name: "Stone",
        scope: FloorMaterialScope::Both,
        fill: "#dedbd2",
        stroke: "#989388",
        pattern: FloorPatternKind::Speckles,
    },
    FloorMaterialSpec {
        id: "wood_deck",
        display_name: "Wood Deck",
        scope: FloorMaterialScope::Outdoor,
        fill: "#d8b47a",
        stroke: "#8f6632",
        pattern: FloorPatternKind::Planks,
    },
    FloorMaterialSpec {
        id: "composite_deck",
        display_name: "Composite Deck",
        scope: FloorMaterialScope::Outdoor,
        fill: "#c9b7a5",
        stroke: "#78695d",
        pattern: FloorPatternKind::Planks,
    },
    FloorMaterialSpec {
        id: "pavers",
        display_name: "Pavers",
        scope: FloorMaterialScope::Outdoor,
        fill: "#d9d3c6",
        stroke: "#918a7d",
        pattern: FloorPatternKind::RunningBond,
    },
    FloorMaterialSpec {
        id: "gravel",
        display_name: "Gravel",
        scope: FloorMaterialScope::Outdoor,
        fill: "#d7d7cf",
        stroke: "#8b8b80",
        pattern: FloorPatternKind::Speckles,
    },
    FloorMaterialSpec {
        id: "grass",
        display_name: "Grass",
        scope: FloorMaterialScope::Outdoor,
        fill: "#d7ead1",
        stroke: "#6da55f",
        pattern: FloorPatternKind::Dots,
    },
    FloorMaterialSpec {
        id: "mulch",
        display_name: "Mulch",
        scope: FloorMaterialScope::Outdoor,
        fill: "#c7a06b",
        stroke: "#7d5131",
        pattern: FloorPatternKind::Speckles,
    },
];
