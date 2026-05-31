use crate::ast::{
    ClearanceSide, DoorSwing, EdgeSide, FloorMaterialLegendMode, Opening, OutdoorAreaKind, Point,
    Position,
};
use crate::lowering::LoweredProgram;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WallSegment {
    pub id: String,
    pub start: Point,
    pub end: Point,
    pub thickness: f64,
    pub is_exterior: bool,
    pub rooms: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpeningPlacementType {
    Door,
    Window,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningPlacement {
    pub id: String,
    #[serde(rename = "type")]
    pub opening_type: OpeningPlacementType,
    pub wall_id: String,
    pub position: f64,
    pub width: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swing: Option<DoorSwing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swing_room: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub double: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_normal: Option<Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sill: Option<f64>,
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedRoom {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub polygon: Polygon,
    pub area: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_material: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub floor_material_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedCourtyard {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub polygon: Polygon,
    pub area: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedOutdoorArea {
    pub kind: OutdoorAreaKind,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub polygon: Polygon,
    pub area: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_material: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub floor_material_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectClearancePlacement {
    pub side: ClearanceSide,
    pub polygon: Polygon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedObject {
    pub name: String,
    pub catalog_id: String,
    pub category: String,
    pub room: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub origin: Point,
    pub facing: EdgeSide,
    pub rotation: f64,
    pub polygon: Polygon,
    pub clearance_polygons: Vec<ObjectClearancePlacement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryIr {
    pub footprint: Polygon,
    pub rooms: Vec<ResolvedRoom>,
    pub courtyards: Vec<ResolvedCourtyard>,
    pub outdoor_areas: Vec<ResolvedOutdoorArea>,
    pub objects: Vec<ResolvedObject>,
    pub walls: Vec<WallSegment>,
    pub openings: Vec<OpeningPlacement>,
    pub floor_material_legend: FloorMaterialLegendMode,
}

const STANDARD_DOOR_WIDTH: f64 = 0.9;
const STANDARD_WINDOW_WIDTH: f64 = 1.2;

#[derive(Debug, Clone)]
struct Edge {
    start: Point,
    end: Point,
    room_name: String,
}

#[derive(Debug, Clone)]
struct WallCandidate {
    start: Point,
    end: Point,
    rooms: Vec<String>,
}

pub fn calculate_polygon_area(points: &[Point]) -> f64 {
    let mut area = 0.0;
    if points.is_empty() {
        return 0.0;
    }
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        area += points[i].x * points[j].y;
        area -= points[j].x * points[i].y;
    }
    area.abs() / 2.0
}

pub fn distance(p1: Point, p2: Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn points_equal(p1: Point, p2: Point, epsilon: f64) -> bool {
    (p1.x - p2.x).abs() < epsilon && (p1.y - p2.y).abs() < epsilon
}

pub fn segments_overlap(
    a1: Point,
    a2: Point,
    b1: Point,
    b2: Point,
    epsilon: f64,
) -> (bool, Option<Point>, Option<Point>) {
    let a_horizontal = (a1.y - a2.y).abs() < epsilon;
    let b_horizontal = (b1.y - b2.y).abs() < epsilon;
    let a_vertical = (a1.x - a2.x).abs() < epsilon;
    let b_vertical = (b1.x - b2.x).abs() < epsilon;

    if a_horizontal && b_horizontal && (a1.y - b1.y).abs() < epsilon {
        let a_min = a1.x.min(a2.x);
        let a_max = a1.x.max(a2.x);
        let b_min = b1.x.min(b2.x);
        let b_max = b1.x.max(b2.x);
        let start = a_min.max(b_min);
        let end = a_max.min(b_max);
        if start < end - epsilon {
            return (
                true,
                Some(Point { x: start, y: a1.y }),
                Some(Point { x: end, y: a1.y }),
            );
        }
    }

    if a_vertical && b_vertical && (a1.x - b1.x).abs() < epsilon {
        let a_min = a1.y.min(a2.y);
        let a_max = a1.y.max(a2.y);
        let b_min = b1.y.min(b2.y);
        let b_max = b1.y.max(b2.y);
        let start = a_min.max(b_min);
        let end = a_max.min(b_max);
        if start < end - epsilon {
            return (
                true,
                Some(Point { x: a1.x, y: start }),
                Some(Point { x: a1.x, y: end }),
            );
        }
    }

    (false, None, None)
}

pub fn generate_geometry(lowered: &LoweredProgram) -> GeometryIr {
    let footprint = Polygon {
        points: lowered.footprint.clone(),
    };

    let rooms: Vec<ResolvedRoom> = lowered
        .rooms
        .iter()
        .map(|room| ResolvedRoom {
            name: room.name.clone(),
            label: room.label.clone(),
            polygon: Polygon {
                points: room.polygon.clone(),
            },
            area: calculate_polygon_area(&room.polygon),
            floor_material: room.floor_material.clone(),
            floor_material_declared: room.floor_material_declared,
        })
        .collect();

    let courtyards: Vec<ResolvedCourtyard> = lowered
        .courtyards
        .iter()
        .map(|courtyard| ResolvedCourtyard {
            name: courtyard.name.clone(),
            label: courtyard.label.clone(),
            polygon: Polygon {
                points: courtyard.polygon.clone(),
            },
            area: calculate_polygon_area(&courtyard.polygon),
        })
        .collect();

    let outdoor_areas: Vec<ResolvedOutdoorArea> = lowered
        .outdoor_areas
        .iter()
        .map(|area| ResolvedOutdoorArea {
            kind: area.kind,
            name: area.name.clone(),
            label: area.label.clone(),
            polygon: Polygon {
                points: area.polygon.clone(),
            },
            area: calculate_polygon_area(&area.polygon),
            floor_material: area.floor_material.clone(),
            floor_material_declared: area.floor_material_declared,
        })
        .collect();

    let objects: Vec<ResolvedObject> = lowered
        .objects
        .iter()
        .map(|object| ResolvedObject {
            name: object.name.clone(),
            catalog_id: object.catalog_id.clone(),
            category: object.category.clone(),
            room: object.room.clone(),
            label: object.label.clone(),
            origin: object.origin,
            facing: object.facing,
            rotation: object.rotation,
            polygon: Polygon {
                points: object.polygon.clone(),
            },
            clearance_polygons: object
                .clearance_polygons
                .iter()
                .map(|clearance| ObjectClearancePlacement {
                    side: clearance.side,
                    polygon: Polygon {
                        points: clearance.polygon.clone(),
                    },
                })
                .collect(),
        })
        .collect();

    let walls = generate_walls(&rooms, &footprint, 0.15);
    let openings = place_openings(&lowered.openings, &walls, &rooms, lowered);

    GeometryIr {
        footprint,
        rooms,
        courtyards,
        outdoor_areas,
        objects,
        walls,
        openings,
        floor_material_legend: lowered.floor_material_legend,
    }
}

fn extract_edges(room: &ResolvedRoom) -> Vec<Edge> {
    let mut edges = Vec::new();
    let n = room.polygon.points.len();
    for i in 0..n {
        let j = (i + 1) % n;
        edges.push(Edge {
            start: room.polygon.points[i],
            end: room.polygon.points[j],
            room_name: room.name.clone(),
        });
    }
    edges
}

fn is_edge_on_footprint(edge: &Edge, footprint: &Polygon) -> bool {
    let n = footprint.points.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let (overlap, _, _) = segments_overlap(
            edge.start,
            edge.end,
            footprint.points[i],
            footprint.points[j],
            1e-10,
        );
        if overlap {
            return true;
        }
    }
    false
}

fn generate_walls(
    rooms: &[ResolvedRoom],
    footprint: &Polygon,
    default_thickness: f64,
) -> Vec<WallSegment> {
    let mut all_edges = Vec::<Edge>::new();
    for room in rooms {
        all_edges.extend(extract_edges(room));
    }

    let mut created = Vec::<WallCandidate>::new();

    for i in 0..all_edges.len() {
        for j in (i + 1)..all_edges.len() {
            let e1 = &all_edges[i];
            let e2 = &all_edges[j];
            if e1.room_name == e2.room_name {
                continue;
            }
            let (overlap, start, end) = segments_overlap(e1.start, e1.end, e2.start, e2.end, 1e-10);
            if overlap {
                let start = start.unwrap();
                let end = end.unwrap();
                if !is_segment_covered(start, end, &created) {
                    created.push(WallCandidate {
                        start,
                        end,
                        rooms: vec![e1.room_name.clone(), e2.room_name.clone()],
                    });
                }
            }
        }
    }

    for edge in &all_edges {
        for segment in get_uncovered_segments(edge.start, edge.end, &created) {
            if !is_segment_covered(segment.start, segment.end, &created) {
                created.push(WallCandidate {
                    start: segment.start,
                    end: segment.end,
                    rooms: vec![edge.room_name.clone()],
                });
            }
        }
    }

    created
        .into_iter()
        .enumerate()
        .map(|(i, wall)| {
            let is_exterior = is_edge_on_footprint(
                &Edge {
                    start: wall.start,
                    end: wall.end,
                    room_name: wall.rooms[0].clone(),
                },
                footprint,
            );
            WallSegment {
                id: format!("wall_{i}"),
                start: wall.start,
                end: wall.end,
                thickness: default_thickness,
                is_exterior,
                rooms: wall.rooms,
            }
        })
        .collect()
}

fn is_segment_covered(start: Point, end: Point, walls: &[WallCandidate]) -> bool {
    for wall in walls {
        let (overlap, s, e) = segments_overlap(start, end, wall.start, wall.end, 1e-10);
        if overlap {
            let overlap_len = distance(s.unwrap(), e.unwrap());
            let segment_len = distance(start, end);
            if (overlap_len - segment_len).abs() < 0.001 {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
}

fn get_uncovered_segments(start: Point, end: Point, existing: &[WallCandidate]) -> Vec<Segment> {
    let is_horizontal = (start.y - end.y).abs() < 0.001;
    let is_vertical = (start.x - end.x).abs() < 0.001;
    if !is_horizontal && !is_vertical {
        return vec![Segment { start, end }];
    }

    let mut overlaps = Vec::<(f64, f64)>::new();
    for wall in existing {
        let (overlap, s, e) = segments_overlap(start, end, wall.start, wall.end, 1e-10);
        if overlap {
            let s = s.unwrap();
            let e = e.unwrap();
            if is_horizontal {
                overlaps.push((s.x.min(e.x), s.x.max(e.x)));
            } else {
                overlaps.push((s.y.min(e.y), s.y.max(e.y)));
            }
        }
    }
    overlaps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut merged = Vec::<(f64, f64)>::new();
    for (from, to) in overlaps {
        if merged.last().is_none_or(|last| last.1 < from - 0.001) {
            merged.push((from, to));
        } else if let Some(last) = merged.last_mut() {
            last.1 = last.1.max(to);
        }
    }

    let mut result = Vec::new();
    let mut current = if is_horizontal {
        start.x.min(end.x)
    } else {
        start.y.min(end.y)
    };
    let seg_end = if is_horizontal {
        start.x.max(end.x)
    } else {
        start.y.max(end.y)
    };
    let fixed = if is_horizontal { start.y } else { start.x };

    for (from, to) in merged {
        if current < from - 0.001 {
            result.push(if is_horizontal {
                Segment {
                    start: Point {
                        x: current,
                        y: fixed,
                    },
                    end: Point { x: from, y: fixed },
                }
            } else {
                Segment {
                    start: Point {
                        x: fixed,
                        y: current,
                    },
                    end: Point { x: fixed, y: from },
                }
            });
        }
        current = current.max(to);
    }

    if current < seg_end - 0.001 {
        result.push(if is_horizontal {
            Segment {
                start: Point {
                    x: current,
                    y: fixed,
                },
                end: Point {
                    x: seg_end,
                    y: fixed,
                },
            }
        } else {
            Segment {
                start: Point {
                    x: fixed,
                    y: current,
                },
                end: Point {
                    x: fixed,
                    y: seg_end,
                },
            }
        });
    }

    result
}

fn resolve_position(position: &Position, wall_length: f64) -> f64 {
    match position {
        Position::Percentage { value } => (value / 100.0) * wall_length,
        Position::Absolute { value } => *value,
    }
}

fn place_openings(
    openings: &[Opening],
    walls: &[WallSegment],
    rooms: &[ResolvedRoom],
    lowered: &LoweredProgram,
) -> Vec<OpeningPlacement> {
    let mut placements = Vec::new();

    for opening in openings {
        match opening {
            Opening::DoorOpening(door) => {
                if let Some([room1, room2]) = &door.between {
                    if let Some(shared) = walls.iter().find(|w| {
                        w.rooms.iter().any(|r| r == room1) && w.rooms.iter().any(|r| r == room2)
                    }) {
                        let len = distance(shared.start, shared.end);
                        placements.push(OpeningPlacement {
                            id: door.name.clone(),
                            opening_type: OpeningPlacementType::Door,
                            wall_id: shared.id.clone(),
                            position: resolve_position(&door.at, len),
                            width: resolve_door_width(door.width, door.double, lowered),
                            swing: door.swing,
                            swing_room: door.swing_room.clone(),
                            double: door.double,
                            outside_normal: room_side_normal(shared, rooms, room1),
                            sill: None,
                        });
                    }
                } else if let (Some(room_name), Some(edge)) = (&door.room, door.edge) {
                    if let Some(target) = find_wall_on_room_edge(walls, rooms, room_name, edge) {
                        let len = distance(target.start, target.end);
                        placements.push(OpeningPlacement {
                            id: door.name.clone(),
                            opening_type: OpeningPlacementType::Door,
                            wall_id: target.id.clone(),
                            position: resolve_position(&door.at, len),
                            width: resolve_door_width(door.width, door.double, lowered),
                            swing: door.swing,
                            swing_room: door.swing_room.clone(),
                            double: door.double,
                            outside_normal: Some(edge_outside_normal(edge)),
                            sill: None,
                        });
                    }
                }
            }
            Opening::WindowOpening(window) => {
                if let Some(target) =
                    find_wall_on_room_edge(walls, rooms, &window.room, window.edge)
                {
                    let len = distance(target.start, target.end);
                    placements.push(OpeningPlacement {
                        id: window.name.clone(),
                        opening_type: OpeningPlacementType::Window,
                        wall_id: target.id.clone(),
                        position: resolve_position(&window.at, len),
                        width: window.width.unwrap_or(
                            lowered
                                .defaults
                                .window_width
                                .unwrap_or(STANDARD_WINDOW_WIDTH),
                        ),
                        swing: None,
                        swing_room: None,
                        double: false,
                        outside_normal: Some(edge_outside_normal(window.edge)),
                        sill: window.sill,
                    });
                }
            }
        }
    }

    placements
}

fn resolve_door_width(width: Option<f64>, double: bool, lowered: &LoweredProgram) -> f64 {
    let standard = lowered.defaults.door_width.unwrap_or(STANDARD_DOOR_WIDTH);
    width.unwrap_or(if double { standard * 2.0 } else { standard })
}

fn edge_outside_normal(edge: EdgeSide) -> Point {
    match edge {
        EdgeSide::North => Point { x: 0.0, y: 1.0 },
        EdgeSide::South => Point { x: 0.0, y: -1.0 },
        EdgeSide::East => Point { x: 1.0, y: 0.0 },
        EdgeSide::West => Point { x: -1.0, y: 0.0 },
    }
}

fn room_side_normal(wall: &WallSegment, rooms: &[ResolvedRoom], room_name: &str) -> Option<Point> {
    let room = rooms.iter().find(|room| room.name == room_name)?;
    let center = polygon_center(&room.polygon.points)?;
    let mid = Point {
        x: (wall.start.x + wall.end.x) / 2.0,
        y: (wall.start.y + wall.end.y) / 2.0,
    };
    let wall_vector = Point {
        x: wall.end.x - wall.start.x,
        y: wall.end.y - wall.start.y,
    };
    let normal = normalize(Point {
        x: -wall_vector.y,
        y: wall_vector.x,
    })?;
    let toward_room = Point {
        x: center.x - mid.x,
        y: center.y - mid.y,
    };
    if dot(normal, toward_room) >= 0.0 {
        Some(normal)
    } else {
        Some(Point {
            x: -normal.x,
            y: -normal.y,
        })
    }
}

fn polygon_center(points: &[Point]) -> Option<Point> {
    if points.is_empty() {
        return None;
    }
    let sum = points
        .iter()
        .fold(Point { x: 0.0, y: 0.0 }, |acc, p| Point {
            x: acc.x + p.x,
            y: acc.y + p.y,
        });
    Some(Point {
        x: sum.x / points.len() as f64,
        y: sum.y / points.len() as f64,
    })
}

fn normalize(vector: Point) -> Option<Point> {
    let length = (vector.x * vector.x + vector.y * vector.y).sqrt();
    if length <= 1e-10 {
        None
    } else {
        Some(Point {
            x: vector.x / length,
            y: vector.y / length,
        })
    }
}

fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

fn find_wall_on_room_edge<'a>(
    walls: &'a [WallSegment],
    rooms: &[ResolvedRoom],
    room_name: &str,
    edge: crate::ast::EdgeSide,
) -> Option<&'a WallSegment> {
    let room = rooms.iter().find(|r| r.name == room_name)?;
    let bounds = bounds(&room.polygon.points);
    for wall in walls {
        if !wall.rooms.iter().any(|r| r == room_name) {
            continue;
        }
        let mid_x = (wall.start.x + wall.end.x) / 2.0;
        let mid_y = (wall.start.y + wall.end.y) / 2.0;
        let is_horizontal = (wall.start.y - wall.end.y).abs() < 1e-10;
        let is_vertical = (wall.start.x - wall.end.x).abs() < 1e-10;
        match edge {
            crate::ast::EdgeSide::South if is_horizontal && (mid_y - bounds.2).abs() < 1e-10 => {
                return Some(wall)
            }
            crate::ast::EdgeSide::North if is_horizontal && (mid_y - bounds.3).abs() < 1e-10 => {
                return Some(wall)
            }
            crate::ast::EdgeSide::West if is_vertical && (mid_x - bounds.0).abs() < 1e-10 => {
                return Some(wall)
            }
            crate::ast::EdgeSide::East if is_vertical && (mid_x - bounds.1).abs() < 1e-10 => {
                return Some(wall)
            }
            _ => {}
        }
    }
    None
}

fn bounds(points: &[Point]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for p in points {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }
    (min_x, max_x, min_y, max_y)
}
