use crate::ast::{CardinalDirection, Point, Program};
use crate::geometry::{
    GeometryIr, OpeningPlacementType, Polygon, ResolvedCourtyard, ResolvedRoom, WallSegment,
};
use crate::lowering::SiteInfo;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgExportOptions {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub padding: Option<f64>,
    pub scale: Option<f64>,
    pub show_labels: Option<bool>,
    pub show_dimensions: Option<bool>,
    pub show_footprint_dimensions: Option<bool>,
    pub show_compass: Option<bool>,
    pub background_color: Option<String>,
    pub wall_color: Option<String>,
    pub wall_width: Option<f64>,
    pub room_fill_color: Option<String>,
    pub room_stroke_color: Option<String>,
    pub room_stroke_width: Option<f64>,
    pub courtyard_fill_color: Option<String>,
    pub courtyard_stroke_color: Option<String>,
    pub courtyard_stroke_width: Option<f64>,
    pub object_fill_color: Option<String>,
    pub object_stroke_color: Option<String>,
    pub object_clearance_color: Option<String>,
    pub door_color: Option<String>,
    pub window_color: Option<String>,
    pub footprint_color: Option<String>,
    pub label_font_size: Option<f64>,
    pub label_color: Option<String>,
    pub dimension_color: Option<String>,
    pub dimension_font_size: Option<f64>,
    pub dimension_offset: Option<f64>,
    pub compass_size: Option<f64>,
    pub compass_color: Option<String>,
    pub street_indicator_color: Option<String>,
}

#[derive(Debug, Clone)]
struct SvgOptions {
    width: f64,
    height: f64,
    padding: f64,
    scale: f64,
    show_labels: bool,
    show_dimensions: bool,
    show_footprint_dimensions: bool,
    show_compass: bool,
    background_color: String,
    wall_color: String,
    wall_width: f64,
    room_fill_color: String,
    room_stroke_color: String,
    room_stroke_width: f64,
    courtyard_fill_color: String,
    courtyard_stroke_color: String,
    courtyard_stroke_width: f64,
    object_fill_color: String,
    object_stroke_color: String,
    object_clearance_color: String,
    door_color: String,
    window_color: String,
    footprint_color: String,
    label_font_size: f64,
    label_color: String,
    dimension_color: String,
    dimension_font_size: f64,
    dimension_offset: f64,
    compass_size: f64,
    compass_color: String,
    street_indicator_color: String,
}

impl SvgOptions {
    fn from_options(options: SvgExportOptions) -> Self {
        Self {
            width: options.width.unwrap_or(1000.0),
            height: options.height.unwrap_or(800.0),
            padding: options.padding.unwrap_or(60.0),
            scale: options.scale.unwrap_or(1.0),
            show_labels: options.show_labels.unwrap_or(true),
            show_dimensions: options.show_dimensions.unwrap_or(false),
            show_footprint_dimensions: options.show_footprint_dimensions.unwrap_or(true),
            show_compass: options.show_compass.unwrap_or(true),
            background_color: options
                .background_color
                .unwrap_or_else(|| "#ffffff".to_string()),
            wall_color: options.wall_color.unwrap_or_else(|| "#2c3e50".to_string()),
            wall_width: options.wall_width.unwrap_or(3.0),
            room_fill_color: options
                .room_fill_color
                .unwrap_or_else(|| "#ecf0f1".to_string()),
            room_stroke_color: options
                .room_stroke_color
                .unwrap_or_else(|| "#bdc3c7".to_string()),
            room_stroke_width: options.room_stroke_width.unwrap_or(1.0),
            courtyard_fill_color: options
                .courtyard_fill_color
                .unwrap_or_else(|| "#d5f5e3".to_string()),
            courtyard_stroke_color: options
                .courtyard_stroke_color
                .unwrap_or_else(|| "#27ae60".to_string()),
            courtyard_stroke_width: options.courtyard_stroke_width.unwrap_or(2.0),
            object_fill_color: options
                .object_fill_color
                .unwrap_or_else(|| "#f8f1df".to_string()),
            object_stroke_color: options
                .object_stroke_color
                .unwrap_or_else(|| "#8a6d3b".to_string()),
            object_clearance_color: options
                .object_clearance_color
                .unwrap_or_else(|| "#f39c12".to_string()),
            door_color: options.door_color.unwrap_or_else(|| "#e74c3c".to_string()),
            window_color: options
                .window_color
                .unwrap_or_else(|| "#3498db".to_string()),
            footprint_color: options
                .footprint_color
                .unwrap_or_else(|| "#7f8c8d".to_string()),
            label_font_size: options.label_font_size.unwrap_or(14.0),
            label_color: options.label_color.unwrap_or_else(|| "#2c3e50".to_string()),
            dimension_color: options
                .dimension_color
                .unwrap_or_else(|| "#666666".to_string()),
            dimension_font_size: options.dimension_font_size.unwrap_or(10.0),
            dimension_offset: options.dimension_offset.unwrap_or(20.0),
            compass_size: options.compass_size.unwrap_or(50.0),
            compass_color: options
                .compass_color
                .unwrap_or_else(|| "#2c3e50".to_string()),
            street_indicator_color: options
                .street_indicator_color
                .unwrap_or_else(|| "#e74c3c".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Transform {
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    height: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonExportOptions {
    pub pretty: Option<bool>,
    pub include_ast: Option<bool>,
}

pub fn export_json(
    geometry: &GeometryIr,
    options: JsonExportOptions,
    ast: Option<&Program>,
) -> Result<String, serde_json::Error> {
    let mut result = serde_json::Map::new();
    result.insert("version".to_string(), json!("1.0.0"));
    result.insert("geometry".to_string(), serde_json::to_value(geometry)?);
    if options.include_ast.unwrap_or(false) {
        if let Some(ast) = ast {
            result.insert("ast".to_string(), serde_json::to_value(ast)?);
        }
    }
    let value = Value::Object(result);
    if options.pretty.unwrap_or(false) {
        serde_json::to_string_pretty(&value)
    } else {
        serde_json::to_string(&value)
    }
}

pub fn export_svg(
    geometry: &GeometryIr,
    options: SvgExportOptions,
    site: Option<SiteInfo>,
) -> String {
    let opts = SvgOptions::from_options(options);
    let transform = create_transform(geometry, &opts);
    let width = number(opts.width);
    let height = number(opts.height);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <defs>
    <style>
      .room-label {{ font-family: Arial, sans-serif; font-weight: 500; }}
    </style>
  </defs>
  
  <!-- Background -->
  <rect width="{width}" height="{height}" fill="{bg}" />
  
  <!-- Footprint (boundary) -->
  {footprint}
  
  <!-- Rooms -->
  {rooms}
  
  <!-- Courtyards -->
  {courtyards}

  <!-- Objects / Fixtures -->
  {objects}
  
  <!-- Walls -->
  {walls}
  
  <!-- Openings (doors/windows) -->
  {openings}
  
  <!-- Labels -->
  {labels}
  
  <!-- Dimensions -->
  {dimensions}
  
  <!-- Compass / Orientation -->
  {compass}
</svg>"#,
        bg = opts.background_color,
        footprint = generate_footprint_svg(&geometry.footprint, transform, &opts),
        rooms = generate_rooms_svg(&geometry.rooms, transform, &opts),
        courtyards = generate_courtyards_svg(&geometry.courtyards, transform, &opts),
        objects = generate_objects_svg(geometry, transform, &opts),
        walls = generate_walls_svg(&geometry.walls, transform, &opts),
        openings = generate_openings_svg(geometry, transform, &opts),
        labels = generate_labels_svg(geometry, transform, &opts),
        dimensions = generate_dimensions_svg(geometry, transform, &opts),
        compass = generate_compass_svg(site, &opts),
    )
}

fn create_transform(geometry: &GeometryIr, opts: &SvgOptions) -> Transform {
    let (min_x, max_x, min_y, max_y) = bounds(&geometry.footprint.points);
    let content_width = (max_x - min_x).max(0.001);
    let content_height = (max_y - min_y).max(0.001);
    let dimension_space = if opts.show_dimensions {
        opts.dimension_offset + 70.0
    } else {
        0.0
    };
    let padding = opts.padding + dimension_space;
    let available_width = (opts.width - padding * 2.0).max(1.0);
    let available_height = (opts.height - padding * 2.0).max(1.0);
    let scale =
        (available_width / content_width).min(available_height / content_height) * opts.scale;
    let scaled_width = content_width * scale;
    let scaled_height = content_height * scale;
    Transform {
        scale,
        offset_x: padding + (available_width - scaled_width) / 2.0 - min_x * scale,
        offset_y: padding + (available_height - scaled_height) / 2.0 - min_y * scale,
        height: opts.height,
    }
}

fn transform_point(p: Point, t: Transform) -> Point {
    Point {
        x: p.x * t.scale + t.offset_x,
        y: t.height - (p.y * t.scale + t.offset_y),
    }
}

fn transform_polygon(points: &[Point], t: Transform) -> Vec<Point> {
    points.iter().map(|p| transform_point(*p, t)).collect()
}

fn points_to_path(points: &[Point]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for (i, p) in points.iter().enumerate() {
        if i == 0 {
            out.push_str(&format!("M {:.2} {:.2}", p.x, p.y));
        } else {
            out.push_str(&format!(" L {:.2} {:.2}", p.x, p.y));
        }
    }
    out.push_str(" Z");
    out
}

fn generate_footprint_svg(footprint: &Polygon, t: Transform, opts: &SvgOptions) -> String {
    let points = transform_polygon(&footprint.points, t);
    format!(
        r#"<path d="{}" fill="none" stroke="{}" stroke-width="2" stroke-dasharray="8,4" />"#,
        points_to_path(&points),
        opts.footprint_color
    )
}

fn generate_rooms_svg(rooms: &[ResolvedRoom], t: Transform, opts: &SvgOptions) -> String {
    rooms
        .iter()
        .map(|room| {
            let points = transform_polygon(&room.polygon.points, t);
            format!(
                r#"<path d="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
                points_to_path(&points),
                opts.room_fill_color,
                opts.room_stroke_color,
                number(opts.room_stroke_width)
            )
        })
        .collect::<Vec<_>>()
        .join("\n    ")
}

fn generate_courtyards_svg(
    courtyards: &[ResolvedCourtyard],
    t: Transform,
    opts: &SvgOptions,
) -> String {
    courtyards
        .iter()
        .map(|courtyard| {
            let points = transform_polygon(&courtyard.polygon.points, t);
            format!(
                r#"<path d="{}" fill="{}" stroke="{}" stroke-width="{}" stroke-dasharray="4,2" />"#,
                points_to_path(&points),
                opts.courtyard_fill_color,
                opts.courtyard_stroke_color,
                number(opts.courtyard_stroke_width)
            )
        })
        .collect::<Vec<_>>()
        .join("\n    ")
}

fn generate_objects_svg(geometry: &GeometryIr, t: Transform, opts: &SvgOptions) -> String {
    let mut out = Vec::new();
    for object in &geometry.objects {
        for clearance in &object.clearance_polygons {
            let points = transform_polygon(&clearance.polygon.points, t);
            out.push(format!(
                r#"<path d="{}" fill="{}" fill-opacity="0.12" stroke="{}" stroke-width="1" stroke-dasharray="3,2" />"#,
                points_to_path(&points),
                opts.object_clearance_color,
                opts.object_clearance_color
            ));
        }
    }
    for object in &geometry.objects {
        let points = transform_polygon(&object.polygon.points, t);
        out.push(format!(
            r#"<path data-object="{}" data-catalog-id="{}" d="{}" fill="{}" stroke="{}" stroke-width="1.5" />"#,
            escape_xml(&object.name),
            escape_xml(&object.catalog_id),
            points_to_path(&points),
            opts.object_fill_color,
            opts.object_stroke_color
        ));
        let origin = transform_point(object.origin, t);
        out.push(format!(
            r#"<circle data-object-origin="{}" cx="{:.2}" cy="{:.2}" r="2.5" fill="{}" />"#,
            escape_xml(&object.name),
            origin.x,
            origin.y,
            opts.object_stroke_color
        ));
    }
    out.join("\n    ")
}

fn generate_walls_svg(walls: &[WallSegment], t: Transform, opts: &SvgOptions) -> String {
    walls
        .iter()
        .map(|wall| {
            let start = transform_point(wall.start, t);
            let end = transform_point(wall.end, t);
            format!(
                r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="{}" stroke-linecap="round" />"#,
                start.x,
                start.y,
                end.x,
                end.y,
                opts.wall_color,
                number(opts.wall_width)
            )
        })
        .collect::<Vec<_>>()
        .join("\n    ")
}

fn generate_openings_svg(geometry: &GeometryIr, t: Transform, opts: &SvgOptions) -> String {
    let mut out = Vec::new();
    for opening in &geometry.openings {
        let Some(wall) = geometry.walls.iter().find(|w| w.id == opening.wall_id) else {
            continue;
        };
        let dx = wall.end.x - wall.start.x;
        let dy = wall.end.y - wall.start.y;
        let wall_length = (dx * dx + dy * dy).sqrt();
        if wall_length <= 0.0 {
            continue;
        }
        let ratio = opening.position / wall_length;
        let center = Point {
            x: wall.start.x + dx * ratio,
            y: wall.start.y + dy * ratio,
        };
        let ux = dx / wall_length;
        let uy = dy / wall_length;
        let half = opening.width / 2.0;
        let p1 = transform_point(
            Point {
                x: center.x - ux * half,
                y: center.y - uy * half,
            },
            t,
        );
        let p2 = transform_point(
            Point {
                x: center.x + ux * half,
                y: center.y + uy * half,
            },
            t,
        );
        let (color, stroke) = match opening.opening_type {
            OpeningPlacementType::Door => (&opts.door_color, 4.0),
            OpeningPlacementType::Window => (&opts.window_color, 3.0),
        };
        out.push(format!(
            r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="{}" stroke-linecap="round" />"#,
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            color,
            number(stroke)
        ));
    }
    out.join("\n    ")
}

fn generate_labels_svg(geometry: &GeometryIr, t: Transform, opts: &SvgOptions) -> String {
    if !opts.show_labels {
        return String::new();
    }
    let mut out = Vec::new();
    for room in &geometry.rooms {
        if let Some(label) = &room.label {
            let center = transform_point(polygon_center(&room.polygon.points), t);
            out.push(format!(
                r#"<text x="{:.2}" y="{:.2}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">{}</text>"#,
                center.x,
                center.y,
                number(opts.label_font_size),
                opts.label_color,
                escape_xml(label)
            ));
        }
    }
    for courtyard in &geometry.courtyards {
        if let Some(label) = &courtyard.label {
            let center = transform_point(polygon_center(&courtyard.polygon.points), t);
            out.push(format!(
                r#"<text x="{:.2}" y="{:.2}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif" font-style="italic">{}</text>"#,
                center.x,
                center.y,
                number(opts.label_font_size),
                opts.courtyard_stroke_color,
                escape_xml(label)
            ));
        }
    }
    for object in &geometry.objects {
        if let Some(label) = &object.label {
            let center = transform_point(polygon_center(&object.polygon.points), t);
            out.push(format!(
                r#"<text x="{:.2}" y="{:.2}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">{}</text>"#,
                center.x,
                center.y,
                number((opts.label_font_size * 0.72).max(7.0)),
                opts.object_stroke_color,
                escape_xml(label)
            ));
        }
    }
    out.join("\n    ")
}

fn generate_dimensions_svg(geometry: &GeometryIr, t: Transform, opts: &SvgOptions) -> String {
    if !opts.show_dimensions {
        return String::new();
    }
    let mut out = Vec::new();
    for room in &geometry.rooms {
        let (min_x, max_x, min_y, max_y) = bounds(&room.polygon.points);
        out.push(dimension_line(
            Point { x: min_x, y: min_y },
            Point { x: max_x, y: min_y },
            opts.dimension_offset,
            &format_dimension(max_x - min_x),
            "bottom",
            t,
            opts,
        ));
        out.push(dimension_line(
            Point { x: min_x, y: min_y },
            Point { x: min_x, y: max_y },
            opts.dimension_offset,
            &format_dimension(max_y - min_y),
            "left",
            t,
            opts,
        ));
    }
    if opts.show_footprint_dimensions {
        let (min_x, max_x, min_y, max_y) = bounds(&geometry.footprint.points);
        out.push(dimension_line(
            Point { x: min_x, y: min_y },
            Point { x: max_x, y: min_y },
            opts.dimension_offset + 25.0,
            &format_dimension(max_x - min_x),
            "bottom",
            t,
            opts,
        ));
        out.push(dimension_line(
            Point { x: min_x, y: min_y },
            Point { x: min_x, y: max_y },
            opts.dimension_offset + 25.0,
            &format_dimension(max_y - min_y),
            "left",
            t,
            opts,
        ));
    }
    out.join("\n    ")
}

fn dimension_line(
    p1: Point,
    p2: Point,
    offset: f64,
    label: &str,
    side: &str,
    t: Transform,
    opts: &SvgOptions,
) -> String {
    let sp1 = transform_point(p1, t);
    let sp2 = transform_point(p2, t);
    let (dp1, dp2, text_x, text_y, rotation) = match side {
        "bottom" => {
            let y = offset;
            (
                Point {
                    x: sp1.x,
                    y: sp1.y + y,
                },
                Point {
                    x: sp2.x,
                    y: sp2.y + y,
                },
                (sp1.x + sp2.x) / 2.0,
                sp1.y + y,
                0.0,
            )
        }
        "left" => {
            let x = -offset;
            (
                Point {
                    x: sp1.x + x,
                    y: sp1.y,
                },
                Point {
                    x: sp2.x + x,
                    y: sp2.y,
                },
                sp1.x + x,
                (sp1.y + sp2.y) / 2.0,
                -90.0,
            )
        }
        _ => (sp1, sp2, (sp1.x + sp2.x) / 2.0, (sp1.y + sp2.y) / 2.0, 0.0),
    };
    let transform = if rotation != 0.0 {
        format!(
            r#" transform="rotate({}, {:.2}, {:.2})""#,
            rotation, text_x, text_y
        )
    } else {
        String::new()
    };
    format!(
        r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="1" />
    <text x="{:.2}" y="{:.2}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif"{}>{}</text>"#,
        dp1.x,
        dp1.y,
        dp2.x,
        dp2.y,
        opts.dimension_color,
        text_x,
        text_y,
        number(opts.dimension_font_size),
        opts.dimension_color,
        transform,
        escape_xml(label)
    )
}

fn generate_compass_svg(site: Option<SiteInfo>, opts: &SvgOptions) -> String {
    if !opts.show_compass {
        return String::new();
    }
    let Some(site) = site else {
        return String::new();
    };
    let size = opts.compass_size;
    let center_x = opts.width - opts.padding - size / 2.0;
    let center_y = opts.padding + size / 2.0;
    let arrow = size * 0.4;
    let label = size * 0.55;
    let (street_x, street_y) = street_label_position(site.street, center_x, center_y, arrow);
    format!(
        r#"<g class="compass">
    <circle cx="{cx:.2}" cy="{cy:.2}" r="{r:.2}" fill="none" stroke="{color}" stroke-width="1" opacity="0.3" />
    <polygon points="{cx:.2},{nt:.2} {nl:.2},{cy:.2} {nr:.2},{cy:.2}" fill="{color}" />
    <line x1="{wl:.2}" y1="{cy:.2}" x2="{er:.2}" y2="{cy:.2}" stroke="{color}" stroke-width="1" />
    <text x="{cx:.2}" y="{ny:.2}" font-size="{fs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif" font-weight="bold">N</text>
    <text x="{cx:.2}" y="{sy:.2}" font-size="{sfs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">S</text>
    <text x="{ex:.2}" y="{cy:.2}" font-size="{sfs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">E</text>
    <text x="{wx:.2}" y="{cy:.2}" font-size="{sfs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">W</text>
    <text x="{sx:.2}" y="{sty:.2}" font-size="{stfs:.2}" fill="{street_color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">STREET</text>
  </g>"#,
        cx = center_x,
        cy = center_y,
        r = size / 2.0,
        color = opts.compass_color,
        nt = center_y - arrow,
        nl = center_x - size * 0.075,
        nr = center_x + size * 0.075,
        wl = center_x - arrow * 0.7,
        er = center_x + arrow * 0.7,
        ny = center_y - label,
        sy = center_y + label,
        ex = center_x + label,
        wx = center_x - label,
        fs = size * 0.2,
        sfs = size * 0.16,
        sx = street_x,
        sty = street_y,
        stfs = size * 0.12,
        street_color = opts.street_indicator_color,
    )
}

fn street_label_position(dir: CardinalDirection, x: f64, y: f64, arrow: f64) -> (f64, f64) {
    match dir {
        CardinalDirection::North => (x, y - arrow - 7.0),
        CardinalDirection::South => (x, y + arrow + 7.0),
        CardinalDirection::East => (x + arrow + 12.0, y),
        CardinalDirection::West => (x - arrow - 12.0, y),
    }
}

fn polygon_center(points: &[Point]) -> Point {
    if points.is_empty() {
        return Point { x: 0.0, y: 0.0 };
    }
    let sum = points
        .iter()
        .fold(Point { x: 0.0, y: 0.0 }, |acc, p| Point {
            x: acc.x + p.x,
            y: acc.y + p.y,
        });
    Point {
        x: sum.x / points.len() as f64,
        y: sum.y / points.len() as f64,
    }
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

fn format_dimension(meters: f64) -> String {
    if meters >= 1.0 {
        let rounded = (meters * 10.0).round() / 10.0;
        if (rounded - rounded.round()).abs() < 1e-9 {
            format!("{}m", rounded.round() as i64)
        } else {
            format!("{rounded:.1}m")
        }
    } else {
        format!("{}cm", (meters * 100.0).round() as i64)
    }
}

fn number(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value}")
    }
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
