use crate::ast::{DoorSwing, EdgeSide, Point};
use crate::catalog::prefers_wall_placement_for;
use crate::flooring::floor_material_spec;
use crate::geometry::{
    GeometryIr, OpeningPlacement, OpeningPlacementType, ResolvedRoom, WallSegment,
};
use std::collections::HashSet;
use std::f64::consts::PI;

const EPS: f64 = 1e-6;
const ARC_STEPS: usize = 12;
const WALL_BACKED_ON_WALL_TOLERANCE: f64 = 0.02;
const WALL_BACKED_NEAR_WALL_DISTANCE: f64 = 0.35;

pub fn layout_warnings(geometry: &GeometryIr) -> Vec<String> {
    let mut warnings = Vec::new();
    let mut seen = HashSet::new();

    for opening in &geometry.openings {
        if opening.opening_type != OpeningPlacementType::Door {
            continue;
        }
        if opening.pocket {
            continue;
        }

        let Some(host_wall) = geometry
            .walls
            .iter()
            .find(|wall| wall.id == opening.wall_id)
        else {
            continue;
        };
        let leaves = door_leaves(opening, host_wall);

        'obstacles: for wall in &geometry.walls {
            if wall.id == host_wall.id {
                continue;
            }

            for leaf in &leaves {
                if swing_path_intersects_wall(leaf, wall) {
                    let key = (opening.id.clone(), wall.id.clone());
                    if seen.insert(key) {
                        warnings.push(format!(
                            "Door \"{}\" swing intersects wall \"{}\"{}; consider changing the swing hand/reverse, width, or position.",
                            opening.id,
                            wall.id,
                            wall_room_description(wall)
                        ));
                    }
                    continue 'obstacles;
                }
            }
        }
    }

    warnings.extend(wall_backed_fixture_warnings(geometry));
    warnings.extend(floor_material_warnings(geometry));
    warnings
}

fn floor_material_warnings(geometry: &GeometryIr) -> Vec<String> {
    let mut warnings = Vec::new();

    for room in &geometry.rooms {
        let Some(material) = &room.floor_material else {
            continue;
        };
        let Some(spec) = floor_material_spec(material) else {
            warnings.push(format!(
                "Room \"{}\" uses unknown floor material \"{}\"; rendering with a generic floor fill.",
                room.name, material
            ));
            continue;
        };
        if !spec.allows_indoor() {
            warnings.push(format!(
                "Room \"{}\" uses outdoor floor material \"{}\"; choose an indoor material or confirm this is intentional.",
                room.name, material
            ));
        }
    }

    for area in &geometry.outdoor_areas {
        let Some(material) = &area.floor_material else {
            continue;
        };
        let Some(spec) = floor_material_spec(material) else {
            warnings.push(format!(
                "Outdoor area \"{}\" uses unknown floor material \"{}\"; rendering with a generic floor fill.",
                area.name, material
            ));
            continue;
        };
        if !spec.allows_outdoor() {
            warnings.push(format!(
                "Outdoor area \"{}\" uses indoor floor material \"{}\"; choose an outdoor material or confirm this is intentional.",
                area.name, material
            ));
        }
    }

    warnings
}

fn wall_backed_fixture_warnings(geometry: &GeometryIr) -> Vec<String> {
    let mut warnings = Vec::new();

    for object in &geometry.objects {
        if !prefers_wall_placement_for(&object.category, &object.catalog_id) {
            continue;
        }

        let Some(room) = geometry.rooms.iter().find(|room| room.name == object.room) else {
            continue;
        };

        let Some((wall, gap)) = wall_backed_gap(room, &object.polygon.points, object.facing) else {
            continue;
        };

        if gap > WALL_BACKED_ON_WALL_TOLERANCE && gap <= WALL_BACKED_NEAR_WALL_DISTANCE {
            warnings.push(format!(
                "Object \"{}\" is {:.2}m from the {} wall of room \"{}\"; wall-backed fixtures should attach to the wall. Use \"attach {} wall\" with an \"at\" distance or move it farther into the room.",
                object.name,
                gap,
                wall,
                room.name,
                wall
            ));
        }
    }

    warnings
}

fn wall_backed_gap(
    room: &ResolvedRoom,
    object_points: &[Point],
    facing: EdgeSide,
) -> Option<(&'static str, f64)> {
    let room_bounds = bounds(&room.polygon.points)?;
    let object_bounds = bounds(object_points)?;
    match facing {
        EdgeSide::North => Some(("south", object_bounds.min_y - room_bounds.min_y)),
        EdgeSide::South => Some(("north", room_bounds.max_y - object_bounds.max_y)),
        EdgeSide::East => Some(("west", object_bounds.min_x - room_bounds.min_x)),
        EdgeSide::West => Some(("east", room_bounds.max_x - object_bounds.max_x)),
    }
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

fn bounds(points: &[Point]) -> Option<Bounds> {
    let first = points.first()?;
    let mut min_x = first.x;
    let mut max_x = first.x;
    let mut min_y = first.y;
    let mut max_y = first.y;

    for point in points.iter().skip(1) {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    Some(Bounds {
        min_x,
        max_x,
        min_y,
        max_y,
    })
}

#[derive(Debug, Clone, Copy)]
struct DoorLeaf {
    hinge: Point,
    closed_free: Point,
    open_free: Point,
}

fn door_leaves(opening: &OpeningPlacement, wall: &WallSegment) -> Vec<DoorLeaf> {
    let dx = wall.end.x - wall.start.x;
    let dy = wall.end.y - wall.start.y;
    let wall_length = (dx * dx + dy * dy).sqrt();
    if wall_length <= EPS {
        return Vec::new();
    }

    let ratio = opening.position / wall_length;
    let center = Point {
        x: wall.start.x + dx * ratio,
        y: wall.start.y + dy * ratio,
    };
    let ux = dx / wall_length;
    let uy = dy / wall_length;
    let half = opening.width / 2.0;
    let p1 = Point {
        x: center.x - ux * half,
        y: center.y - uy * half,
    };
    let p2 = Point {
        x: center.x + ux * half,
        y: center.y + uy * half,
    };

    let outside = opening.outside_normal.unwrap_or(Point { x: 0.0, y: -1.0 });
    let open_normal = if opening.swing.is_some_and(DoorSwing::opens_to_outside) {
        outside
    } else {
        Point {
            x: -outside.x,
            y: -outside.y,
        }
    };

    if opening.double {
        let leaf_width = opening.width / 2.0;
        return vec![
            door_leaf(p1, center, open_normal, leaf_width),
            door_leaf(p2, center, open_normal, leaf_width),
        ];
    }

    let swing = opening.swing.unwrap_or(DoorSwing::LeftHand);
    let (left, right) = left_right_points_from_outside(p1, p2, outside);
    let (hinge, closed_free) = if swing.hinge_is_left() {
        (left, right)
    } else {
        (right, left)
    };
    vec![door_leaf(hinge, closed_free, open_normal, opening.width)]
}

fn door_leaf(hinge: Point, closed_free: Point, open_normal: Point, leaf_width: f64) -> DoorLeaf {
    DoorLeaf {
        hinge,
        closed_free,
        open_free: Point {
            x: hinge.x + open_normal.x * leaf_width,
            y: hinge.y + open_normal.y * leaf_width,
        },
    }
}

fn swing_path_intersects_wall(leaf: &DoorLeaf, wall: &WallSegment) -> bool {
    let open_start = nudge_from(leaf.hinge, leaf.open_free);
    if segments_intersect(open_start, leaf.open_free, wall.start, wall.end) {
        return true;
    }

    let arc = arc_points(leaf.hinge, leaf.closed_free, leaf.open_free);
    for segment in arc.windows(2) {
        if segments_intersect(segment[0], segment[1], wall.start, wall.end) {
            return true;
        }
    }

    false
}

fn arc_points(hinge: Point, closed_free: Point, open_free: Point) -> Vec<Point> {
    let radius = distance(hinge, closed_free);
    if radius <= EPS {
        return vec![closed_free, open_free];
    }

    let start = (closed_free.y - hinge.y).atan2(closed_free.x - hinge.x);
    let end = (open_free.y - hinge.y).atan2(open_free.x - hinge.x);
    let mut delta = end - start;
    while delta <= -PI {
        delta += PI * 2.0;
    }
    while delta > PI {
        delta -= PI * 2.0;
    }

    (0..=ARC_STEPS)
        .map(|i| {
            let angle = start + delta * (i as f64 / ARC_STEPS as f64);
            Point {
                x: hinge.x + angle.cos() * radius,
                y: hinge.y + angle.sin() * radius,
            }
        })
        .collect()
}

fn nudge_from(start: Point, end: Point) -> Point {
    let len = distance(start, end);
    if len <= EPS {
        return start;
    }
    Point {
        x: start.x + (end.x - start.x) / len * EPS * 10.0,
        y: start.y + (end.y - start.y) / len * EPS * 10.0,
    }
}

fn left_right_points_from_outside(p1: Point, p2: Point, outside: Point) -> (Point, Point) {
    let view = Point {
        x: -outside.x,
        y: -outside.y,
    };
    let right = Point {
        x: view.y,
        y: -view.x,
    };
    if dot(p1, right) <= dot(p2, right) {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);

    if o1.abs() <= EPS && on_segment(a, c, b) {
        return true;
    }
    if o2.abs() <= EPS && on_segment(a, d, b) {
        return true;
    }
    if o3.abs() <= EPS && on_segment(c, a, d) {
        return true;
    }
    if o4.abs() <= EPS && on_segment(c, b, d) {
        return true;
    }

    (o1 > 0.0) != (o2 > 0.0) && (o3 > 0.0) != (o4 > 0.0)
}

fn orientation(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn on_segment(a: Point, b: Point, c: Point) -> bool {
    b.x >= a.x.min(c.x) - EPS
        && b.x <= a.x.max(c.x) + EPS
        && b.y >= a.y.min(c.y) - EPS
        && b.y <= a.y.max(c.y) + EPS
}

fn wall_room_description(wall: &WallSegment) -> String {
    match wall.rooms.as_slice() {
        [room] => format!(" for room \"{room}\""),
        [a, b] => format!(" between rooms \"{a}\" and \"{b}\""),
        _ => String::new(),
    }
}

fn distance(a: Point, b: Point) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}
