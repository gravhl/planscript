use crate::ast::*;
use crate::catalog::{prefers_wall_placement, Catalog, CatalogItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoweredRoom {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub polygon: Vec<Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_material: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub floor_material_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoweredCourtyard {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub polygon: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoweredOutdoorArea {
    pub kind: OutdoorAreaKind,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub polygon: Vec<Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_material: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub floor_material_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoweredObjectClearance {
    pub side: ClearanceSide,
    pub polygon: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoweredObject {
    pub name: String,
    pub catalog_id: String,
    pub category: String,
    pub room: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub origin: Point,
    pub facing: EdgeSide,
    pub rotation: f64,
    pub polygon: Vec<Point>,
    pub clearance_polygons: Vec<LoweredObjectClearance>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outdoor_floor: Option<String>,
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteInfo {
    pub street: CardinalDirection,
    pub hemisphere: Hemisphere,
    pub back: CardinalDirection,
    pub morning_sun: CardinalDirection,
    pub afternoon_sun: CardinalDirection,
    pub good_sun: CardinalDirection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoweredProgram {
    pub name: String,
    pub footprint: Vec<Point>,
    pub rooms: Vec<LoweredRoom>,
    pub courtyards: Vec<LoweredCourtyard>,
    pub outdoor_areas: Vec<LoweredOutdoorArea>,
    pub objects: Vec<LoweredObject>,
    pub openings: Vec<Opening>,
    pub wall_overrides: Vec<WallThicknessOverride>,
    pub assertions: Vec<Assertion>,
    pub defaults: Defaults,
    pub floor_material_legend: FloorMaterialLegendMode,
    pub render_mode: RenderMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<SiteInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoweringError {
    pub message: String,
    pub room_name: Option<String>,
}

impl LoweringError {
    fn new(message: impl Into<String>, room_name: impl Into<Option<String>>) -> Self {
        Self {
            message: message.into(),
            room_name: room_name.into(),
        }
    }
}

impl fmt::Display for LoweringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LoweringError {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Bounds {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct ZoneBounds {
    bounds: Bounds,
}

pub fn lower(program: &Program) -> Result<LoweredProgram, LoweringError> {
    let catalog = Catalog::builtins();
    lower_with_catalog(program, &catalog)
}

pub fn lower_with_catalog(
    program: &Program,
    catalog: &Catalog,
) -> Result<LoweredProgram, LoweringError> {
    let plan = &program.plan;
    let footprint = lower_footprint(&plan.footprint);
    let default_floor = program.defaults.as_ref().and_then(|d| d.floor.clone());
    let default_outdoor_floor = program
        .defaults
        .as_ref()
        .and_then(|d| d.outdoor_floor.clone());
    let mut resolved = HashMap::<String, LoweredRoom>::new();
    let mut ordered = Vec::<LoweredRoom>::new();
    let mut zone_bounds = HashMap::<String, ZoneBounds>::new();

    for room in &plan.rooms {
        let polygon = lower_room_geometry(room, &resolved)?;
        let lowered = LoweredRoom {
            name: room.name.clone(),
            label: room.label.clone(),
            polygon,
            zone: None,
            floor_material: room.floor.clone().or_else(|| default_floor.clone()),
            floor_material_declared: room.floor.is_some() || default_floor.is_some(),
        };
        resolved.insert(lowered.name.clone(), lowered.clone());
        ordered.push(lowered);
    }

    for zone in &plan.zones {
        let zone_rooms = lower_zone(zone, &resolved, &mut zone_bounds, default_floor.as_deref())?;
        for room in zone_rooms {
            resolved.insert(room.name.clone(), room.clone());
            ordered.push(room);
        }
    }

    let courtyards = plan.courtyards.iter().map(lower_courtyard).collect();
    let outdoor_areas = plan
        .outdoor_areas
        .iter()
        .map(|area| lower_outdoor_area(area, default_outdoor_floor.as_deref()))
        .collect();
    let objects = plan
        .objects
        .iter()
        .map(|object| lower_object(object, &resolved, catalog))
        .collect::<Result<Vec<_>, _>>()?;

    let defaults = Defaults {
        door_width: program.defaults.as_ref().and_then(|d| d.door_width),
        window_width: program.defaults.as_ref().and_then(|d| d.window_width),
        floor: default_floor,
        outdoor_floor: default_outdoor_floor,
    };

    let site = program
        .site
        .as_ref()
        .map(|site| derive_site_info(site.street, site.hemisphere.unwrap_or(Hemisphere::North)));

    Ok(LoweredProgram {
        name: plan.name.clone(),
        footprint,
        rooms: ordered,
        courtyards,
        outdoor_areas,
        objects,
        openings: plan.openings.clone(),
        wall_overrides: plan.wall_overrides.clone(),
        assertions: plan.assertions.clone(),
        defaults,
        floor_material_legend: plan
            .legend
            .as_ref()
            .map(|legend| legend.floor_materials)
            .unwrap_or(FloorMaterialLegendMode::Auto),
        render_mode: plan
            .render
            .as_ref()
            .or(program.render.as_ref())
            .map(|render| render.mode)
            .unwrap_or(RenderMode::Color),
        site,
    })
}

fn lower_footprint(footprint: &Footprint) -> Vec<Point> {
    match footprint {
        Footprint::FootprintPolygon { points } => points.clone(),
        Footprint::FootprintRect { p1, p2 } => rect_diagonal_to_polygon(*p1, *p2),
    }
}

fn rect_diagonal_to_polygon(p1: Point, p2: Point) -> Vec<Point> {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);
    vec![
        Point { x: min_x, y: min_y },
        Point { x: max_x, y: min_y },
        Point { x: max_x, y: max_y },
        Point { x: min_x, y: max_y },
    ]
}

fn rect_at_size_to_polygon(at: Point, size: Point) -> Vec<Point> {
    vec![
        at,
        Point {
            x: at.x + size.x,
            y: at.y,
        },
        Point {
            x: at.x + size.x,
            y: at.y + size.y,
        },
        Point {
            x: at.x,
            y: at.y + size.y,
        },
    ]
}

fn rect_center_size_to_polygon(center: Point, size: Point) -> Vec<Point> {
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;
    vec![
        Point {
            x: center.x - half_w,
            y: center.y - half_h,
        },
        Point {
            x: center.x + half_w,
            y: center.y - half_h,
        },
        Point {
            x: center.x + half_w,
            y: center.y + half_h,
        },
        Point {
            x: center.x - half_w,
            y: center.y + half_h,
        },
    ]
}

fn get_bounds(points: &[Point]) -> Bounds {
    let mut bounds = Bounds {
        min_x: f64::INFINITY,
        max_x: f64::NEG_INFINITY,
        min_y: f64::INFINITY,
        max_y: f64::NEG_INFINITY,
    };
    for p in points {
        bounds.min_x = bounds.min_x.min(p.x);
        bounds.max_x = bounds.max_x.max(p.x);
        bounds.min_y = bounds.min_y.min(p.y);
        bounds.max_y = bounds.max_y.max(p.y);
    }
    bounds
}

fn get_room_bounds(room: &LoweredRoom) -> Bounds {
    get_bounds(&room.polygon)
}

fn get_room_edge(room: &LoweredRoom, edge: EdgeRefSide) -> f64 {
    let bounds = get_room_bounds(room);
    match edge {
        EdgeRefSide::Left => bounds.min_x,
        EdgeRefSide::Right => bounds.max_x,
        EdgeRefSide::Top => bounds.max_y,
        EdgeRefSide::Bottom => bounds.min_y,
    }
}

fn resolve_size(
    size: &SizeValue,
    room: &RoomDefinition,
    target: Bounds,
    resolved: &HashMap<String, LoweredRoom>,
) -> Result<Point, LoweringError> {
    let x = if size.x.is_auto() {
        if let Some(ext) = &room.extend {
            if ext.axis == Axis::X {
                let from = resolved.get(&ext.from.room).ok_or_else(|| {
                    LoweringError::new(
                        format!("Room \"{}\" extend references unknown room", room.name),
                        Some(room.name.clone()),
                    )
                })?;
                let to = resolved.get(&ext.to.room).ok_or_else(|| {
                    LoweringError::new(
                        format!("Room \"{}\" extend references unknown room", room.name),
                        Some(room.name.clone()),
                    )
                })?;
                (get_room_edge(from, ext.from.edge) - get_room_edge(to, ext.to.edge)).abs()
            } else {
                target.max_x - target.min_x
            }
        } else {
            target.max_x - target.min_x
        }
    } else {
        size.x.as_number().unwrap()
    };

    let y = if size.y.is_auto() {
        if let Some(ext) = &room.extend {
            if ext.axis == Axis::Y {
                let from = resolved.get(&ext.from.room).ok_or_else(|| {
                    LoweringError::new(
                        format!("Room \"{}\" extend references unknown room", room.name),
                        Some(room.name.clone()),
                    )
                })?;
                let to = resolved.get(&ext.to.room).ok_or_else(|| {
                    LoweringError::new(
                        format!("Room \"{}\" extend references unknown room", room.name),
                        Some(room.name.clone()),
                    )
                })?;
                (get_room_edge(from, ext.from.edge) - get_room_edge(to, ext.to.edge)).abs()
            } else {
                target.max_y - target.min_y
            }
        } else {
            target.max_y - target.min_y
        }
    } else {
        size.y.as_number().unwrap()
    };

    Ok(Point { x, y })
}

fn lower_room_geometry(
    room: &RoomDefinition,
    resolved: &HashMap<String, LoweredRoom>,
) -> Result<Vec<Point>, LoweringError> {
    match &room.geometry {
        RoomGeometry::RoomPolygon { points } => Ok(points.clone()),
        RoomGeometry::RoomRectDiagonal { p1, p2 } => Ok(rect_diagonal_to_polygon(*p1, *p2)),
        RoomGeometry::RoomRectAtSize { at, size } => Ok(rect_at_size_to_polygon(*at, *size)),
        RoomGeometry::RoomRectCenterSize { center, size } => {
            Ok(rect_center_size_to_polygon(*center, *size))
        }
        RoomGeometry::RoomRectSizeOnly { size } => {
            let attach = room.attach.as_ref().ok_or_else(|| {
                LoweringError::new(
                    format!(
                        "Room \"{}\" uses rect size() but has no attach directive",
                        room.name
                    ),
                    Some(room.name.clone()),
                )
            })?;
            let target_room = resolved.get(&attach.target).ok_or_else(|| {
                LoweringError::new(
                    format!(
                        "Room \"{}\" attaches to unknown room \"{}\"",
                        room.name, attach.target
                    ),
                    Some(room.name.clone()),
                )
            })?;
            let target = get_room_bounds(target_room);
            let resolved_size = resolve_size(size, room, target, resolved)?;
            let gap = room.gap.as_ref().map_or(0.0, |g| g.distance);

            let mut x = match attach.direction {
                RelativeDirection::EastOf => target.max_x + gap,
                RelativeDirection::WestOf => target.min_x - resolved_size.x - gap,
                RelativeDirection::NorthOf | RelativeDirection::SouthOf => target.min_x,
            };
            let mut y = match attach.direction {
                RelativeDirection::NorthOf => target.max_y + gap,
                RelativeDirection::SouthOf => target.min_y - resolved_size.y - gap,
                RelativeDirection::EastOf | RelativeDirection::WestOf => target.min_y,
            };

            if let Some(align) = &room.align {
                if let Some((my_edge, with_room, with_edge)) = align.explicit_parts() {
                    let with = resolved.get(with_room).ok_or_else(|| {
                        LoweringError::new(
                            format!(
                                "Room \"{}\" aligns with unknown room \"{}\"",
                                room.name, with_room
                            ),
                            Some(room.name.clone()),
                        )
                    })?;
                    let with_value = get_room_edge(with, match_align_edge(with_edge));
                    match my_edge {
                        AlignEdge::Left => x = with_value,
                        AlignEdge::Right => x = with_value - resolved_size.x,
                        AlignEdge::Top => y = with_value - resolved_size.y,
                        AlignEdge::Bottom => y = with_value,
                    }
                } else {
                    apply_simple_alignment(
                        attach.direction,
                        align.alignment().unwrap_or(AlignmentType::Center),
                        target,
                        resolved_size,
                        &mut x,
                        &mut y,
                    );
                }
            } else {
                apply_simple_alignment(
                    attach.direction,
                    AlignmentType::Center,
                    target,
                    resolved_size,
                    &mut x,
                    &mut y,
                );
            }

            Ok(rect_at_size_to_polygon(Point { x, y }, resolved_size))
        }
        RoomGeometry::RoomFill {
            between,
            width,
            height,
        } => {
            let room1 = resolved.get(&between[0]).ok_or_else(|| {
                LoweringError::new(
                    format!(
                        "Room \"{}\" references unknown room \"{}\"",
                        room.name, between[0]
                    ),
                    Some(room.name.clone()),
                )
            })?;
            let room2 = resolved.get(&between[1]).ok_or_else(|| {
                LoweringError::new(
                    format!(
                        "Room \"{}\" references unknown room \"{}\"",
                        room.name, between[1]
                    ),
                    Some(room.name.clone()),
                )
            })?;
            let b1 = get_room_bounds(room1);
            let b2 = get_room_bounds(room2);
            let horizontal_gap = b1.min_x.max(b2.min_x) - b1.max_x.min(b2.max_x);
            let vertical_gap = b1.min_y.max(b2.min_y) - b1.max_y.min(b2.max_y);

            let (mut min_x, mut max_x, mut min_y, mut max_y);
            if horizontal_gap > 0.0 {
                min_x = b1.max_x.min(b2.max_x);
                max_x = b1.min_x.max(b2.min_x);
                min_y = b1.min_y.max(b2.min_y);
                max_y = b1.max_y.min(b2.max_y);
            } else if vertical_gap > 0.0 {
                min_y = b1.max_y.min(b2.max_y);
                max_y = b1.min_y.max(b2.min_y);
                min_x = b1.min_x.max(b2.min_x);
                max_x = b1.max_x.min(b2.max_x);
            } else {
                min_x = b1.min_x.min(b2.min_x);
                max_x = b1.max_x.max(b2.max_x);
                min_y = b1.min_y.min(b2.min_y);
                max_y = b1.max_y.max(b2.max_y);
            }

            if let Some(w) = width {
                let cx = (min_x + max_x) / 2.0;
                min_x = cx - w / 2.0;
                max_x = cx + w / 2.0;
            }
            if let Some(h) = height {
                let cy = (min_y + max_y) / 2.0;
                min_y = cy - h / 2.0;
                max_y = cy + h / 2.0;
            }
            Ok(rect_diagonal_to_polygon(
                Point { x: min_x, y: min_y },
                Point { x: max_x, y: max_y },
            ))
        }
        RoomGeometry::RoomRectSpan { span_x, span_y } => {
            let from_room = resolved.get(&span_x.from.room).ok_or_else(|| {
                LoweringError::new(
                    format!(
                        "Room \"{}\" references unknown room \"{}\"",
                        room.name, span_x.from.room
                    ),
                    Some(room.name.clone()),
                )
            })?;
            let to_room = resolved.get(&span_x.to.room).ok_or_else(|| {
                LoweringError::new(
                    format!(
                        "Room \"{}\" references unknown room \"{}\"",
                        room.name, span_x.to.room
                    ),
                    Some(room.name.clone()),
                )
            })?;
            Ok(rect_diagonal_to_polygon(
                Point {
                    x: get_room_edge(from_room, span_x.from.edge),
                    y: span_y.from,
                },
                Point {
                    x: get_room_edge(to_room, span_x.to.edge),
                    y: span_y.to,
                },
            ))
        }
    }
}

fn match_align_edge(edge: AlignEdge) -> EdgeRefSide {
    match edge {
        AlignEdge::Top => EdgeRefSide::Top,
        AlignEdge::Bottom => EdgeRefSide::Bottom,
        AlignEdge::Left => EdgeRefSide::Left,
        AlignEdge::Right => EdgeRefSide::Right,
    }
}

fn apply_simple_alignment(
    direction: RelativeDirection,
    alignment: AlignmentType,
    target: Bounds,
    size: Point,
    x: &mut f64,
    y: &mut f64,
) {
    if matches!(
        direction,
        RelativeDirection::EastOf | RelativeDirection::WestOf
    ) {
        match alignment {
            AlignmentType::Top => *y = target.max_y - size.y,
            AlignmentType::Bottom => *y = target.min_y,
            AlignmentType::Center => *y = (target.min_y + target.max_y) / 2.0 - size.y / 2.0,
            _ => *y = target.min_y,
        }
    } else {
        match alignment {
            AlignmentType::Left => *x = target.min_x,
            AlignmentType::Right => *x = target.max_x - size.x,
            AlignmentType::Center => *x = (target.min_x + target.max_x) / 2.0 - size.x / 2.0,
            _ => *x = target.min_x,
        }
    }
}

fn lower_zone(
    zone: &ZoneDefinition,
    resolved: &HashMap<String, LoweredRoom>,
    zone_bounds: &mut HashMap<String, ZoneBounds>,
    default_floor: Option<&str>,
) -> Result<Vec<LoweredRoom>, LoweringError> {
    let mut local = HashMap::<String, LoweredRoom>::new();
    let mut local_order = Vec::new();
    for room in &zone.rooms {
        let polygon = lower_room_geometry(room, &local)?;
        let lowered = LoweredRoom {
            name: room.name.clone(),
            label: room.label.clone(),
            polygon,
            zone: Some(zone.name.clone()),
            floor_material: room
                .floor
                .clone()
                .or_else(|| default_floor.map(str::to_string)),
            floor_material_declared: room.floor.is_some() || default_floor.is_some(),
        };
        local.insert(lowered.name.clone(), lowered.clone());
        local_order.push(lowered);
    }

    let local_bounds = calculate_zone_bounds(&local_order);
    let zone_width = local_bounds.max_x - local_bounds.min_x;
    let zone_height = local_bounds.max_y - local_bounds.min_y;
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    if let Some(attach) = &zone.attach {
        let target_bounds = if let Some(target_zone) = zone_bounds.get(&attach.target) {
            target_zone.bounds
        } else if let Some(target_room) = resolved.get(&attach.target) {
            get_room_bounds(target_room)
        } else {
            return Err(LoweringError::new(
                format!(
                    "Zone \"{}\" attaches to unknown zone or room \"{}\"",
                    zone.name, attach.target
                ),
                Some(zone.name.clone()),
            ));
        };

        let gap = zone.gap.as_ref().map_or(0.0, |g| g.distance);
        match attach.direction {
            RelativeDirection::EastOf => offset_x = target_bounds.max_x + gap - local_bounds.min_x,
            RelativeDirection::WestOf => {
                offset_x = target_bounds.min_x - gap - zone_width - local_bounds.min_x
            }
            RelativeDirection::NorthOf => offset_y = target_bounds.max_y + gap - local_bounds.min_y,
            RelativeDirection::SouthOf => {
                offset_y = target_bounds.min_y - gap - zone_height - local_bounds.min_y
            }
        }

        if let Some(align) = &zone.align {
            if let Some(alignment) = align.alignment() {
                if matches!(
                    attach.direction,
                    RelativeDirection::EastOf | RelativeDirection::WestOf
                ) {
                    match alignment {
                        AlignmentType::Top => {
                            offset_y = target_bounds.max_y - zone_height - local_bounds.min_y
                        }
                        AlignmentType::Bottom => {
                            offset_y = target_bounds.min_y - local_bounds.min_y
                        }
                        AlignmentType::Center => {
                            offset_y = (target_bounds.min_y + target_bounds.max_y) / 2.0
                                - zone_height / 2.0
                                - local_bounds.min_y
                        }
                        _ => {}
                    }
                } else {
                    match alignment {
                        AlignmentType::Left => offset_x = target_bounds.min_x - local_bounds.min_x,
                        AlignmentType::Right => {
                            offset_x = target_bounds.max_x - zone_width - local_bounds.min_x
                        }
                        AlignmentType::Center => {
                            offset_x = (target_bounds.min_x + target_bounds.max_x) / 2.0
                                - zone_width / 2.0
                                - local_bounds.min_x
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let translated: Vec<LoweredRoom> = local_order
        .into_iter()
        .map(|mut room| {
            room.polygon = translate_polygon(
                &room.polygon,
                Point {
                    x: offset_x,
                    y: offset_y,
                },
            );
            room
        })
        .collect();

    zone_bounds.insert(
        zone.name.clone(),
        ZoneBounds {
            bounds: calculate_zone_bounds(&translated),
        },
    );

    Ok(translated)
}

fn translate_polygon(points: &[Point], offset: Point) -> Vec<Point> {
    points
        .iter()
        .map(|p| Point {
            x: p.x + offset.x,
            y: p.y + offset.y,
        })
        .collect()
}

fn calculate_zone_bounds(rooms: &[LoweredRoom]) -> Bounds {
    if rooms.is_empty() {
        return Bounds {
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
        };
    }
    let mut out = Bounds {
        min_x: f64::INFINITY,
        max_x: f64::NEG_INFINITY,
        min_y: f64::INFINITY,
        max_y: f64::NEG_INFINITY,
    };
    for room in rooms {
        let b = get_bounds(&room.polygon);
        out.min_x = out.min_x.min(b.min_x);
        out.max_x = out.max_x.max(b.max_x);
        out.min_y = out.min_y.min(b.min_y);
        out.max_y = out.max_y.max(b.max_y);
    }
    out
}

fn lower_courtyard(courtyard: &CourtyardDefinition) -> LoweredCourtyard {
    let polygon = match &courtyard.geometry {
        CourtyardGeometry::CourtyardRect { p1, p2 } => rect_diagonal_to_polygon(*p1, *p2),
        CourtyardGeometry::CourtyardPolygon { points } => points.clone(),
    };
    LoweredCourtyard {
        name: courtyard.name.clone(),
        label: courtyard.label.clone(),
        polygon,
    }
}

fn lower_outdoor_area(
    area: &OutdoorAreaDefinition,
    default_outdoor_floor: Option<&str>,
) -> LoweredOutdoorArea {
    let polygon = match &area.geometry {
        OutdoorGeometry::OutdoorPolygon { points } => points.clone(),
        OutdoorGeometry::OutdoorRect { p1, p2 } => rect_diagonal_to_polygon(*p1, *p2),
        OutdoorGeometry::OutdoorRectAtSize { at, size } => rect_at_size_to_polygon(*at, *size),
        OutdoorGeometry::OutdoorRectCenterSize { center, size } => {
            rect_center_size_to_polygon(*center, *size)
        }
    };
    LoweredOutdoorArea {
        kind: area.kind,
        name: area.name.clone(),
        label: area.label.clone(),
        polygon,
        floor_material: area
            .floor
            .clone()
            .or_else(|| default_outdoor_floor.map(str::to_string)),
        floor_material_declared: area.floor.is_some() || default_outdoor_floor.is_some(),
    }
}

fn lower_object(
    object: &ObjectDefinition,
    rooms: &HashMap<String, LoweredRoom>,
    catalog: &Catalog,
) -> Result<LoweredObject, LoweringError> {
    let room = rooms.get(&object.room).ok_or_else(|| {
        LoweringError::new(
            format!(
                "Object \"{}\" references unknown room \"{}\"",
                object.name, object.room
            ),
            Some(object.room.clone()),
        )
    })?;
    let item = catalog.get(&object.catalog_id).ok_or_else(|| {
        LoweringError::new(
            format!(
                "Object \"{}\" references unknown catalog item \"{}\"",
                object.name, object.catalog_id
            ),
            Some(object.name.clone()),
        )
    })?;

    let room_bounds = get_room_bounds(room);
    let facing = object
        .facing
        .or_else(|| object.attach.as_ref().map(|a| opposite_edge(a.edge)))
        .unwrap_or(EdgeSide::North);
    let base_rotation = facing_rotation(facing);
    let rotation = normalize_degrees(base_rotation + object.rotate.unwrap_or(0.0));
    let origin = object_origin(object, item, room_bounds, facing)?;

    let mut local = item.footprint.clone();
    if let Some(axis) = object.mirror {
        mirror_polygon(&mut local, axis);
    }
    let polygon = transform_local_polygon(&local, origin, rotation);

    let mut clearance_values = item.clearances.clone();
    for override_item in &object.clearance_overrides {
        clearance_values.insert(override_item.side, override_item.value);
    }
    let mut clearance_values = clearance_values.into_iter().collect::<Vec<_>>();
    clearance_values.sort_by_key(|(side, _)| clearance_side_order(*side));
    let clearance_polygons = clearance_values
        .into_iter()
        .filter(|(_, value)| *value > 0.0)
        .map(|(side, value)| {
            let mut local_clearance = clearance_polygon(item, side, value);
            if let Some(axis) = object.mirror {
                mirror_polygon(&mut local_clearance, axis);
            }
            LoweredObjectClearance {
                side,
                polygon: transform_local_polygon(&local_clearance, origin, rotation),
            }
        })
        .collect();

    Ok(LoweredObject {
        name: object.name.clone(),
        catalog_id: object.catalog_id.clone(),
        category: item.category.clone(),
        room: object.room.clone(),
        label: object.label.clone().or_else(|| Some(item.name.clone())),
        origin,
        facing,
        rotation,
        polygon,
        clearance_polygons,
    })
}

fn clearance_side_order(side: ClearanceSide) -> u8 {
    match side {
        ClearanceSide::Front => 0,
        ClearanceSide::Back => 1,
        ClearanceSide::Left => 2,
        ClearanceSide::Right => 3,
    }
}

fn object_origin(
    object: &ObjectDefinition,
    item: &CatalogItem,
    room_bounds: Bounds,
    facing: EdgeSide,
) -> Result<Point, LoweringError> {
    if let Some(position) = &object.at {
        match position {
            ObjectPosition::Point { point } => return Ok(*point),
            ObjectPosition::Distance { position } => {
                let attach = object.attach.as_ref().ok_or_else(|| {
                    LoweringError::new(
                        format!(
                            "Object \"{}\" uses distance placement but has no wall attachment",
                            object.name
                        ),
                        Some(object.name.clone()),
                    )
                })?;
                return Ok(point_on_room_edge(room_bounds, attach.edge, position));
            }
        }
    }

    if let Some(attach) = &object.attach {
        let default_position = Position::Percentage { value: 50.0 };
        return Ok(point_on_room_edge(
            room_bounds,
            attach.edge,
            &default_position,
        ));
    }

    if prefers_wall_placement(item) {
        let default_position = Position::Percentage { value: 50.0 };
        return Ok(point_on_room_edge(
            room_bounds,
            opposite_edge(facing),
            &default_position,
        ));
    }

    let center = Point {
        x: (room_bounds.min_x + room_bounds.max_x) / 2.0,
        y: (room_bounds.min_y + room_bounds.max_y) / 2.0,
    };
    let local_center = Point {
        x: 0.0,
        y: item.size.depth / 2.0,
    };
    let rotated_center = rotate_point(local_center, facing_rotation(facing));
    Ok(Point {
        x: center.x - rotated_center.x,
        y: center.y - rotated_center.y,
    })
}

fn point_on_room_edge(bounds: Bounds, edge: EdgeSide, position: &Position) -> Point {
    match edge {
        EdgeSide::South => {
            let len = bounds.max_x - bounds.min_x;
            Point {
                x: bounds.min_x + resolve_position(position, len),
                y: bounds.min_y,
            }
        }
        EdgeSide::North => {
            let len = bounds.max_x - bounds.min_x;
            Point {
                x: bounds.min_x + resolve_position(position, len),
                y: bounds.max_y,
            }
        }
        EdgeSide::West => {
            let len = bounds.max_y - bounds.min_y;
            Point {
                x: bounds.min_x,
                y: bounds.min_y + resolve_position(position, len),
            }
        }
        EdgeSide::East => {
            let len = bounds.max_y - bounds.min_y;
            Point {
                x: bounds.max_x,
                y: bounds.min_y + resolve_position(position, len),
            }
        }
    }
}

fn resolve_position(position: &Position, length: f64) -> f64 {
    match position {
        Position::Percentage { value } => value / 100.0 * length,
        Position::Absolute { value } => *value,
    }
}

fn opposite_edge(edge: EdgeSide) -> EdgeSide {
    match edge {
        EdgeSide::North => EdgeSide::South,
        EdgeSide::South => EdgeSide::North,
        EdgeSide::East => EdgeSide::West,
        EdgeSide::West => EdgeSide::East,
    }
}

fn facing_rotation(facing: EdgeSide) -> f64 {
    match facing {
        EdgeSide::North => 0.0,
        EdgeSide::East => -90.0,
        EdgeSide::South => 180.0,
        EdgeSide::West => 90.0,
    }
}

fn normalize_degrees(value: f64) -> f64 {
    let mut normalized = value % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}

fn transform_local_polygon(points: &[Point], origin: Point, rotation_degrees: f64) -> Vec<Point> {
    points
        .iter()
        .map(|point| {
            let rotated = rotate_point(*point, rotation_degrees);
            Point {
                x: rotated.x + origin.x,
                y: rotated.y + origin.y,
            }
        })
        .collect()
}

fn rotate_point(point: Point, rotation_degrees: f64) -> Point {
    let radians = rotation_degrees.to_radians();
    let cos = radians.cos();
    let sin = radians.sin();
    Point {
        x: point.x * cos - point.y * sin,
        y: point.x * sin + point.y * cos,
    }
}

fn mirror_polygon(points: &mut [Point], axis: MirrorAxis) {
    for point in points {
        match axis {
            MirrorAxis::X => point.y = -point.y,
            MirrorAxis::Y => point.x = -point.x,
        }
    }
}

fn clearance_polygon(item: &CatalogItem, side: ClearanceSide, value: f64) -> Vec<Point> {
    let half = item.size.width / 2.0;
    let depth = item.size.depth;
    match side {
        ClearanceSide::Front => vec![
            Point { x: -half, y: depth },
            Point { x: half, y: depth },
            Point {
                x: half,
                y: depth + value,
            },
            Point {
                x: -half,
                y: depth + value,
            },
        ],
        ClearanceSide::Back => vec![
            Point {
                x: -half,
                y: -value,
            },
            Point { x: half, y: -value },
            Point { x: half, y: 0.0 },
            Point { x: -half, y: 0.0 },
        ],
        ClearanceSide::Left => vec![
            Point {
                x: -half - value,
                y: 0.0,
            },
            Point { x: -half, y: 0.0 },
            Point { x: -half, y: depth },
            Point {
                x: -half - value,
                y: depth,
            },
        ],
        ClearanceSide::Right => vec![
            Point { x: half, y: 0.0 },
            Point {
                x: half + value,
                y: 0.0,
            },
            Point {
                x: half + value,
                y: depth,
            },
            Point { x: half, y: depth },
        ],
    }
}

fn opposite(dir: CardinalDirection) -> CardinalDirection {
    match dir {
        CardinalDirection::North => CardinalDirection::South,
        CardinalDirection::South => CardinalDirection::North,
        CardinalDirection::East => CardinalDirection::West,
        CardinalDirection::West => CardinalDirection::East,
    }
}

fn derive_site_info(street: CardinalDirection, hemisphere: Hemisphere) -> SiteInfo {
    SiteInfo {
        street,
        hemisphere,
        back: opposite(street),
        morning_sun: CardinalDirection::East,
        afternoon_sun: CardinalDirection::West,
        good_sun: if hemisphere == Hemisphere::North {
            CardinalDirection::South
        } else {
            CardinalDirection::North
        },
    }
}
