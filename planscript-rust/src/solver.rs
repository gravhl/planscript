use crate::ast::CardinalDirection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, HashSet, VecDeque};

const GRID_SNAP: f64 = 0.05;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IntentFootprint {
    Rect { min: [f64; 2], max: [f64; 2] },
    Polygon { points: Vec<[f64; 2]> },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BandSpec {
    pub id: String,
    pub min_width: Option<f64>,
    pub target_width: Option<f64>,
    pub max_width: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DepthSpec {
    pub id: String,
    pub min_depth: Option<f64>,
    pub target_depth: Option<f64>,
    pub max_depth: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AspectRatio {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoomSpec {
    pub id: String,
    #[serde(rename = "type")]
    pub room_type: String,
    pub label: Option<String>,
    pub min_area: f64,
    pub target_area: Option<f64>,
    pub max_area: Option<f64>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub aspect: Option<AspectRatio>,
    pub fill_cell: Option<bool>,
    pub preferred_bands: Option<Vec<String>>,
    pub preferred_depths: Option<Vec<String>>,
    pub must_touch_exterior: Option<bool>,
    pub must_touch_edge: Option<EdgeDirection>,
    pub adjacent_to: Option<Vec<String>>,
    pub avoid_adjacent_to: Option<Vec<String>>,
    pub needs_access_from: Option<Vec<String>>,
    pub is_circulation: Option<bool>,
    pub has_exterior_door: Option<bool>,
    pub is_ensuite: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpeningDefaults {
    pub door_width: f64,
    pub window_width: f64,
    pub exterior_door_width: Option<f64>,
    pub corridor_width: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HardConstraints {
    pub no_overlap: bool,
    pub inside_footprint: bool,
    pub all_rooms_reachable: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Weights {
    pub respect_preferred_zones: Option<f64>,
    pub adjacency_satisfaction: Option<f64>,
    pub minimize_hall_area: Option<f64>,
    pub maximize_exterior_glazing: Option<f64>,
    pub bathroom_clustering: Option<f64>,
    pub compactness: Option<f64>,
    pub minimize_exterior_wall_breaks: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccessRule {
    pub room_type: String,
    pub accessible_from: Option<Vec<String>>,
    pub can_lead_to: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayoutIntent {
    pub units: String,
    pub footprint: IntentFootprint,
    pub bands: Option<Vec<BandSpec>>,
    pub depths: Option<Vec<DepthSpec>>,
    pub front_edge: EdgeDirection,
    pub garden_edge: Option<EdgeDirection>,
    pub defaults: OpeningDefaults,
    pub rooms: Vec<RoomSpec>,
    pub hard: HardConstraints,
    pub access_rule_preset: Option<String>,
    pub access_rules: Option<Vec<AccessRule>>,
    pub weights: Option<Weights>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacedRoom {
    pub id: String,
    pub rect: Rect,
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub room_type: String,
    pub band: Option<String>,
    pub depth: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlacedOpeningType {
    Door,
    Window,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacedOpening {
    #[serde(rename = "type")]
    pub opening_type: PlacedOpeningType,
    pub room_id: String,
    pub edge: EdgeDirection,
    pub position: f64,
    pub width: f64,
    pub is_exterior: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connects_to: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementFailure {
    pub room_id: String,
    pub reason: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanState {
    pub footprint: IntentFootprint,
    pub placed: HashMap<String, PlacedRoom>,
    pub unplaced: Vec<String>,
    pub openings: Vec<PlacedOpening>,
    pub corridor_polygon: Option<Vec<Point>>,
    pub failure_reasons: Option<Vec<PlacementFailure>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutCell {
    pub band_id: String,
    pub depth_id: String,
    pub rect: Rect,
    pub inside_footprint: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedBand {
    pub id: String,
    pub x1: f64,
    pub x2: f64,
    pub width: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedDepth {
    pub id: String,
    pub y1: f64,
    pub y2: f64,
    pub depth: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutFrame {
    pub footprint_rect: Rect,
    pub footprint_polygon: Vec<Point>,
    pub is_polygon_footprint: bool,
    pub bands: Vec<ResolvedBand>,
    pub depths: Vec<ResolvedDepth>,
    pub cells: Vec<LayoutCell>,
    pub front_edge: EdgeDirection,
    pub garden_edge: Option<EdgeDirection>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SolveOptions {
    pub variants: Option<usize>,
    pub repair: Option<bool>,
    pub generate_corridor: Option<bool>,
    pub fill_gaps: Option<bool>,
    pub debug: Option<bool>,
    pub inspect: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub total: f64,
    pub components: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "success")]
pub enum SolverResult {
    #[serde(rename = "true")]
    Success {
        plan_script: String,
        state: PlanState,
        score: ScoreBreakdown,
        frame: LayoutFrame,
        warnings: Option<Vec<String>>,
        inspect_trace: Option<String>,
    },
    #[serde(rename = "false")]
    Failure {
        error: String,
        partial_state: Option<PlanState>,
        violations: Option<Vec<String>>,
        inspect_trace: Option<String>,
    },
}

#[derive(Debug, Clone)]
struct Candidate {
    rect: Rect,
    cell: LayoutCell,
    score: f64,
}

pub fn solve(intent: LayoutIntent, options: SolveOptions) -> SolverResult {
    let normalized = normalize_intent(intent);
    let frame = build_layout_frame(&normalized);
    let mut state = place_rooms(&normalized, &frame);

    if !state.unplaced.is_empty() {
        let failures = state.failure_reasons.clone().unwrap_or_default();
        return SolverResult::Failure {
            error: format!(
                "Could not place {} room(s): {}",
                state.unplaced.len(),
                state.unplaced.join(", ")
            ),
            partial_state: Some(state.clone()),
            violations: Some(
                failures
                    .into_iter()
                    .flat_map(|f| {
                        let mut lines = vec![f.reason];
                        if let Some(details) = f.details {
                            lines.push(format!("  -> {details}"));
                        }
                        lines
                    })
                    .collect(),
            ),
            inspect_trace: options.inspect.unwrap_or(false).then(|| {
                let mut placed: Vec<_> = state
                    .placed
                    .values()
                    .map(|r| {
                        format!(
                            "{}: [{:.2},{:.2}] to [{:.2},{:.2}]",
                            r.id, r.rect.x1, r.rect.y1, r.rect.x2, r.rect.y2
                        )
                    })
                    .collect();
                placed.sort();
                format!("Placement failed\nPlaced rooms:\n{}", placed.join("\n"))
            }),
        };
    }

    if options.repair.unwrap_or(true) {
        repair_placement(&mut state, &normalized, &frame);
    }

    place_openings(&mut state, &normalized, &frame);

    let mut warnings = Vec::new();
    let violations = validate_plan_hard(&state, &normalized, &frame, false);
    if !violations.is_empty() {
        return SolverResult::Failure {
            error: "Plan has constraint violations after opening placement".to_string(),
            partial_state: Some(state),
            violations: Some(violations),
            inspect_trace: None,
        };
    }

    if normalized.hard.all_rooms_reachable.unwrap_or(true) {
        if let Some(error) = validate_reachability(&normalized, &state, &frame) {
            return SolverResult::Failure {
                error: "Plan has unreachable rooms".to_string(),
                partial_state: Some(state),
                violations: Some(vec![error]),
                inspect_trace: None,
            };
        }
    } else if let Some(error) = validate_reachability(&normalized, &state, &frame) {
        warnings.push(error);
    }

    let score = score_plan(&state, &normalized, &frame);
    let plan_script = emit_plan_script(&state, &normalized);

    SolverResult::Success {
        plan_script,
        state,
        score,
        frame,
        warnings: (!warnings.is_empty()).then_some(warnings),
        inspect_trace: options
            .inspect
            .unwrap_or(false)
            .then(|| "Rust solver completed".to_string()),
    }
}

pub fn parse_intent(json_source: &str) -> Result<LayoutIntent, serde_json::Error> {
    serde_json::from_str(json_source)
}

pub fn validate_intent(intent: &LayoutIntent) -> Vec<String> {
    let mut errors = Vec::new();
    if intent.rooms.is_empty() {
        errors.push("No rooms specified".to_string());
    }
    if intent.defaults.door_width <= 0.0 {
        errors.push("Missing defaults.doorWidth".to_string());
    }
    if intent.defaults.window_width <= 0.0 {
        errors.push("Missing defaults.windowWidth".to_string());
    }
    let mut ids = HashSet::new();
    for room in &intent.rooms {
        if !ids.insert(room.id.clone()) {
            errors.push(format!("Duplicate room ID: {}", room.id));
        }
        if room.min_area <= 0.0 {
            errors.push(format!("Room {} has invalid minArea", room.id));
        }
    }
    for room in &intent.rooms {
        for adj in room.adjacent_to.iter().flatten() {
            if !ids.contains(adj) {
                errors.push(format!(
                    "Room {} references unknown adjacentTo: {}",
                    room.id, adj
                ));
            }
        }
        for access in room.needs_access_from.iter().flatten() {
            if !ids.contains(access) {
                errors.push(format!(
                    "Room {} references unknown needsAccessFrom: {}",
                    room.id, access
                ));
            }
        }
    }
    errors
}

pub fn get_layout_intent_json_schema_string() -> String {
    serde_json::to_string_pretty(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "PlanScript Layout Intent",
        "type": "object",
        "required": ["units", "footprint", "frontEdge", "defaults", "rooms", "hard"],
        "properties": {
            "units": { "enum": ["m", "cm"] },
            "footprint": {
                "oneOf": [
                    { "type": "object", "required": ["kind", "min", "max"], "properties": { "kind": { "const": "rect" }, "min": { "type": "array" }, "max": { "type": "array" } } },
                    { "type": "object", "required": ["kind", "points"], "properties": { "kind": { "const": "polygon" }, "points": { "type": "array" } } }
                ]
            },
            "frontEdge": { "enum": ["north", "south", "east", "west"] },
            "gardenEdge": { "enum": ["north", "south", "east", "west"] },
            "defaults": {
                "type": "object",
                "required": ["doorWidth", "windowWidth"],
                "properties": {
                    "doorWidth": { "type": "number" },
                    "windowWidth": { "type": "number" },
                    "exteriorDoorWidth": { "type": "number" },
                    "corridorWidth": { "type": "number" }
                }
            },
            "rooms": { "type": "array" },
            "hard": { "type": "object" }
        }
    }))
    .unwrap()
}

pub fn normalize_intent(mut intent: LayoutIntent) -> LayoutIntent {
    if intent.defaults.corridor_width.is_none() {
        intent.defaults.corridor_width = Some(1.2);
    }
    if intent.defaults.exterior_door_width.is_none() {
        intent.defaults.exterior_door_width = Some(intent.defaults.door_width);
    }
    if intent.units == "cm" {
        intent.units = "m".to_string();
        intent.footprint = convert_footprint_to_meters(intent.footprint);
        intent.defaults.door_width /= 100.0;
        intent.defaults.window_width /= 100.0;
        intent.defaults.exterior_door_width =
            intent.defaults.exterior_door_width.map(|v| v / 100.0);
        intent.defaults.corridor_width = intent.defaults.corridor_width.map(|v| v / 100.0);
        for room in &mut intent.rooms {
            room.min_area /= 10_000.0;
            room.target_area = room.target_area.map(|v| v / 10_000.0);
            room.max_area = room.max_area.map(|v| v / 10_000.0);
            room.min_width = room.min_width.map(|v| v / 100.0);
            room.min_height = room.min_height.map(|v| v / 100.0);
            room.max_width = room.max_width.map(|v| v / 100.0);
            room.max_height = room.max_height.map(|v| v / 100.0);
        }
        if let Some(bands) = &mut intent.bands {
            for band in bands {
                band.min_width = band.min_width.map(|v| v / 100.0);
                band.target_width = band.target_width.map(|v| v / 100.0);
                band.max_width = band.max_width.map(|v| v / 100.0);
            }
        }
        if let Some(depths) = &mut intent.depths {
            for depth in depths {
                depth.min_depth = depth.min_depth.map(|v| v / 100.0);
                depth.target_depth = depth.target_depth.map(|v| v / 100.0);
                depth.max_depth = depth.max_depth.map(|v| v / 100.0);
            }
        }
    }
    if intent.hard.all_rooms_reachable.is_none() {
        intent.hard.all_rooms_reachable = Some(true);
    }
    intent
}

fn convert_footprint_to_meters(fp: IntentFootprint) -> IntentFootprint {
    match fp {
        IntentFootprint::Rect { min, max } => IntentFootprint::Rect {
            min: [min[0] / 100.0, min[1] / 100.0],
            max: [max[0] / 100.0, max[1] / 100.0],
        },
        IntentFootprint::Polygon { points } => IntentFootprint::Polygon {
            points: points
                .into_iter()
                .map(|p| [p[0] / 100.0, p[1] / 100.0])
                .collect(),
        },
    }
}

pub fn build_layout_frame(intent: &LayoutIntent) -> LayoutFrame {
    let footprint_rect = footprint_to_rect(&intent.footprint);
    let footprint_polygon = footprint_to_polygon(&intent.footprint);
    let is_polygon_footprint = matches!(intent.footprint, IntentFootprint::Polygon { .. });
    let total_width = rect_width(footprint_rect);
    let total_height = rect_height(footprint_rect);
    let bands = resolve_bands(
        intent.bands.as_deref(),
        total_width,
        footprint_rect.x1,
        intent,
    );
    let depths = resolve_depths(
        intent.depths.as_deref(),
        total_height,
        footprint_rect.y1,
        intent,
    );
    let mut cells = Vec::new();
    for band in &bands {
        for depth in &depths {
            let rect = Rect {
                x1: band.x1,
                y1: depth.y1,
                x2: band.x2,
                y2: depth.y2,
            };
            let inside_footprint = if is_polygon_footprint {
                rect_overlaps_polygon(rect, &footprint_polygon)
            } else {
                true
            };
            cells.push(LayoutCell {
                band_id: band.id.clone(),
                depth_id: depth.id.clone(),
                rect,
                inside_footprint,
            });
        }
    }
    LayoutFrame {
        footprint_rect,
        footprint_polygon,
        is_polygon_footprint,
        bands,
        depths,
        cells,
        front_edge: intent.front_edge,
        garden_edge: intent.garden_edge,
    }
}

fn resolve_bands(
    specs: Option<&[BandSpec]>,
    total_width: f64,
    start_x: f64,
    intent: &LayoutIntent,
) -> Vec<ResolvedBand> {
    if let Some(specs) = specs {
        if !specs.is_empty() {
            return distribute_bands(specs, total_width, start_x);
        }
    }
    let has_left = intent
        .rooms
        .iter()
        .any(|r| pref_contains(&r.preferred_bands, "left"));
    let has_right = intent
        .rooms
        .iter()
        .any(|r| pref_contains(&r.preferred_bands, "right"));
    let has_center = intent
        .rooms
        .iter()
        .any(|r| pref_contains(&r.preferred_bands, "center"));
    if has_left && has_right {
        let left_width = snap(total_width * 0.4);
        return vec![
            ResolvedBand {
                id: "left".to_string(),
                x1: start_x,
                x2: snap(start_x + left_width),
                width: left_width,
            },
            ResolvedBand {
                id: "right".to_string(),
                x1: snap(start_x + left_width),
                x2: snap(start_x + total_width),
                width: snap(total_width - left_width),
            },
        ];
    }
    if has_center {
        let side = snap(total_width * 0.3);
        let center = snap(total_width - 2.0 * side);
        return vec![
            ResolvedBand {
                id: "left".to_string(),
                x1: start_x,
                x2: snap(start_x + side),
                width: side,
            },
            ResolvedBand {
                id: "center".to_string(),
                x1: snap(start_x + side),
                x2: snap(start_x + side + center),
                width: center,
            },
            ResolvedBand {
                id: "right".to_string(),
                x1: snap(start_x + side + center),
                x2: snap(start_x + total_width),
                width: side,
            },
        ];
    }
    vec![ResolvedBand {
        id: "full".to_string(),
        x1: start_x,
        x2: snap(start_x + total_width),
        width: total_width,
    }]
}

fn distribute_bands(specs: &[BandSpec], total_width: f64, start_x: f64) -> Vec<ResolvedBand> {
    let total_target: f64 = if specs.iter().all(|s| s.target_width.is_some()) {
        specs.iter().map(|s| s.target_width.unwrap_or(0.0)).sum()
    } else {
        total_width
    };
    let targets_sum: f64 = specs.iter().filter_map(|s| s.target_width).sum();
    let missing = specs
        .iter()
        .filter(|s| s.target_width.is_none())
        .count()
        .max(1);
    let mut current = start_x;
    let mut bands = Vec::new();
    for (i, spec) in specs.iter().enumerate() {
        let mut width = if let Some(target) = spec.target_width {
            snap((target / total_target.max(0.001)) * total_width)
        } else {
            snap((total_width - targets_sum) / missing as f64)
        };
        if let Some(min) = spec.min_width {
            width = width.max(min);
        }
        if let Some(max) = spec.max_width {
            width = width.min(max);
        }
        if i == specs.len() - 1 {
            width = snap(start_x + total_width - current);
        }
        bands.push(ResolvedBand {
            id: spec.id.clone(),
            x1: current,
            x2: snap(current + width),
            width,
        });
        current = snap(current + width);
    }
    bands
}

fn resolve_depths(
    specs: Option<&[DepthSpec]>,
    total_height: f64,
    start_y: f64,
    intent: &LayoutIntent,
) -> Vec<ResolvedDepth> {
    if let Some(specs) = specs {
        if !specs.is_empty() {
            return distribute_depths(specs, total_height, start_y, intent.front_edge);
        }
    }
    let has_front = intent
        .rooms
        .iter()
        .any(|r| pref_contains(&r.preferred_depths, "front"));
    let has_back = intent
        .rooms
        .iter()
        .any(|r| pref_contains(&r.preferred_depths, "back"));
    let has_middle = intent
        .rooms
        .iter()
        .any(|r| pref_contains(&r.preferred_depths, "middle"));
    let front_first = matches!(
        intent.front_edge,
        EdgeDirection::South | EdgeDirection::West
    );
    if has_front && has_back && has_middle {
        let front = snap(total_height * 0.33);
        let middle = snap(total_height * 0.34);
        let back = snap(total_height - front - middle);
        let zones = if front_first {
            vec![("front", front), ("middle", middle), ("back", back)]
        } else {
            vec![("back", back), ("middle", middle), ("front", front)]
        };
        return build_depths(zones, start_y);
    }
    if has_front && has_back {
        let front = snap(total_height * 0.45);
        let back = snap(total_height - front);
        let zones = if front_first {
            vec![("front", front), ("back", back)]
        } else {
            vec![("back", back), ("front", front)]
        };
        return build_depths(zones, start_y);
    }
    vec![ResolvedDepth {
        id: "full".to_string(),
        y1: start_y,
        y2: snap(start_y + total_height),
        depth: total_height,
    }]
}

fn build_depths(zones: Vec<(&str, f64)>, start_y: f64) -> Vec<ResolvedDepth> {
    let mut out = Vec::new();
    let mut current = start_y;
    for (id, depth) in zones {
        out.push(ResolvedDepth {
            id: id.to_string(),
            y1: current,
            y2: snap(current + depth),
            depth,
        });
        current = snap(current + depth);
    }
    out
}

fn distribute_depths(
    specs: &[DepthSpec],
    total_height: f64,
    start_y: f64,
    front_edge: EdgeDirection,
) -> Vec<ResolvedDepth> {
    let front_first = matches!(front_edge, EdgeDirection::South | EdgeDirection::West);
    let ordered: Vec<&DepthSpec> = if front_first {
        specs.iter().collect()
    } else {
        specs.iter().rev().collect()
    };
    let total_target = if ordered.iter().all(|s| s.target_depth.is_some()) {
        ordered.iter().map(|s| s.target_depth.unwrap()).sum::<f64>()
    } else {
        total_height
    };
    let targets_sum: f64 = ordered.iter().filter_map(|s| s.target_depth).sum();
    let missing = ordered
        .iter()
        .filter(|s| s.target_depth.is_none())
        .count()
        .max(1);
    let mut current = start_y;
    let mut depths = Vec::new();
    for (i, spec) in ordered.iter().enumerate() {
        let mut depth = if let Some(target) = spec.target_depth {
            snap((target / total_target.max(0.001)) * total_height)
        } else {
            snap((total_height - targets_sum) / missing as f64)
        };
        if let Some(min) = spec.min_depth {
            depth = depth.max(min);
        }
        if let Some(max) = spec.max_depth {
            depth = depth.min(max);
        }
        if i == ordered.len() - 1 {
            depth = snap(start_y + total_height - current);
        }
        depths.push(ResolvedDepth {
            id: spec.id.clone(),
            y1: current,
            y2: snap(current + depth),
            depth,
        });
        current = snap(current + depth);
    }
    depths
}

pub fn place_rooms(intent: &LayoutIntent, frame: &LayoutFrame) -> PlanState {
    let ordered = order_rooms(&intent.rooms);
    let mut state = PlanState {
        footprint: intent.footprint.clone(),
        placed: HashMap::new(),
        unplaced: ordered.iter().map(|r| r.id.clone()).collect(),
        openings: Vec::new(),
        corridor_polygon: None,
        failure_reasons: None,
    };
    let mut failures = Vec::new();

    for room in ordered {
        let preferred: Vec<LayoutCell> = frame
            .cells
            .iter()
            .filter(|c| {
                c.inside_footprint
                    && pref_matches(&room.preferred_bands, &c.band_id)
                    && pref_matches(&room.preferred_depths, &c.depth_id)
            })
            .cloned()
            .collect();
        let valid: Vec<LayoutCell> = frame
            .cells
            .iter()
            .filter(|c| c.inside_footprint)
            .cloned()
            .collect();
        let mut candidates = generate_candidates(
            room,
            if preferred.is_empty() {
                &valid
            } else {
                &preferred
            },
            &state,
            intent,
            frame,
        );
        if candidates.is_empty() && !preferred.is_empty() {
            candidates = generate_candidates(room, &valid, &state, intent, frame);
        }
        if let Some(best) = candidates
            .into_iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        {
            let placed = PlacedRoom {
                id: room.id.clone(),
                rect: best.rect,
                label: Some(room.label.clone().unwrap_or_else(|| room.id.clone())),
                room_type: room.room_type.clone(),
                band: Some(best.cell.band_id),
                depth: Some(best.cell.depth_id),
            };
            state.placed.insert(room.id.clone(), placed);
            state.unplaced.retain(|id| id != &room.id);
        } else {
            let mut placed_snapshot: Vec<String> = state
                .placed
                .values()
                .map(|r| {
                    format!(
                        "{} [{:.2},{:.2}]-[{:.2},{:.2}]",
                        r.id, r.rect.x1, r.rect.y1, r.rect.x2, r.rect.y2
                    )
                })
                .collect();
            placed_snapshot.sort();
            failures.push(PlacementFailure {
                room_id: room.id.clone(),
                reason: format!(
                    "Could not generate a valid placement for {} '{}'",
                    room.room_type, room.id
                ),
                details: Some(format!(
                    "Room may be too large for the available cells or blocked by adjacency/edge constraints. Placed at failure: {}",
                    placed_snapshot.join("; ")
                )),
            });
        }
    }

    if !failures.is_empty() {
        state.failure_reasons = Some(failures);
    }
    state
}

fn order_rooms(rooms: &[RoomSpec]) -> Vec<&RoomSpec> {
    let mut attached: HashMap<String, Vec<&RoomSpec>> = HashMap::new();
    let mut standalone = Vec::new();
    for room in rooms {
        let is_attached = room.is_ensuite.unwrap_or(false)
            || room.room_type == "closet"
            || room.room_type == "ensuite"
            || (room.room_type == "bath"
                && room.adjacent_to.as_ref().is_some_and(|a| a.len() == 1)
                && rooms.iter().any(|r| {
                    r.id == room.adjacent_to.as_ref().unwrap()[0] && r.room_type == "bedroom"
                }));
        if is_attached && room.adjacent_to.as_ref().is_some_and(|a| a.len() == 1) {
            attached
                .entry(room.adjacent_to.as_ref().unwrap()[0].clone())
                .or_default()
                .push(room);
        } else {
            standalone.push(room);
        }
    }
    standalone.sort_by(|a, b| {
        room_priority(b, rooms)
            .partial_cmp(&room_priority(a, rooms))
            .unwrap()
    });
    let mut out = Vec::new();
    for room in standalone {
        out.push(room);
        if let Some(children) = attached.get(&room.id) {
            out.extend(children.iter().copied());
        }
    }
    out
}

fn room_priority(room: &RoomSpec, rooms: &[RoomSpec]) -> f64 {
    let mut p = room.min_area;
    if is_circulation(room) && room.has_exterior_door.unwrap_or(false) {
        p += 500.0;
    } else if is_circulation(room) {
        p += 300.0;
    }
    if room.must_touch_edge.is_some() {
        p += 100.0;
    }
    if room.must_touch_exterior.unwrap_or(false) {
        p += 50.0;
    }
    if let Some(adj) = &room.adjacent_to {
        if adj
            .iter()
            .any(|id| rooms.iter().any(|r| &r.id == id && is_circulation(r)))
        {
            p += 80.0;
        }
        p += adj.len() as f64 * 5.0;
    }
    if room.preferred_bands.as_ref().is_some_and(|v| !v.is_empty()) {
        p += 5.0;
    }
    if room
        .preferred_depths
        .as_ref()
        .is_some_and(|v| !v.is_empty())
    {
        p += 5.0;
    }
    if room.room_type == "bath" || room.room_type == "laundry" {
        let needs_circulation_access = room.adjacent_to.as_ref().is_some_and(|adjacent| {
            adjacent.iter().any(|id| {
                rooms.iter().any(|candidate| {
                    &candidate.id == id
                        && (is_circulation(candidate)
                            || matches!(candidate.room_type.as_str(), "kitchen" | "living"))
                })
            })
        });
        if room.room_type == "bath" && room.is_ensuite != Some(true) && needs_circulation_access {
            p += 90.0;
        } else {
            p -= 20.0;
        }
    }
    p
}

fn generate_candidates(
    room: &RoomSpec,
    cells: &[LayoutCell],
    state: &PlanState,
    intent: &LayoutIntent,
    frame: &LayoutFrame,
) -> Vec<Candidate> {
    let placed: Vec<&PlacedRoom> = state.placed.values().collect();
    let adjacent_rooms: Vec<&PlacedRoom> = room
        .adjacent_to
        .as_ref()
        .map(|ids| {
            placed
                .iter()
                .copied()
                .filter(|p| ids.contains(&p.id))
                .collect()
        })
        .unwrap_or_default();

    let mut out = Vec::new();
    for cell in cells {
        for (w, h) in room_sizes(room, rect_width(cell.rect), rect_height(cell.rect)) {
            let mut positions = Vec::new();
            positions.extend([
                (cell.rect.x1, cell.rect.y1, 2.0),
                (cell.rect.x2 - w, cell.rect.y1, 2.0),
                (cell.rect.x1, cell.rect.y2 - h, 2.0),
                (cell.rect.x2 - w, cell.rect.y2 - h, 2.0),
            ]);
            if room.must_touch_exterior.unwrap_or(false) || room.must_touch_edge.is_some() {
                positions.extend(edge_positions(cell.rect, w, h, frame, room));
            }
            for adj in &adjacent_rooms {
                positions.extend(adjacent_positions(adj.rect, cell.rect, w, h, 25.0));
            }
            for other in &placed {
                if !adjacent_rooms.iter().any(|a| a.id == other.id) {
                    positions.extend(adjacent_positions(other.rect, cell.rect, w, h, 3.0));
                }
            }
            let step = 0.2;
            let mut scan_x = cell.rect.x1;
            while scan_x <= cell.rect.x2 - w + 0.001 {
                let mut scan_y = cell.rect.y1;
                while scan_y <= cell.rect.y2 - h + 0.001 {
                    positions.push((scan_x, scan_y, 0.0));
                    scan_y += step;
                }
                scan_x += step;
            }

            for (x, y, bonus) in positions {
                let rect = Rect {
                    x1: snap(x),
                    y1: snap(y),
                    x2: snap(x + w),
                    y2: snap(y + h),
                };
                let cell_tolerance = if bonus >= 25.0 { 1.0 } else { 0.001 };
                if rect.x1 < cell.rect.x1 - cell_tolerance
                    || rect.x2 > cell.rect.x2 + cell_tolerance
                    || rect.y1 < cell.rect.y1 - cell_tolerance
                    || rect.y2 > cell.rect.y2 + cell_tolerance
                {
                    continue;
                }
                if !rect_inside(rect, frame.footprint_rect) && intent.hard.inside_footprint {
                    continue;
                }
                if frame.is_polygon_footprint
                    && !rect_inside_polygon(rect, &frame.footprint_polygon)
                {
                    continue;
                }
                if placed.iter().any(|p| rects_overlap(rect, p.rect, 0.001)) {
                    continue;
                }
                if let Some(edge) = room.must_touch_edge {
                    if !touches_edge(rect, frame.footprint_rect, edge, 0.01) {
                        continue;
                    }
                }
                if room.must_touch_exterior.unwrap_or(false)
                    && !touches_exterior(rect, frame.footprint_rect, 0.01)
                {
                    continue;
                }
                if let Some(adj) = &room.adjacent_to {
                    let placed_required: Vec<&PlacedRoom> = placed
                        .iter()
                        .copied()
                        .filter(|p| adj.contains(&p.id))
                        .collect();
                    if !placed_required.is_empty()
                        && !placed_required
                            .iter()
                            .any(|p| rects_adjacent(rect, p.rect, 0.01))
                    {
                        continue;
                    }
                }
                let score = score_candidate(rect, room, cell, frame, &adjacent_rooms, bonus);
                out.push(Candidate {
                    rect,
                    cell: cell.clone(),
                    score,
                });
            }
        }
    }
    dedup_candidates(out)
}

fn room_sizes(room: &RoomSpec, cell_w: f64, cell_h: f64) -> Vec<(f64, f64)> {
    if room.fill_cell.unwrap_or(false) {
        return vec![(snap(cell_w), snap(cell_h))];
    }
    let target = room.target_area.unwrap_or(room.min_area * 1.1);
    let aspect_min = room.aspect.as_ref().map_or(0.5, |a| a.min);
    let aspect_max = room.aspect.as_ref().map_or(2.0, |a| a.max);
    let mut sizes = Vec::new();
    if is_circulation(room) || room.has_exterior_door.unwrap_or(false) {
        let strip_h = snap(
            (target / cell_w.max(0.001))
                .max(room.min_height.unwrap_or(1.2))
                .min(cell_h),
        );
        if cell_w * strip_h >= room.min_area * 0.95 {
            push_size(&mut sizes, snap(cell_w), strip_h);
        }
        let strip_w = snap(
            (target / cell_h.max(0.001))
                .max(room.min_width.unwrap_or(1.2))
                .min(cell_w),
        );
        if strip_w * cell_h >= room.min_area * 0.95 {
            push_size(&mut sizes, strip_w, snap(cell_h));
        }
    }
    for area in [room.min_area, target * 0.95, target, target * 1.05] {
        if area < room.min_area * 0.95 {
            continue;
        }
        for aspect in [1.0, 0.75, 1.33] {
            if aspect < aspect_min || aspect > aspect_max {
                continue;
            }
            let mut w = snap((area * aspect).sqrt());
            let mut h = snap((area / aspect).sqrt());
            if let Some(min) = room.min_width {
                if w < min {
                    w = min;
                    h = snap(area / w);
                }
            }
            if let Some(min) = room.min_height {
                if h < min {
                    h = min;
                    w = snap(area / h);
                }
            }
            if let Some(max) = room.max_width {
                if w > max {
                    w = max;
                    h = snap(area / w);
                }
            }
            if let Some(max) = room.max_height {
                if h > max {
                    h = max;
                    w = snap(area / h);
                }
            }
            if w > cell_w {
                w = cell_w;
                h = snap((room.min_area / w).max(h.min(cell_h)));
            }
            if h > cell_h {
                h = cell_h;
                w = snap((room.min_area / h).max(w.min(cell_w)));
            }
            if w <= cell_w + 0.001 && h <= cell_h + 0.001 && w * h >= room.min_area * 0.95 {
                push_size(&mut sizes, w, h);
                if (w - h).abs() > 0.1 && h <= cell_w && w <= cell_h {
                    push_size(&mut sizes, h, w);
                }
            }
        }
    }
    sizes
}

fn push_size(sizes: &mut Vec<(f64, f64)>, w: f64, h: f64) {
    if !sizes
        .iter()
        .any(|(ew, eh)| (ew - w).abs() < 0.1 && (eh - h).abs() < 0.1)
    {
        sizes.push((w, h));
    }
}

fn edge_positions(
    cell: Rect,
    w: f64,
    h: f64,
    frame: &LayoutFrame,
    room: &RoomSpec,
) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    let edges = [
        EdgeDirection::South,
        EdgeDirection::North,
        EdgeDirection::West,
        EdgeDirection::East,
    ];
    for edge in edges {
        if room.must_touch_edge.is_some_and(|e| e != edge) {
            continue;
        }
        match edge {
            EdgeDirection::South if (cell.y1 - frame.footprint_rect.y1).abs() < 0.01 => {
                out.push((cell.x1, cell.y1, 8.0))
            }
            EdgeDirection::North if (cell.y2 - frame.footprint_rect.y2).abs() < 0.01 => {
                out.push((cell.x1, cell.y2 - h, 8.0))
            }
            EdgeDirection::West if (cell.x1 - frame.footprint_rect.x1).abs() < 0.01 => {
                out.push((cell.x1, cell.y1, 8.0))
            }
            EdgeDirection::East if (cell.x2 - frame.footprint_rect.x2).abs() < 0.01 => {
                out.push((cell.x2 - w, cell.y1, 8.0))
            }
            _ => {}
        }
    }
    out
}

fn adjacent_positions(adj: Rect, cell: Rect, w: f64, h: f64, bonus: f64) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    let y1 = adj.y1.max(cell.y1);
    let y2 = adj.y2.min(cell.y2);
    if y2 - y1 >= h - 0.01 {
        out.push((adj.x2, y1, bonus));
        out.push((adj.x2, y2 - h, bonus));
        out.push((adj.x1 - w, y1, bonus));
        out.push((adj.x1 - w, y2 - h, bonus));
        let mut y = y1;
        while y <= y2 - h + 0.001 {
            out.push((adj.x2, y, bonus - 1.0));
            out.push((adj.x1 - w, y, bonus - 1.0));
            y += 0.2;
        }
    }
    let x1 = adj.x1.max(cell.x1);
    let x2 = adj.x2.min(cell.x2);
    if x2 - x1 >= w - 0.01 {
        out.push((x1, adj.y2, bonus));
        out.push((x2 - w, adj.y2, bonus));
        out.push((x1, adj.y1 - h, bonus));
        out.push((x2 - w, adj.y1 - h, bonus));
        let mut x = x1;
        while x <= x2 - w + 0.001 {
            out.push((x, adj.y2, bonus - 1.0));
            out.push((x, adj.y1 - h, bonus - 1.0));
            x += 0.2;
        }
    }
    out
}

fn score_candidate(
    rect: Rect,
    room: &RoomSpec,
    cell: &LayoutCell,
    frame: &LayoutFrame,
    adjacent_rooms: &[&PlacedRoom],
    bonus: f64,
) -> f64 {
    let mut score = bonus;
    if (rect.x1 - cell.rect.x1).abs() < 0.01 || (rect.x2 - cell.rect.x2).abs() < 0.01 {
        score += 2.0;
    }
    if (rect.y1 - cell.rect.y1).abs() < 0.01 || (rect.y2 - cell.rect.y2).abs() < 0.01 {
        score += 2.0;
    }
    if matches!(room.room_type.as_str(), "bath" | "ensuite" | "closet") {
        if cell.band_id.contains("private") {
            score += 15.0;
        }
        if cell.band_id.contains("public") {
            score -= 5.0;
        }
        if touches_exterior(rect, frame.footprint_rect, 0.01) {
            score -= 8.0;
        }
    }
    if room.must_touch_exterior.unwrap_or(false)
        && touches_exterior(rect, frame.footprint_rect, 0.01)
    {
        score += 5.0;
    }
    if let Some(edge) = room.must_touch_edge {
        if touches_edge(rect, frame.footprint_rect, edge, 0.01) {
            score += 8.0;
        }
    }
    if is_circulation(room) || room.has_exterior_door.unwrap_or(false) {
        if (rect_width(rect) - rect_width(cell.rect)).abs() < 0.1
            || (rect_height(rect) - rect_height(cell.rect)).abs() < 0.1
        {
            score += 25.0;
        }
    }
    for adj in adjacent_rooms {
        if rects_adjacent(rect, adj.rect, 0.01) {
            let len = shared_edge_length(rect, adj.rect);
            score += if len >= 1.0 {
                25.0 + (len * 2.0).min(10.0)
            } else {
                -15.0
            };
        }
    }
    let area = rect_area(rect);
    let target = room.target_area.unwrap_or(room.min_area * 1.1);
    score -= ((area - target).abs() / target.max(0.001)) * 5.0;
    let aspect = rect_width(rect) / rect_height(rect).max(0.001);
    score -= (aspect - 1.0).abs() * 6.0;
    if !(0.6..=1.67).contains(&aspect) {
        score -= 2.0;
    }
    score
}

fn dedup_candidates(candidates: Vec<Candidate>) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = Vec::new();
    'outer: for c in candidates {
        for u in &out {
            if (u.rect.x1 - c.rect.x1).abs() < 0.05
                && (u.rect.y1 - c.rect.y1).abs() < 0.05
                && (u.rect.x2 - c.rect.x2).abs() < 0.05
                && (u.rect.y2 - c.rect.y2).abs() < 0.05
            {
                continue 'outer;
            }
        }
        out.push(c);
    }
    out
}

pub fn repair_placement(state: &mut PlanState, intent: &LayoutIntent, frame: &LayoutFrame) -> bool {
    let ids: Vec<String> = state.placed.keys().cloned().collect();
    let spec_map: HashMap<&str, &RoomSpec> =
        intent.rooms.iter().map(|r| (r.id.as_str(), r)).collect();
    let mut repaired = false;
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let a = state.placed[&ids[i]].clone();
            let b = state.placed[&ids[j]].clone();
            let area_a = rect_area(a.rect);
            let area_b = rect_area(b.rect);
            if (area_a - area_b).abs() / area_a.max(area_b).max(0.001) > 0.2 {
                continue;
            }
            let Some(spec_a) = spec_map.get(a.id.as_str()).copied() else {
                continue;
            };
            let Some(spec_b) = spec_map.get(b.id.as_str()).copied() else {
                continue;
            };
            if !swap_valid(spec_a, b.rect, spec_b, a.rect, frame.footprint_rect) {
                continue;
            }
            let current = adjacency_satisfaction(state, intent);
            state.placed.get_mut(&ids[i]).unwrap().rect = b.rect;
            state.placed.get_mut(&ids[j]).unwrap().rect = a.rect;
            let swapped = adjacency_satisfaction(state, intent);
            if swapped > current {
                repaired = true;
            } else {
                state.placed.get_mut(&ids[i]).unwrap().rect = a.rect;
                state.placed.get_mut(&ids[j]).unwrap().rect = b.rect;
            }
        }
    }
    repaired
}

fn swap_valid(a: &RoomSpec, rect_a: Rect, b: &RoomSpec, rect_b: Rect, fp: Rect) -> bool {
    if a.must_touch_edge
        .is_some_and(|e| !touches_edge(rect_a, fp, e, 0.01))
    {
        return false;
    }
    if b.must_touch_edge
        .is_some_and(|e| !touches_edge(rect_b, fp, e, 0.01))
    {
        return false;
    }
    if a.must_touch_exterior.unwrap_or(false) && !touches_exterior(rect_a, fp, 0.01) {
        return false;
    }
    if b.must_touch_exterior.unwrap_or(false) && !touches_exterior(rect_b, fp, 0.01) {
        return false;
    }
    true
}

fn place_openings(state: &mut PlanState, intent: &LayoutIntent, frame: &LayoutFrame) {
    state.openings.clear();
    let mut door_pairs = HashSet::<(String, String)>::new();
    let specs: HashMap<&str, &RoomSpec> = intent.rooms.iter().map(|r| (r.id.as_str(), r)).collect();

    for room in state.placed.values() {
        if specs
            .get(room.id.as_str())
            .is_some_and(|s| s.has_exterior_door.unwrap_or(false))
        {
            if let Some(edge) =
                exterior_edge_for_room(room.rect, frame.footprint_rect, Some(intent.front_edge))
            {
                state.openings.push(PlacedOpening {
                    opening_type: PlacedOpeningType::Door,
                    room_id: room.id.clone(),
                    edge,
                    position: 0.5,
                    width: intent
                        .defaults
                        .exterior_door_width
                        .unwrap_or(intent.defaults.door_width),
                    is_exterior: true,
                    connects_to: None,
                });
            }
        }
    }

    let rooms: Vec<PlacedRoom> = state.placed.values().cloned().collect();
    for i in 0..rooms.len() {
        for j in (i + 1)..rooms.len() {
            let a = &rooms[i];
            let b = &rooms[j];
            if !rects_adjacent(a.rect, b.rect, 0.01) {
                continue;
            }
            if shared_edge_length(a.rect, b.rect) < intent.defaults.door_width {
                continue;
            }
            if !door_allowed(a, b, &specs) {
                continue;
            }
            let mut pair = [a.id.clone(), b.id.clone()];
            pair.sort();
            let pair_tuple = (pair[0].clone(), pair[1].clone());
            if door_pairs.contains(&pair_tuple) {
                continue;
            }
            if single_door_room(a, &specs) && has_internal_door(&state.openings, &a.id) {
                continue;
            }
            if single_door_room(b, &specs) && has_internal_door(&state.openings, &b.id) {
                continue;
            }
            let edge = shared_edge_direction(a.rect, b.rect).unwrap_or(EdgeDirection::East);
            state.openings.push(PlacedOpening {
                opening_type: PlacedOpeningType::Door,
                room_id: a.id.clone(),
                edge,
                position: 0.5,
                width: intent.defaults.door_width,
                is_exterior: false,
                connects_to: Some(b.id.clone()),
            });
            door_pairs.insert(pair_tuple);
        }
    }

    for room in state.placed.values() {
        let Some(spec) = specs.get(room.id.as_str()).copied() else {
            continue;
        };
        let wants_window = spec.must_touch_exterior.unwrap_or(false)
            || matches!(
                spec.room_type.as_str(),
                "living" | "dining" | "kitchen" | "bedroom" | "office"
            );
        if !wants_window {
            continue;
        }
        let preferred = intent.garden_edge.or(spec.must_touch_edge);
        if let Some(edge) = exterior_edge_for_room(room.rect, frame.footprint_rect, preferred) {
            state.openings.push(PlacedOpening {
                opening_type: PlacedOpeningType::Window,
                room_id: room.id.clone(),
                edge,
                position: 0.5,
                width: intent.defaults.window_width,
                is_exterior: true,
                connects_to: None,
            });
        }
    }
}

fn door_allowed(a: &PlacedRoom, b: &PlacedRoom, specs: &HashMap<&str, &RoomSpec>) -> bool {
    let spec_a = specs.get(a.id.as_str());
    let spec_b = specs.get(b.id.as_str());
    if let (Some(sa), Some(sb)) = (spec_a, spec_b) {
        if sa.is_ensuite.unwrap_or(false) {
            return sa
                .adjacent_to
                .as_ref()
                .is_some_and(|adj| adj.contains(&b.id));
        }
        if sb.is_ensuite.unwrap_or(false) {
            return sb
                .adjacent_to
                .as_ref()
                .is_some_and(|adj| adj.contains(&a.id));
        }
        if sa.room_type == "closet" {
            return sa
                .adjacent_to
                .as_ref()
                .is_some_and(|adj| adj.contains(&b.id));
        }
        if sb.room_type == "closet" {
            return sb
                .adjacent_to
                .as_ref()
                .is_some_and(|adj| adj.contains(&a.id));
        }
    }
    true
}

fn single_door_room(room: &PlacedRoom, specs: &HashMap<&str, &RoomSpec>) -> bool {
    specs.get(room.id.as_str()).is_some_and(|spec| {
        matches!(
            spec.room_type.as_str(),
            "bath" | "ensuite" | "closet" | "laundry"
        )
    })
}

fn has_internal_door(openings: &[PlacedOpening], room_id: &str) -> bool {
    openings.iter().any(|o| {
        o.opening_type == PlacedOpeningType::Door
            && !o.is_exterior
            && (o.room_id == room_id || o.connects_to.as_deref() == Some(room_id))
    })
}

fn validate_plan_hard(
    state: &PlanState,
    intent: &LayoutIntent,
    frame: &LayoutFrame,
    include_reachability: bool,
) -> Vec<String> {
    let mut errors = Vec::new();
    let rooms: Vec<&PlacedRoom> = state.placed.values().collect();
    if intent.hard.no_overlap {
        for i in 0..rooms.len() {
            for j in (i + 1)..rooms.len() {
                if rects_overlap(rooms[i].rect, rooms[j].rect, 0.001) {
                    errors.push(format!("Rooms {} and {} overlap", rooms[i].id, rooms[j].id));
                }
            }
        }
    }
    if intent.hard.inside_footprint {
        for room in &rooms {
            if !rect_inside(room.rect, frame.footprint_rect) {
                errors.push(format!("Room {} is outside footprint", room.id));
            }
        }
    }
    if include_reachability && intent.hard.all_rooms_reachable.unwrap_or(true) {
        if let Some(err) = validate_reachability(intent, state, frame) {
            errors.push(err);
        }
    }
    errors
}

fn validate_reachability(
    intent: &LayoutIntent,
    state: &PlanState,
    frame: &LayoutFrame,
) -> Option<String> {
    if !intent.hard.all_rooms_reachable.unwrap_or(true) {
        return None;
    }
    if state.placed.len() <= 1 {
        return None;
    }
    let entry = find_entry_room(intent, state, frame)?;
    let graph = build_door_graph(state);
    let unreachable = find_unreachable_rooms(&entry.id, state, &graph);
    (!unreachable.is_empty()).then(|| {
        format!(
            "Unreachable rooms from entry {}: {}",
            entry.id,
            unreachable.join(", ")
        )
    })
}

fn find_entry_room<'a>(
    intent: &LayoutIntent,
    state: &'a PlanState,
    frame: &LayoutFrame,
) -> Option<&'a PlacedRoom> {
    let specs: HashMap<&str, &RoomSpec> = intent.rooms.iter().map(|r| (r.id.as_str(), r)).collect();
    state
        .placed
        .values()
        .find(|r| {
            specs
                .get(r.id.as_str())
                .is_some_and(|s| s.has_exterior_door.unwrap_or(false))
        })
        .or_else(|| state.placed.values().find(|r| r.room_type == "foyer"))
        .or_else(|| {
            state.placed.values().find(|r| {
                is_circulation_spec(r, &specs)
                    && touches_edge(r.rect, frame.footprint_rect, intent.front_edge, 0.01)
            })
        })
        .or_else(|| {
            state
                .placed
                .values()
                .find(|r| touches_edge(r.rect, frame.footprint_rect, intent.front_edge, 0.01))
        })
}

fn build_door_graph(state: &PlanState) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = state
        .placed
        .keys()
        .map(|id| (id.clone(), HashSet::new()))
        .collect();
    for opening in &state.openings {
        if opening.opening_type == PlacedOpeningType::Door {
            if let Some(other) = &opening.connects_to {
                graph
                    .entry(opening.room_id.clone())
                    .or_default()
                    .insert(other.clone());
                graph
                    .entry(other.clone())
                    .or_default()
                    .insert(opening.room_id.clone());
            }
        }
    }
    graph
}

fn find_unreachable_rooms(
    entry: &str,
    state: &PlanState,
    graph: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([entry.to_string()]);
    visited.insert(entry.to_string());
    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for n in neighbors {
                if visited.insert(n.clone()) {
                    queue.push_back(n.clone());
                }
            }
        }
    }
    state
        .placed
        .keys()
        .filter(|id| !visited.contains(*id))
        .cloned()
        .collect()
}

fn score_plan(state: &PlanState, intent: &LayoutIntent, frame: &LayoutFrame) -> ScoreBreakdown {
    let mut components = HashMap::new();
    let adjacency = adjacency_satisfaction(state, intent) as f64;
    let compactness = state
        .placed
        .values()
        .map(|r| {
            let w = rect_width(r.rect);
            let h = rect_height(r.rect);
            1.0 - ((w / h.max(0.001)) - 1.0).abs().min(1.0)
        })
        .sum::<f64>();
    let exterior = state
        .placed
        .values()
        .filter(|r| touches_exterior(r.rect, frame.footprint_rect, 0.01))
        .count() as f64;
    components.insert("adjacencySatisfaction".to_string(), adjacency);
    components.insert("compactness".to_string(), compactness);
    components.insert("maximizeExteriorGlazing".to_string(), exterior);
    let total = adjacency * 3.0 + compactness + exterior;
    ScoreBreakdown { total, components }
}

fn adjacency_satisfaction(state: &PlanState, intent: &LayoutIntent) -> usize {
    let mut count = 0;
    for spec in &intent.rooms {
        if let Some(adjacent) = &spec.adjacent_to {
            if let Some(room) = state.placed.get(&spec.id) {
                for adj in adjacent {
                    if let Some(other) = state.placed.get(adj) {
                        if rects_adjacent(room.rect, other.rect, 0.01) {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

fn emit_plan_script(state: &PlanState, intent: &LayoutIntent) -> String {
    let mut out = String::new();
    out.push_str("units m\n\n");
    out.push_str("defaults {\n");
    out.push_str(&format!(
        "  door_width {}\n",
        fmt(intent.defaults.door_width)
    ));
    out.push_str(&format!(
        "  window_width {}\n",
        fmt(intent.defaults.window_width)
    ));
    out.push_str("}\n\n");
    out.push_str("plan \"Generated Plan\" {\n");
    match &intent.footprint {
        IntentFootprint::Rect { min, max } => {
            out.push_str(&format!(
                "  footprint rect ({}, {}) ({}, {})\n\n",
                fmt(min[0]),
                fmt(min[1]),
                fmt(max[0]),
                fmt(max[1])
            ));
        }
        IntentFootprint::Polygon { points } => {
            out.push_str("  footprint polygon ");
            for p in points {
                out.push_str(&format!("({}, {}) ", fmt(p[0]), fmt(p[1])));
            }
            out.push_str("\n\n");
        }
    }
    let mut rooms: Vec<&PlacedRoom> = state.placed.values().collect();
    rooms.sort_by(|a, b| a.id.cmp(&b.id));
    for room in rooms {
        out.push_str(&format!("  room {} {{\n", room.id));
        out.push_str(&format!(
            "    rect ({}, {}) ({}, {})\n",
            fmt(room.rect.x1),
            fmt(room.rect.y1),
            fmt(room.rect.x2),
            fmt(room.rect.y2)
        ));
        if let Some(label) = &room.label {
            out.push_str(&format!("    label \"{}\"\n", label.replace('"', "")));
        }
        out.push_str("  }\n\n");
    }
    let mut door_count = 1;
    let mut window_count = 1;
    for opening in &state.openings {
        match opening.opening_type {
            PlacedOpeningType::Door if opening.is_exterior => {
                out.push_str(&format!("  opening door d{} {{\n", door_count));
                door_count += 1;
                out.push_str(&format!(
                    "    on {}.edge {}\n",
                    opening.room_id,
                    edge_label(opening.edge)
                ));
                out.push_str("    at 50%\n");
                out.push_str(&format!("    width {}\n", fmt(opening.width)));
                out.push_str("  }\n\n");
            }
            PlacedOpeningType::Door => {
                if let Some(to) = &opening.connects_to {
                    out.push_str(&format!("  opening door d{} {{\n", door_count));
                    door_count += 1;
                    out.push_str(&format!("    between {} and {}\n", opening.room_id, to));
                    out.push_str("    on shared_edge\n");
                    out.push_str("    at 50%\n");
                    out.push_str(&format!("    width {}\n", fmt(opening.width)));
                    out.push_str("  }\n\n");
                }
            }
            PlacedOpeningType::Window => {
                out.push_str(&format!("  opening window w{} {{\n", window_count));
                window_count += 1;
                out.push_str(&format!(
                    "    on {}.edge {}\n",
                    opening.room_id,
                    edge_label(opening.edge)
                ));
                out.push_str("    at 50%\n");
                out.push_str(&format!("    width {}\n", fmt(opening.width)));
                out.push_str("  }\n\n");
            }
        }
    }
    out.push_str("  assert no_overlap rooms\n");
    out.push_str("  assert inside footprint all_rooms\n");
    out.push_str("}\n");
    out
}

fn pref_contains(prefs: &Option<Vec<String>>, value: &str) -> bool {
    prefs.as_ref().is_some_and(|p| p.iter().any(|v| v == value))
}

fn pref_matches(prefs: &Option<Vec<String>>, value: &str) -> bool {
    prefs
        .as_ref()
        .map_or(true, |p| p.is_empty() || p.iter().any(|v| v == value))
}

fn is_circulation(room: &RoomSpec) -> bool {
    room.is_circulation.unwrap_or(false)
        || matches!(
            room.room_type.as_str(),
            "hall" | "corridor" | "foyer" | "stairwell"
        )
}

fn is_circulation_spec(room: &PlacedRoom, specs: &HashMap<&str, &RoomSpec>) -> bool {
    specs
        .get(room.id.as_str())
        .is_some_and(|s| is_circulation(s))
        || matches!(
            room.room_type.as_str(),
            "hall" | "corridor" | "foyer" | "stairwell"
        )
}

pub fn snap(value: f64) -> f64 {
    (value / GRID_SNAP).round() * GRID_SNAP
}

pub fn rect_width(r: Rect) -> f64 {
    r.x2 - r.x1
}
pub fn rect_height(r: Rect) -> f64 {
    r.y2 - r.y1
}
pub fn rect_area(r: Rect) -> f64 {
    rect_width(r) * rect_height(r)
}

pub fn rects_overlap(a: Rect, b: Rect, epsilon: f64) -> bool {
    !(a.x2 <= b.x1 + epsilon
        || b.x2 <= a.x1 + epsilon
        || a.y2 <= b.y1 + epsilon
        || b.y2 <= a.y1 + epsilon)
}

pub fn rect_inside(inner: Rect, outer: Rect) -> bool {
    inner.x1 >= outer.x1 - 0.001
        && inner.y1 >= outer.y1 - 0.001
        && inner.x2 <= outer.x2 + 0.001
        && inner.y2 <= outer.y2 + 0.001
}

pub fn rects_adjacent(a: Rect, b: Rect, epsilon: f64) -> bool {
    ((a.x2 - b.x1).abs() < epsilon || (a.x1 - b.x2).abs() < epsilon)
        && a.y1 < b.y2 - epsilon
        && a.y2 > b.y1 + epsilon
        || ((a.y2 - b.y1).abs() < epsilon || (a.y1 - b.y2).abs() < epsilon)
            && a.x1 < b.x2 - epsilon
            && a.x2 > b.x1 + epsilon
}

pub fn shared_edge_length(a: Rect, b: Rect) -> f64 {
    if (a.x2 - b.x1).abs() < 0.001 || (a.x1 - b.x2).abs() < 0.001 {
        return (a.y2.min(b.y2) - a.y1.max(b.y1)).max(0.0);
    }
    if (a.y2 - b.y1).abs() < 0.001 || (a.y1 - b.y2).abs() < 0.001 {
        return (a.x2.min(b.x2) - a.x1.max(b.x1)).max(0.0);
    }
    0.0
}

pub fn touches_edge(rect: Rect, footprint: Rect, edge: EdgeDirection, epsilon: f64) -> bool {
    match edge {
        EdgeDirection::South => (rect.y1 - footprint.y1).abs() < epsilon,
        EdgeDirection::North => (rect.y2 - footprint.y2).abs() < epsilon,
        EdgeDirection::West => (rect.x1 - footprint.x1).abs() < epsilon,
        EdgeDirection::East => (rect.x2 - footprint.x2).abs() < epsilon,
    }
}

pub fn touches_exterior(rect: Rect, footprint: Rect, epsilon: f64) -> bool {
    [
        EdgeDirection::South,
        EdgeDirection::North,
        EdgeDirection::West,
        EdgeDirection::East,
    ]
    .iter()
    .any(|e| touches_edge(rect, footprint, *e, epsilon))
}

fn footprint_to_rect(fp: &IntentFootprint) -> Rect {
    match fp {
        IntentFootprint::Rect { min, max } => Rect {
            x1: min[0],
            y1: min[1],
            x2: max[0],
            y2: max[1],
        },
        IntentFootprint::Polygon { points } => {
            let xs: Vec<f64> = points.iter().map(|p| p[0]).collect();
            let ys: Vec<f64> = points.iter().map(|p| p[1]).collect();
            Rect {
                x1: xs.iter().copied().fold(f64::INFINITY, f64::min),
                y1: ys.iter().copied().fold(f64::INFINITY, f64::min),
                x2: xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                y2: ys.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            }
        }
    }
}

fn footprint_to_polygon(fp: &IntentFootprint) -> Vec<Point> {
    match fp {
        IntentFootprint::Rect { min, max } => vec![
            Point {
                x: min[0],
                y: min[1],
            },
            Point {
                x: max[0],
                y: min[1],
            },
            Point {
                x: max[0],
                y: max[1],
            },
            Point {
                x: min[0],
                y: max[1],
            },
        ],
        IntentFootprint::Polygon { points } => {
            points.iter().map(|p| Point { x: p[0], y: p[1] }).collect()
        }
    }
}

fn rect_overlaps_polygon(rect: Rect, polygon: &[Point]) -> bool {
    let center = Point {
        x: (rect.x1 + rect.x2) / 2.0,
        y: (rect.y1 + rect.y2) / 2.0,
    };
    point_in_polygon(center, polygon)
}

fn rect_inside_polygon(rect: Rect, polygon: &[Point]) -> bool {
    let points = [
        Point {
            x: rect.x1,
            y: rect.y1,
        },
        Point {
            x: rect.x2,
            y: rect.y1,
        },
        Point {
            x: rect.x2,
            y: rect.y2,
        },
        Point {
            x: rect.x1,
            y: rect.y2,
        },
        Point {
            x: (rect.x1 + rect.x2) / 2.0,
            y: (rect.y1 + rect.y2) / 2.0,
        },
    ];
    points
        .iter()
        .all(|p| point_in_polygon(*p, polygon) || point_on_boundary(*p, polygon))
}

fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let mut j = polygon.len().saturating_sub(1);
    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];
        if (pi.y > point.y) != (pj.y > point.y)
            && point.x < ((pj.x - pi.x) * (point.y - pi.y)) / (pj.y - pi.y) + pi.x
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn point_on_boundary(point: Point, polygon: &[Point]) -> bool {
    polygon.iter().enumerate().any(|(i, a)| {
        let b = polygon[(i + 1) % polygon.len()];
        let d = ((point.x - a.x).powi(2) + (point.y - a.y).powi(2)).sqrt()
            + ((point.x - b.x).powi(2) + (point.y - b.y).powi(2)).sqrt()
            - ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();
        d.abs() < 0.001
    })
}

fn exterior_edge_for_room(
    rect: Rect,
    fp: Rect,
    preferred: Option<EdgeDirection>,
) -> Option<EdgeDirection> {
    if let Some(edge) = preferred {
        if touches_edge(rect, fp, edge, 0.01) {
            return Some(edge);
        }
    }
    [
        EdgeDirection::South,
        EdgeDirection::North,
        EdgeDirection::West,
        EdgeDirection::East,
    ]
    .into_iter()
    .find(|e| touches_edge(rect, fp, *e, 0.01))
}

fn shared_edge_direction(a: Rect, b: Rect) -> Option<EdgeDirection> {
    if (a.x2 - b.x1).abs() < 0.001 {
        Some(EdgeDirection::East)
    } else if (a.x1 - b.x2).abs() < 0.001 {
        Some(EdgeDirection::West)
    } else if (a.y2 - b.y1).abs() < 0.001 {
        Some(EdgeDirection::North)
    } else if (a.y1 - b.y2).abs() < 0.001 {
        Some(EdgeDirection::South)
    } else {
        None
    }
}

fn edge_label(edge: EdgeDirection) -> &'static str {
    match edge {
        EdgeDirection::North => "north",
        EdgeDirection::South => "south",
        EdgeDirection::East => "east",
        EdgeDirection::West => "west",
    }
}

fn fmt(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.3}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

impl From<EdgeDirection> for CardinalDirection {
    fn from(value: EdgeDirection) -> Self {
        match value {
            EdgeDirection::North => CardinalDirection::North,
            EdgeDirection::South => CardinalDirection::South,
            EdgeDirection::East => CardinalDirection::East,
            EdgeDirection::West => CardinalDirection::West,
        }
    }
}
