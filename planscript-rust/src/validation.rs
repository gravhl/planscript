use crate::ast::{Assertion, CardinalDirection, EdgeSide, OrientationTarget, Point};
use crate::geometry::{calculate_polygon_area, GeometryIr, OpeningPlacementType, WallSegment};
use crate::lowering::{LoweredProgram, SiteInfo};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "E101")]
    PolygonNotClosed,
    #[serde(rename = "E102")]
    PolygonSelfIntersecting,
    #[serde(rename = "E103")]
    PolygonTooFewPoints,
    #[serde(rename = "E130")]
    RoomOutsideFootprint,
    #[serde(rename = "E201")]
    RoomsOverlap,
    #[serde(rename = "E310")]
    OpeningNotOnWall,
    #[serde(rename = "E311")]
    OpeningExceedsWall,
    #[serde(rename = "E420")]
    RoomNoAccess,
    #[serde(rename = "E501")]
    MinAreaViolation,
    #[serde(rename = "E502")]
    ZeroArea,
    #[serde(rename = "E601")]
    OrientationNoSite,
    #[serde(rename = "E602")]
    OrientationNoWindow,
    #[serde(rename = "E603")]
    OrientationRoomNotNearStreet,
    #[serde(rename = "E604")]
    OrientationRoomNotAwayFromStreet,
    #[serde(rename = "E605")]
    OrientationNoGardenView,
    #[serde(rename = "E701")]
    RoomOverlapsCourtyard,
    #[serde(rename = "E801")]
    RoomsNotConnected,
    #[serde(rename = "E901")]
    ObjectOutsideRoom,
    #[serde(rename = "E902")]
    ObjectsOverlap,
    #[serde(rename = "E903")]
    ObjectClearanceViolation,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PolygonNotClosed => "E101",
            Self::PolygonSelfIntersecting => "E102",
            Self::PolygonTooFewPoints => "E103",
            Self::RoomOutsideFootprint => "E130",
            Self::RoomsOverlap => "E201",
            Self::OpeningNotOnWall => "E310",
            Self::OpeningExceedsWall => "E311",
            Self::RoomNoAccess => "E420",
            Self::MinAreaViolation => "E501",
            Self::ZeroArea => "E502",
            Self::OrientationNoSite => "E601",
            Self::OrientationNoWindow => "E602",
            Self::OrientationRoomNotNearStreet => "E603",
            Self::OrientationRoomNotAwayFromStreet => "E604",
            Self::OrientationNoGardenView => "E605",
            Self::RoomOverlapsCourtyard => "E701",
            Self::RoomsNotConnected => "E801",
            Self::ObjectOutsideRoom => "E901",
            Self::ObjectsOverlap => "E902",
            Self::ObjectClearanceViolation => "E903",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, Value>>,
}

impl ValidationError {
    fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_str().to_string(),
            message: message.into(),
            room: None,
            details: None,
        }
    }

    fn room(mut self, room: impl Into<String>) -> Self {
        self.room = Some(room.into());
        self
    }

    fn detail(mut self, key: &str, value: Value) -> Self {
        self.details
            .get_or_insert_with(HashMap::new)
            .insert(key.to_string(), value);
        self
    }
}

pub fn validate(lowered: &LoweredProgram, geometry: &GeometryIr) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    errors.extend(validate_polygons(lowered));
    errors.extend(validate_no_courtyard_overlap(lowered));

    for assertion in &lowered.assertions {
        match assertion {
            Assertion::AssertionInsideFootprint { .. } => {
                errors.extend(validate_inside_footprint(lowered));
            }
            Assertion::AssertionNoOverlap { .. } => {
                errors.extend(validate_no_overlap(lowered));
            }
            Assertion::AssertionOpeningsOnWalls => {
                errors.extend(validate_openings_on_walls(geometry));
            }
            Assertion::AssertionRoomsConnected => {
                errors.extend(validate_rooms_connected(lowered, geometry));
            }
            Assertion::AssertionObjectsInsideRooms => {
                errors.extend(validate_objects_inside_rooms(geometry));
            }
            Assertion::AssertionObjectNoOverlap => {
                errors.extend(validate_object_no_overlap(geometry));
            }
            Assertion::AssertionObjectClearances => {
                errors.extend(validate_object_clearances(geometry));
            }
            Assertion::AssertionMinRoomArea { room, min_area } => {
                errors.extend(validate_min_room_area(lowered, room, *min_area));
            }
            _ => {}
        }
    }

    errors.extend(validate_orientation_assertions(lowered, geometry));
    errors
}

fn validate_polygons(lowered: &LoweredProgram) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if lowered.footprint.len() < 3 {
        errors.push(ValidationError::new(
            ErrorCode::PolygonTooFewPoints,
            "Footprint polygon must have at least 3 points",
        ));
    }

    for room in &lowered.rooms {
        if room.polygon.len() < 3 {
            errors.push(
                ValidationError::new(
                    ErrorCode::PolygonTooFewPoints,
                    format!("Room \"{}\" polygon must have at least 3 points", room.name),
                )
                .room(&room.name),
            );
        }
        if calculate_polygon_area(&room.polygon) == 0.0 {
            errors.push(
                ValidationError::new(
                    ErrorCode::ZeroArea,
                    format!("Room \"{}\" has zero area", room.name),
                )
                .room(&room.name),
            );
        }
    }
    errors
}

fn validate_inside_footprint(lowered: &LoweredProgram) -> Vec<ValidationError> {
    lowered
        .rooms
        .iter()
        .filter(|room| !polygon_inside_polygon(&room.polygon, &lowered.footprint))
        .map(|room| {
            ValidationError::new(
                ErrorCode::RoomOutsideFootprint,
                format!("Room \"{}\" is outside the footprint", room.name),
            )
            .room(&room.name)
        })
        .collect()
}

fn validate_no_overlap(lowered: &LoweredProgram) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for i in 0..lowered.rooms.len() {
        for j in (i + 1)..lowered.rooms.len() {
            let a = &lowered.rooms[i];
            let b = &lowered.rooms[j];
            if polygons_overlap(&a.polygon, &b.polygon) {
                errors.push(
                    ValidationError::new(
                        ErrorCode::RoomsOverlap,
                        format!("Rooms \"{}\" and \"{}\" overlap", a.name, b.name),
                    )
                    .detail("room1", json!(a.name))
                    .detail("room2", json!(b.name)),
                );
            }
        }
    }
    errors
}

fn validate_no_courtyard_overlap(lowered: &LoweredProgram) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for room in &lowered.rooms {
        for courtyard in &lowered.courtyards {
            if polygons_overlap(&room.polygon, &courtyard.polygon) {
                errors.push(
                    ValidationError::new(
                        ErrorCode::RoomOverlapsCourtyard,
                        format!(
                            "Room \"{}\" overlaps with courtyard \"{}\"",
                            room.name, courtyard.name
                        ),
                    )
                    .room(&room.name)
                    .detail("courtyard", json!(courtyard.name)),
                );
            }
        }
    }
    errors
}

fn validate_openings_on_walls(geometry: &GeometryIr) -> Vec<ValidationError> {
    let wall_ids: HashSet<&str> = geometry.walls.iter().map(|w| w.id.as_str()).collect();
    geometry
        .openings
        .iter()
        .filter(|opening| !wall_ids.contains(opening.wall_id.as_str()))
        .map(|opening| {
            ValidationError::new(
                ErrorCode::OpeningNotOnWall,
                format!("Opening \"{}\" is not placed on a valid wall", opening.id),
            )
            .detail("openingId", json!(opening.id))
        })
        .collect()
}

fn validate_objects_inside_rooms(geometry: &GeometryIr) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for object in &geometry.objects {
        let Some(room) = geometry.rooms.iter().find(|room| room.name == object.room) else {
            errors.push(
                ValidationError::new(
                    ErrorCode::ObjectOutsideRoom,
                    format!(
                        "Object \"{}\" references missing room \"{}\"",
                        object.name, object.room
                    ),
                )
                .detail("object", json!(object.name))
                .detail("room", json!(object.room)),
            );
            continue;
        };
        if !polygon_inside_polygon(&object.polygon.points, &room.polygon.points) {
            errors.push(
                ValidationError::new(
                    ErrorCode::ObjectOutsideRoom,
                    format!(
                        "Object \"{}\" is outside room \"{}\"",
                        object.name, object.room
                    ),
                )
                .room(&object.room)
                .detail("object", json!(object.name)),
            );
        }
    }
    errors
}

fn validate_object_no_overlap(geometry: &GeometryIr) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for i in 0..geometry.objects.len() {
        for j in (i + 1)..geometry.objects.len() {
            let a = &geometry.objects[i];
            let b = &geometry.objects[j];
            if a.room == b.room && polygons_overlap(&a.polygon.points, &b.polygon.points) {
                errors.push(
                    ValidationError::new(
                        ErrorCode::ObjectsOverlap,
                        format!("Objects \"{}\" and \"{}\" overlap", a.name, b.name),
                    )
                    .room(&a.room)
                    .detail("object1", json!(a.name))
                    .detail("object2", json!(b.name)),
                );
            }
        }
    }
    errors
}

fn validate_object_clearances(geometry: &GeometryIr) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for object in &geometry.objects {
        let Some(room) = geometry.rooms.iter().find(|room| room.name == object.room) else {
            continue;
        };
        for clearance in &object.clearance_polygons {
            if !polygon_inside_polygon(&clearance.polygon.points, &room.polygon.points) {
                errors.push(
                    ValidationError::new(
                        ErrorCode::ObjectClearanceViolation,
                        format!(
                            "Object \"{}\" {:?} clearance is outside room \"{}\"",
                            object.name, clearance.side, object.room
                        ),
                    )
                    .room(&object.room)
                    .detail("object", json!(object.name))
                    .detail(
                        "side",
                        json!(format!("{:?}", clearance.side).to_lowercase()),
                    ),
                );
            }
            for other in &geometry.objects {
                if other.name == object.name || other.room != object.room {
                    continue;
                }
                if polygons_overlap(&clearance.polygon.points, &other.polygon.points) {
                    errors.push(
                        ValidationError::new(
                            ErrorCode::ObjectClearanceViolation,
                            format!(
                                "Object \"{}\" {:?} clearance overlaps object \"{}\"",
                                object.name, clearance.side, other.name
                            ),
                        )
                        .room(&object.room)
                        .detail("object", json!(object.name))
                        .detail(
                            "side",
                            json!(format!("{:?}", clearance.side).to_lowercase()),
                        )
                        .detail("blockingObject", json!(other.name)),
                    );
                }
            }
        }
    }
    errors
}

fn validate_min_room_area(
    lowered: &LoweredProgram,
    room: &str,
    min_area: f64,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if let Some(found) = lowered.rooms.iter().find(|r| r.name == room) {
        let area = calculate_polygon_area(&found.polygon);
        if area < min_area {
            errors.push(
                ValidationError::new(
                    ErrorCode::MinAreaViolation,
                    format!(
                        "Room \"{}\" area ({:.2}) is less than minimum ({})",
                        room, area, min_area
                    ),
                )
                .room(room)
                .detail("actual", json!(area))
                .detail("minimum", json!(min_area)),
            );
        }
    }
    errors
}

fn validate_rooms_connected(
    lowered: &LoweredProgram,
    geometry: &GeometryIr,
) -> Vec<ValidationError> {
    if lowered.rooms.len() <= 1 {
        return Vec::new();
    }

    let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
    for room in &lowered.rooms {
        adjacency.insert(room.name.clone(), HashSet::new());
    }

    for i in 0..lowered.rooms.len() {
        for j in (i + 1)..lowered.rooms.len() {
            let a = &lowered.rooms[i];
            let b = &lowered.rooms[j];
            if polygons_share_edge(&a.polygon, &b.polygon, 0.01) {
                adjacency.get_mut(&a.name).unwrap().insert(b.name.clone());
                adjacency.get_mut(&b.name).unwrap().insert(a.name.clone());
            }
        }
    }

    for opening in &geometry.openings {
        if opening.opening_type == OpeningPlacementType::Door {
            if let Some(wall) = geometry.walls.iter().find(|w| w.id == opening.wall_id) {
                if wall.rooms.len() == 2 {
                    adjacency
                        .get_mut(&wall.rooms[0])
                        .map(|s| s.insert(wall.rooms[1].clone()));
                    adjacency
                        .get_mut(&wall.rooms[1])
                        .map(|s| s.insert(wall.rooms[0].clone()));
                }
            }
        }
    }

    let start = lowered.rooms[0].name.clone();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start.clone()]);
    visited.insert(start);
    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = adjacency.get(&current) {
            for neighbor in neighbors {
                if visited.insert(neighbor.clone()) {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    let disconnected: Vec<String> = lowered
        .rooms
        .iter()
        .filter(|r| !visited.contains(&r.name))
        .map(|r| r.name.clone())
        .collect();

    if disconnected.is_empty() {
        Vec::new()
    } else {
        vec![ValidationError::new(
            ErrorCode::RoomsNotConnected,
            format!(
                "Rooms are not fully connected. Disconnected rooms: {}",
                disconnected.join(", ")
            ),
        )
        .detail(
            "connectedRooms",
            json!(visited.into_iter().collect::<Vec<_>>()),
        )
        .detail("disconnectedRooms", json!(disconnected))]
    }
}

fn validate_orientation_assertions(
    lowered: &LoweredProgram,
    geometry: &GeometryIr,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let needs_site = lowered.assertions.iter().any(|assertion| {
        matches!(
            assertion,
            Assertion::AssertionOrientationHasWindow { .. }
                | Assertion::AssertionOrientationNearStreet { .. }
                | Assertion::AssertionOrientationAwayFromStreet { .. }
                | Assertion::AssertionOrientationGardenView { .. }
        )
    });
    if needs_site && lowered.site.is_none() {
        errors.push(ValidationError::new(
            ErrorCode::OrientationNoSite,
            "Orientation assertions require a site declaration with street direction",
        ));
        return errors;
    }
    let Some(site) = lowered.site else {
        return errors;
    };

    for assertion in &lowered.assertions {
        match assertion {
            Assertion::AssertionOrientationHasWindow { room, target } => {
                if let Some(error) = validate_orientation_has_window(room, *target, site, geometry)
                {
                    errors.push(error);
                }
            }
            Assertion::AssertionOrientationNearStreet { room } => {
                if !is_room_near_direction(room, site.street, lowered) {
                    errors.push(
                        ValidationError::new(
                            ErrorCode::OrientationRoomNotNearStreet,
                            format!(
                                "Room \"{}\" is not near the street ({:?})",
                                room, site.street
                            ),
                        )
                        .room(room)
                        .detail(
                            "streetDirection",
                            json!(format!("{:?}", site.street).to_lowercase()),
                        ),
                    );
                }
            }
            Assertion::AssertionOrientationAwayFromStreet { room } => {
                if !is_room_near_direction(room, site.back, lowered) {
                    errors.push(
                        ValidationError::new(
                            ErrorCode::OrientationRoomNotAwayFromStreet,
                            format!(
                                "Room \"{}\" is not away from the street (should be near {:?})",
                                room, site.back
                            ),
                        )
                        .room(room),
                    );
                }
            }
            Assertion::AssertionOrientationGardenView { room } => {
                let dirs = get_room_window_directions(room, geometry);
                if !dirs.contains(&site.back) {
                    errors.push(
                        ValidationError::new(
                            ErrorCode::OrientationNoGardenView,
                            format!(
                                "Room \"{}\" does not have a window with garden view (facing {:?})",
                                room, site.back
                            ),
                        )
                        .room(room),
                    );
                }
            }
            _ => {}
        }
    }
    errors
}

fn validate_orientation_has_window(
    room: &str,
    target: OrientationTarget,
    site: SiteInfo,
    geometry: &GeometryIr,
) -> Option<ValidationError> {
    let target_dirs = resolve_orientation_target(target, site);
    let window_dirs = get_room_window_directions(room, geometry);
    if target_dirs.iter().any(|dir| window_dirs.contains(dir)) {
        return None;
    }
    let label = match target {
        OrientationTarget::MorningSun => "morning sun (east)".to_string(),
        OrientationTarget::AfternoonSun => "afternoon sun (west)".to_string(),
        OrientationTarget::GoodSun => {
            format!("good sun ({})", cardinal_label(site.good_sun))
        }
        OrientationTarget::Street => format!("street ({})", cardinal_label(site.street)),
        _ => cardinal_label(target_dirs[0]).to_string(),
    };
    Some(
        ValidationError::new(
            ErrorCode::OrientationNoWindow,
            format!("Room \"{}\" does not have a window facing {}", room, label),
        )
        .room(room)
        .detail(
            "targetDirections",
            json!(target_dirs
                .iter()
                .map(|d| cardinal_label(*d))
                .collect::<Vec<_>>()),
        ),
    )
}

fn resolve_orientation_target(target: OrientationTarget, site: SiteInfo) -> Vec<CardinalDirection> {
    match target {
        OrientationTarget::MorningSun => vec![site.morning_sun],
        OrientationTarget::AfternoonSun => vec![site.afternoon_sun],
        OrientationTarget::GoodSun => vec![site.good_sun],
        OrientationTarget::Street => vec![site.street],
        OrientationTarget::North => vec![CardinalDirection::North],
        OrientationTarget::South => vec![CardinalDirection::South],
        OrientationTarget::East => vec![CardinalDirection::East],
        OrientationTarget::West => vec![CardinalDirection::West],
    }
}

fn get_room_window_directions(room_name: &str, geometry: &GeometryIr) -> Vec<CardinalDirection> {
    let mut dirs = Vec::new();
    for opening in &geometry.openings {
        if opening.opening_type != OpeningPlacementType::Window {
            continue;
        }
        let Some(wall) = geometry.walls.iter().find(|w| w.id == opening.wall_id) else {
            continue;
        };
        if !wall.rooms.iter().any(|r| r == room_name) {
            continue;
        }
        let Some(room) = geometry.rooms.iter().find(|r| r.name == room_name) else {
            continue;
        };
        if let Some(dir) = wall_direction(wall, &room.polygon.points) {
            dirs.push(dir);
        }
    }
    dirs
}

fn wall_direction(wall: &WallSegment, points: &[Point]) -> Option<CardinalDirection> {
    let (min_x, max_x, min_y, max_y) = bounds(points);
    let mid_x = (wall.start.x + wall.end.x) / 2.0;
    let mid_y = (wall.start.y + wall.end.y) / 2.0;
    let eps = 0.01;
    if (wall.start.y - wall.end.y).abs() < eps {
        if (mid_y - max_y).abs() < eps {
            return Some(CardinalDirection::North);
        }
        if (mid_y - min_y).abs() < eps {
            return Some(CardinalDirection::South);
        }
    }
    if (wall.start.x - wall.end.x).abs() < eps {
        if (mid_x - max_x).abs() < eps {
            return Some(CardinalDirection::East);
        }
        if (mid_x - min_x).abs() < eps {
            return Some(CardinalDirection::West);
        }
    }
    None
}

fn is_room_near_direction(
    room_name: &str,
    direction: CardinalDirection,
    lowered: &LoweredProgram,
) -> bool {
    let Some(room) = lowered.rooms.iter().find(|r| r.name == room_name) else {
        return false;
    };
    let fp = bounds(&lowered.footprint);
    let rb = bounds(&room.polygon);
    let tolerance = 0.5;
    match direction {
        CardinalDirection::North => (rb.3 - fp.3).abs() < tolerance,
        CardinalDirection::South => (rb.2 - fp.2).abs() < tolerance,
        CardinalDirection::East => (rb.1 - fp.1).abs() < tolerance,
        CardinalDirection::West => (rb.0 - fp.0).abs() < tolerance,
    }
}

fn polygon_inside_polygon(inner: &[Point], outer: &[Point]) -> bool {
    inner.iter().all(|point| {
        point_in_polygon(*point, outer) || point_on_polygon_boundary(*point, outer, 1e-10)
    })
}

fn polygons_overlap(a: &[Point], b: &[Point]) -> bool {
    for p in a {
        if point_in_polygon(*p, b) && !point_on_polygon_boundary(*p, b, 1e-10) {
            return true;
        }
    }
    for p in b {
        if point_in_polygon(*p, a) && !point_on_polygon_boundary(*p, a, 1e-10) {
            return true;
        }
    }
    edges_intersect(a, b)
}

fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    if n == 0 {
        return false;
    }
    let mut j = n - 1;
    for i in 0..n {
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

fn point_on_polygon_boundary(point: Point, polygon: &[Point], epsilon: f64) -> bool {
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        let d1 = dist(point, p1);
        let d2 = dist(point, p2);
        let d12 = dist(p1, p2);
        if (d1 + d2 - d12).abs() < epsilon {
            return true;
        }
    }
    false
}

fn edges_intersect(a: &[Point], b: &[Point]) -> bool {
    for i in 0..a.len() {
        let a1 = a[i];
        let a2 = a[(i + 1) % a.len()];
        for j in 0..b.len() {
            let b1 = b[j];
            let b2 = b[(j + 1) % b.len()];
            if segments_intersect(a1, a2, b1, b2) {
                return true;
            }
        }
    }
    false
}

fn segments_intersect(a1: Point, a2: Point, b1: Point, b2: Point) -> bool {
    let d1 = direction(b1, b2, a1);
    let d2 = direction(b1, b2, a2);
    let d3 = direction(a1, a2, b1);
    let d4 = direction(a1, a2, b2);
    ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
}

fn direction(p1: Point, p2: Point, p3: Point) -> f64 {
    (p3.x - p1.x) * (p2.y - p1.y) - (p2.x - p1.x) * (p3.y - p1.y)
}

fn polygons_share_edge(a: &[Point], b: &[Point], epsilon: f64) -> bool {
    for i in 0..a.len() {
        let a1 = a[i];
        let a2 = a[(i + 1) % a.len()];
        for j in 0..b.len() {
            let b1 = b[j];
            let b2 = b[(j + 1) % b.len()];
            if edges_overlap(a1, a2, b1, b2, epsilon) {
                return true;
            }
        }
    }
    false
}

fn edges_overlap(a1: Point, a2: Point, b1: Point, b2: Point, epsilon: f64) -> bool {
    let a_h = (a1.y - a2.y).abs() < epsilon;
    let b_h = (b1.y - b2.y).abs() < epsilon;
    let a_v = (a1.x - a2.x).abs() < epsilon;
    let b_v = (b1.x - b2.x).abs() < epsilon;

    if a_h && b_h && (a1.y - b1.y).abs() < epsilon {
        let start = a1.x.min(a2.x).max(b1.x.min(b2.x));
        let end = a1.x.max(a2.x).min(b1.x.max(b2.x));
        return end - start > epsilon;
    }
    if a_v && b_v && (a1.x - b1.x).abs() < epsilon {
        let start = a1.y.min(a2.y).max(b1.y.min(b2.y));
        let end = a1.y.max(a2.y).min(b1.y.max(b2.y));
        return end - start > epsilon;
    }
    false
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

fn dist(a: Point, b: Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn cardinal_label(dir: CardinalDirection) -> &'static str {
    match dir {
        CardinalDirection::North => "north",
        CardinalDirection::South => "south",
        CardinalDirection::East => "east",
        CardinalDirection::West => "west",
    }
}

#[allow(dead_code)]
fn edge_side_to_cardinal(edge: EdgeSide) -> CardinalDirection {
    match edge {
        EdgeSide::North => CardinalDirection::North,
        EdgeSide::South => CardinalDirection::South,
        EdgeSide::East => CardinalDirection::East,
        EdgeSide::West => CardinalDirection::West,
    }
}
