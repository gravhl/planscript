use crate::ast::{CardinalDirection, DoorSwing, Point, Program};
use crate::geometry::{
    GeometryIr, OpeningPlacementType, Polygon, ResolvedCourtyard, ResolvedObject, ResolvedRoom,
    WallSegment,
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
                .unwrap_or_else(|| "#7f8c8d".to_string()),
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

    let svg = format!(
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
    );
    strip_blank_line_whitespace(&svg)
}

fn strip_blank_line_whitespace(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for (index, line) in input.lines().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        if !line.trim().is_empty() {
            out.push_str(line);
        }
    }
    out
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

fn rotate_point(point: Point, rotation_degrees: f64) -> Point {
    let radians = rotation_degrees.to_radians();
    let cos = radians.cos();
    let sin = radians.sin();
    Point {
        x: point.x * cos - point.y * sin,
        y: point.x * sin + point.y * cos,
    }
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
                r#"<path class="object-clearance" d="{}" fill="{}" fill-opacity="0.08" stroke="{}" stroke-opacity="0.28" stroke-width="0.75" stroke-dasharray="1,4" />"#,
                points_to_path(&points),
                opts.object_clearance_color,
                opts.object_clearance_color
            ));
        }
    }
    for object in &geometry.objects {
        out.push(render_object_svg(object, t, opts));
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

fn render_object_svg(object: &ResolvedObject, t: Transform, opts: &SvgOptions) -> String {
    if let Some(fixture) = render_builtin_fixture_svg(object, t, opts) {
        return fixture;
    }

    let points = transform_polygon(&object.polygon.points, t);
    format!(
        r#"<path data-object="{}" data-catalog-id="{}" d="{}" fill="{}" stroke="{}" stroke-width="1.5" />"#,
        escape_xml(&object.name),
        escape_xml(&object.catalog_id),
        points_to_path(&points),
        opts.object_fill_color,
        opts.object_stroke_color
    )
}

fn render_builtin_fixture_svg(
    object: &ResolvedObject,
    t: Transform,
    opts: &SvgOptions,
) -> Option<String> {
    let kind = match object.catalog_id.as_str() {
        "builtin.sanitary.toilet.floor_mounted" => "toilet",
        "builtin.sanitary.sink.wall_hung" => "bath-sink",
        "builtin.sanitary.shower.size_900x900" => "shower",
        "builtin.sanitary.tub.size_1700" => "tub",
        "builtin.kitchen.sink" => "kitchen-sink",
        "builtin.kitchen.range.size_600" => "range",
        "builtin.kitchen.fridge.size_900" => "fridge",
        "builtin.laundry.washer" => "washer",
        "builtin.laundry.dryer" => "dryer",
        "builtin.furniture.bed.queen" => "bed",
        "builtin.furniture.sofa.three_seat" => "sofa",
        "builtin.furniture.table.dining_6" => "dining-table",
        _ => return None,
    };

    let dims = object_local_dimensions(object);
    let mut parts = vec![fixture_base_path(object, kind, t, opts)];
    match kind {
        "toilet" => render_toilet_details(object, dims, t, opts, &mut parts),
        "bath-sink" => render_bath_sink_details(object, dims, t, opts, &mut parts),
        "shower" => render_shower_details(object, dims, t, opts, &mut parts),
        "tub" => render_tub_details(object, dims, t, opts, &mut parts),
        "kitchen-sink" => render_kitchen_sink_details(object, dims, t, opts, &mut parts),
        "range" => render_range_details(object, dims, t, opts, &mut parts),
        "fridge" => render_fridge_details(object, dims, t, opts, &mut parts),
        "washer" => render_appliance_drum_details(object, dims, t, opts, &mut parts, "washer"),
        "dryer" => render_appliance_drum_details(object, dims, t, opts, &mut parts, "dryer"),
        "bed" => render_bed_details(object, dims, t, opts, &mut parts),
        "sofa" => render_sofa_details(object, dims, t, opts, &mut parts),
        "dining-table" => render_dining_table_details(object, dims, t, opts, &mut parts),
        _ => {}
    }

    Some(format!(
        r#"<g class="fixture fixture-{}" data-object="{}" data-catalog-id="{}">
      {}
    </g>"#,
        kind,
        escape_xml(&object.name),
        escape_xml(&object.catalog_id),
        parts.join("\n      ")
    ))
}

#[derive(Debug, Clone, Copy)]
struct ObjectDimensions {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    width: f64,
    depth: f64,
}

fn object_local_dimensions(object: &ResolvedObject) -> ObjectDimensions {
    let mut local = object
        .polygon
        .points
        .iter()
        .map(|point| {
            rotate_point(
                Point {
                    x: point.x - object.origin.x,
                    y: point.y - object.origin.y,
                },
                -object.rotation,
            )
        })
        .collect::<Vec<_>>();
    if local.is_empty() {
        local.push(Point { x: -0.25, y: 0.0 });
        local.push(Point { x: 0.25, y: 0.5 });
    }
    let (min_x, max_x, min_y, max_y) = bounds(&local);
    ObjectDimensions {
        min_x,
        max_x,
        min_y,
        max_y,
        width: (max_x - min_x).max(0.001),
        depth: (max_y - min_y).max(0.001),
    }
}

fn fixture_base_path(
    object: &ResolvedObject,
    kind: &str,
    t: Transform,
    opts: &SvgOptions,
) -> String {
    let points = transform_polygon(&object.polygon.points, t);
    format!(
        r#"<path class="fixture-base fixture-{}" d="{}" fill="{}" stroke="{}" stroke-width="1.5" />"#,
        kind,
        points_to_path(&points),
        opts.object_fill_color,
        opts.object_stroke_color
    )
}

fn render_toilet_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    let cx = (d.min_x + d.max_x) / 2.0;
    out.push(fixture_rect(
        object,
        cx - d.width * 0.36,
        d.min_y + d.depth * 0.05,
        cx + d.width * 0.36,
        d.min_y + d.depth * 0.22,
        "fixture-detail fixture-toilet-tank",
        t,
        opts,
    ));
    out.push(fixture_ellipse(
        object,
        cx,
        d.min_y + d.depth * 0.52,
        d.width * 0.33,
        d.depth * 0.26,
        "fixture-detail fixture-toilet-bowl",
        t,
        opts,
    ));
    out.push(fixture_ellipse(
        object,
        cx,
        d.min_y + d.depth * 0.53,
        d.width * 0.18,
        d.depth * 0.13,
        "fixture-detail fixture-toilet-water",
        t,
        opts,
    ));
}

fn render_bath_sink_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    let cx = (d.min_x + d.max_x) / 2.0;
    let cy = d.min_y + d.depth * 0.48;
    out.push(fixture_ellipse(
        object,
        cx,
        cy,
        d.width * 0.34,
        d.depth * 0.30,
        "fixture-detail fixture-sink-basin",
        t,
        opts,
    ));
    out.push(fixture_ellipse(
        object,
        cx,
        cy,
        d.width * 0.22,
        d.depth * 0.18,
        "fixture-detail fixture-sink-inner",
        t,
        opts,
    ));
    out.push(fixture_ellipse(
        object,
        cx,
        d.min_y + d.depth * 0.50,
        d.width * 0.035,
        d.width * 0.035,
        "fixture-detail fixture-drain",
        t,
        opts,
    ));
    out.push(fixture_line(
        object,
        cx,
        d.min_y + d.depth * 0.12,
        cx,
        d.min_y + d.depth * 0.30,
        "fixture-detail fixture-faucet",
        t,
        opts,
    ));
}

fn render_shower_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    out.push(fixture_line(
        object,
        d.min_x + d.width * 0.12,
        d.min_y + d.depth * 0.12,
        d.max_x - d.width * 0.12,
        d.max_y - d.depth * 0.12,
        "fixture-detail fixture-shower-slope",
        t,
        opts,
    ));
    out.push(fixture_ellipse(
        object,
        d.max_x - d.width * 0.22,
        d.min_y + d.depth * 0.22,
        d.width * 0.055,
        d.width * 0.055,
        "fixture-detail fixture-drain",
        t,
        opts,
    ));
    out.push(fixture_ellipse(
        object,
        d.min_x + d.width * 0.20,
        d.max_y - d.depth * 0.18,
        d.width * 0.06,
        d.width * 0.06,
        "fixture-detail fixture-shower-head",
        t,
        opts,
    ));
}

fn render_tub_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    out.push(fixture_rect(
        object,
        d.min_x + d.width * 0.08,
        d.min_y + d.depth * 0.12,
        d.max_x - d.width * 0.08,
        d.max_y - d.depth * 0.12,
        "fixture-detail fixture-tub-basin",
        t,
        opts,
    ));
    out.push(fixture_ellipse(
        object,
        d.max_x - d.width * 0.18,
        d.min_y + d.depth * 0.50,
        d.width * 0.035,
        d.width * 0.035,
        "fixture-detail fixture-drain",
        t,
        opts,
    ));
}

fn render_kitchen_sink_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    let gap = d.width * 0.04;
    let basin_w = d.width * 0.33;
    let y1 = d.min_y + d.depth * 0.18;
    let y2 = d.max_y - d.depth * 0.18;
    let cx = (d.min_x + d.max_x) / 2.0;
    out.push(fixture_rect(
        object,
        cx - gap - basin_w,
        y1,
        cx - gap,
        y2,
        "fixture-detail fixture-sink-basin",
        t,
        opts,
    ));
    out.push(fixture_rect(
        object,
        cx + gap,
        y1,
        cx + gap + basin_w,
        y2,
        "fixture-detail fixture-sink-basin",
        t,
        opts,
    ));
    for x in [cx - gap - basin_w / 2.0, cx + gap + basin_w / 2.0] {
        out.push(fixture_ellipse(
            object,
            x,
            (y1 + y2) / 2.0,
            d.width * 0.025,
            d.width * 0.025,
            "fixture-detail fixture-drain",
            t,
            opts,
        ));
    }
    out.push(fixture_line(
        object,
        cx,
        d.min_y + d.depth * 0.07,
        cx,
        d.min_y + d.depth * 0.22,
        "fixture-detail fixture-faucet",
        t,
        opts,
    ));
}

fn render_range_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    for (x, y) in [(0.32, 0.35), (0.68, 0.35), (0.32, 0.66), (0.68, 0.66)] {
        out.push(fixture_ellipse(
            object,
            d.min_x + d.width * x,
            d.min_y + d.depth * y,
            d.width * 0.10,
            d.width * 0.10,
            "fixture-detail fixture-burner",
            t,
            opts,
        ));
    }
    out.push(fixture_line(
        object,
        d.min_x + d.width * 0.12,
        d.min_y + d.depth * 0.18,
        d.max_x - d.width * 0.12,
        d.min_y + d.depth * 0.18,
        "fixture-detail fixture-control-line",
        t,
        opts,
    ));
}

fn render_fridge_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    let x = d.min_x + d.width * 0.52;
    out.push(fixture_line(
        object,
        x,
        d.min_y + d.depth * 0.08,
        x,
        d.max_y - d.depth * 0.08,
        "fixture-detail fixture-door-split",
        t,
        opts,
    ));
    out.push(fixture_line(
        object,
        d.min_x + d.width * 0.62,
        d.min_y + d.depth * 0.25,
        d.min_x + d.width * 0.62,
        d.max_y - d.depth * 0.25,
        "fixture-detail fixture-handle",
        t,
        opts,
    ));
}

fn render_appliance_drum_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
    kind: &str,
) {
    let cx = (d.min_x + d.max_x) / 2.0;
    let cy = d.min_y + d.depth * 0.55;
    out.push(fixture_ellipse(
        object,
        cx,
        cy,
        d.width * 0.24,
        d.width * 0.24,
        &format!("fixture-detail fixture-{kind}-drum"),
        t,
        opts,
    ));
    out.push(fixture_line(
        object,
        d.min_x + d.width * 0.12,
        d.min_y + d.depth * 0.16,
        d.max_x - d.width * 0.12,
        d.min_y + d.depth * 0.16,
        &format!("fixture-detail fixture-{kind}-controls"),
        t,
        opts,
    ));
}

fn render_bed_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    out.push(fixture_rect(
        object,
        d.min_x + d.width * 0.08,
        d.min_y + d.depth * 0.08,
        d.max_x - d.width * 0.08,
        d.max_y - d.depth * 0.08,
        "fixture-detail fixture-mattress",
        t,
        opts,
    ));
    for x1 in [d.min_x + d.width * 0.12, d.min_x + d.width * 0.52] {
        out.push(fixture_rect(
            object,
            x1,
            d.min_y + d.depth * 0.10,
            x1 + d.width * 0.36,
            d.min_y + d.depth * 0.26,
            "fixture-detail fixture-pillow",
            t,
            opts,
        ));
    }
}

fn render_sofa_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    out.push(fixture_rect(
        object,
        d.min_x + d.width * 0.05,
        d.min_y + d.depth * 0.08,
        d.max_x - d.width * 0.05,
        d.min_y + d.depth * 0.25,
        "fixture-detail fixture-sofa-back",
        t,
        opts,
    ));
    out.push(fixture_rect(
        object,
        d.min_x + d.width * 0.05,
        d.min_y + d.depth * 0.25,
        d.max_x - d.width * 0.05,
        d.max_y - d.depth * 0.10,
        "fixture-detail fixture-sofa-seat",
        t,
        opts,
    ));
    for x in [0.36, 0.64] {
        out.push(fixture_line(
            object,
            d.min_x + d.width * x,
            d.min_y + d.depth * 0.28,
            d.min_x + d.width * x,
            d.max_y - d.depth * 0.12,
            "fixture-detail fixture-sofa-cushion",
            t,
            opts,
        ));
    }
}

fn render_dining_table_details(
    object: &ResolvedObject,
    d: ObjectDimensions,
    t: Transform,
    opts: &SvgOptions,
    out: &mut Vec<String>,
) {
    let cx = (d.min_x + d.max_x) / 2.0;
    let cy = (d.min_y + d.max_y) / 2.0;
    out.push(fixture_ellipse(
        object,
        cx,
        cy,
        d.width * 0.34,
        d.depth * 0.28,
        "fixture-detail fixture-table-top",
        t,
        opts,
    ));
    let chair_w = d.width * 0.11;
    let chair_d = d.depth * 0.16;
    for x in [0.25, 0.50, 0.75] {
        out.push(fixture_rect(
            object,
            d.min_x + d.width * x - chair_w / 2.0,
            d.min_y + d.depth * 0.06,
            d.min_x + d.width * x + chair_w / 2.0,
            d.min_y + d.depth * 0.06 + chair_d,
            "fixture-detail fixture-chair",
            t,
            opts,
        ));
        out.push(fixture_rect(
            object,
            d.min_x + d.width * x - chair_w / 2.0,
            d.max_y - d.depth * 0.06 - chair_d,
            d.min_x + d.width * x + chair_w / 2.0,
            d.max_y - d.depth * 0.06,
            "fixture-detail fixture-chair",
            t,
            opts,
        ));
    }
}

fn fixture_rect(
    object: &ResolvedObject,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    class_name: &str,
    t: Transform,
    opts: &SvgOptions,
) -> String {
    fixture_path(
        object,
        &[
            Point { x: x1, y: y1 },
            Point { x: x2, y: y1 },
            Point { x: x2, y: y2 },
            Point { x: x1, y: y2 },
        ],
        class_name,
        t,
        opts,
    )
}

fn fixture_ellipse(
    object: &ResolvedObject,
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    class_name: &str,
    t: Transform,
    opts: &SvgOptions,
) -> String {
    let points = (0..24)
        .map(|i| {
            let angle = i as f64 / 24.0 * std::f64::consts::PI * 2.0;
            Point {
                x: cx + angle.cos() * rx,
                y: cy + angle.sin() * ry,
            }
        })
        .collect::<Vec<_>>();
    fixture_path(object, &points, class_name, t, opts)
}

fn fixture_path(
    object: &ResolvedObject,
    local_points: &[Point],
    class_name: &str,
    t: Transform,
    opts: &SvgOptions,
) -> String {
    let points = local_points
        .iter()
        .map(|point| transform_point(local_to_plan(object, *point), t))
        .collect::<Vec<_>>();
    format!(
        r##"<path class="{}" d="{}" fill="#ffffff" fill-opacity="0.42" stroke="{}" stroke-width="1" />"##,
        class_name,
        points_to_path(&points),
        opts.object_stroke_color
    )
}

fn fixture_line(
    object: &ResolvedObject,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    class_name: &str,
    t: Transform,
    opts: &SvgOptions,
) -> String {
    let p1 = transform_point(local_to_plan(object, Point { x: x1, y: y1 }), t);
    let p2 = transform_point(local_to_plan(object, Point { x: x2, y: y2 }), t);
    format!(
        r#"<line class="{}" x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="1" stroke-linecap="round" />"#,
        class_name, p1.x, p1.y, p2.x, p2.y, opts.object_stroke_color
    )
}

fn local_to_plan(object: &ResolvedObject, point: Point) -> Point {
    let rotated = rotate_point(point, object.rotation);
    Point {
        x: object.origin.x + rotated.x,
        y: object.origin.y + rotated.y,
    }
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
        let plan_p1 = Point {
            x: center.x - ux * half,
            y: center.y - uy * half,
        };
        let plan_p2 = Point {
            x: center.x + ux * half,
            y: center.y + uy * half,
        };
        let p1 = transform_point(plan_p1, t);
        let p2 = transform_point(plan_p2, t);
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
        if opening.opening_type == OpeningPlacementType::Door
            && (opening.swing.is_some() || opening.double)
        {
            out.push(render_door_swing(opening, plan_p1, plan_p2, t, color));
        }
    }
    out.join("\n    ")
}

fn render_door_swing(
    opening: &crate::geometry::OpeningPlacement,
    plan_p1: Point,
    plan_p2: Point,
    t: Transform,
    color: &str,
) -> String {
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
        let center = midpoint(plan_p1, plan_p2);
        let leaf_width = opening.width / 2.0;
        return [
            render_door_leaf(plan_p1, center, open_normal, leaf_width, t, color),
            render_door_leaf(plan_p2, center, open_normal, leaf_width, t, color),
        ]
        .join("\n    ");
    }

    let swing = opening.swing.unwrap_or(DoorSwing::LeftHand);
    let (left, right) = left_right_points_from_outside(plan_p1, plan_p2, outside);
    let (hinge, closed_free) = if swing.hinge_is_left() {
        (left, right)
    } else {
        (right, left)
    };
    render_door_leaf(hinge, closed_free, open_normal, opening.width, t, color)
}

fn render_door_leaf(
    hinge: Point,
    closed_free: Point,
    open_normal: Point,
    leaf_width: f64,
    t: Transform,
    color: &str,
) -> String {
    let open_free = Point {
        x: hinge.x + open_normal.x * leaf_width,
        y: hinge.y + open_normal.y * leaf_width,
    };
    let svg_hinge = transform_point(hinge, t);
    let svg_closed = transform_point(closed_free, t);
    let svg_open = transform_point(open_free, t);
    let radius = leaf_width * t.scale;
    let sweep = arc_sweep(svg_hinge, svg_closed, svg_open);
    format!(
        r#"<line class="door-leaf" x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="1.5" stroke-linecap="round" />
    <path class="door-swing" d="M {:.2} {:.2} A {:.2} {:.2} 0 0 {} {:.2} {:.2}" fill="none" stroke="{}" stroke-width="1" stroke-dasharray="2,2" opacity="0.8" />"#,
        svg_hinge.x,
        svg_hinge.y,
        svg_open.x,
        svg_open.y,
        color,
        svg_closed.x,
        svg_closed.y,
        radius,
        radius,
        sweep,
        svg_open.x,
        svg_open.y,
        color
    )
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

fn midpoint(a: Point, b: Point) -> Point {
    Point {
        x: (a.x + b.x) / 2.0,
        y: (a.y + b.y) / 2.0,
    }
}

fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

fn arc_sweep(hinge: Point, closed: Point, open: Point) -> u8 {
    let start = Point {
        x: closed.x - hinge.x,
        y: closed.y - hinge.y,
    };
    let end = Point {
        x: open.x - hinge.x,
        y: open.y - hinge.y,
    };
    if start.x * end.y - start.y * end.x > 0.0 {
        1
    } else {
        0
    }
}

fn generate_labels_svg(geometry: &GeometryIr, t: Transform, opts: &SvgOptions) -> String {
    if !opts.show_labels {
        return String::new();
    }
    let obstacles = label_obstacles(geometry, t);
    let mut placed = Vec::new();
    let mut out = Vec::new();

    for room in &geometry.rooms {
        if let Some(label) = &room.label {
            let polygon = transform_polygon(&room.polygon.points, t);
            let candidate = best_area_label_candidate(
                label,
                opts.label_font_size,
                &polygon,
                &obstacles,
                &placed,
                opts.width,
                opts.height,
            );
            placed.push(candidate.rect);
            out.push(format!(
                r#"<text class="room-label" data-room-label="{}" x="{:.2}" y="{:.2}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">{}</text>"#,
                escape_xml(&room.name),
                candidate.x,
                candidate.y,
                number(opts.label_font_size),
                opts.label_color,
                escape_xml(label)
            ));
        }
    }
    for courtyard in &geometry.courtyards {
        if let Some(label) = &courtyard.label {
            let polygon = transform_polygon(&courtyard.polygon.points, t);
            let candidate = best_area_label_candidate(
                label,
                opts.label_font_size,
                &polygon,
                &obstacles,
                &placed,
                opts.width,
                opts.height,
            );
            placed.push(candidate.rect);
            out.push(format!(
                r#"<text class="courtyard-label" data-courtyard-label="{}" x="{:.2}" y="{:.2}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif" font-style="italic">{}</text>"#,
                escape_xml(&courtyard.name),
                candidate.x,
                candidate.y,
                number(opts.label_font_size),
                opts.courtyard_stroke_color,
                escape_xml(label)
            ));
        }
    }
    out.extend(generate_object_labels_svg(
        geometry,
        t,
        opts,
        &obstacles,
        &mut placed,
    ));
    out.join("\n    ")
}

#[derive(Debug, Clone, Copy)]
struct LabelRect {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

#[derive(Debug, Clone, Copy)]
struct LabelCandidate {
    x: f64,
    y: f64,
    rotation: f64,
    rect: LabelRect,
    preference: f64,
}

fn generate_object_labels_svg(
    geometry: &GeometryIr,
    t: Transform,
    opts: &SvgOptions,
    obstacles: &[LabelRect],
    placed: &mut Vec<LabelRect>,
) -> Vec<String> {
    let font_size = (opts.label_font_size * 0.72).max(7.0);
    let mut out = Vec::new();

    for object in &geometry.objects {
        let Some(label) = &object.label else {
            continue;
        };
        let object_rect = rect_for_points(&transform_polygon(&object.polygon.points, t));
        let candidate = best_label_candidate(
            label,
            font_size,
            object_rect,
            obstacles,
            placed,
            opts.width,
            opts.height,
        );
        placed.push(candidate.rect);

        let transform = if candidate.rotation.abs() > f64::EPSILON {
            format!(
                r#" transform="rotate({:.0}, {:.2}, {:.2})""#,
                candidate.rotation, candidate.x, candidate.y
            )
        } else {
            String::new()
        };
        out.push(format!(
            r#"<text class="fixture-label" data-object-label="{}" x="{:.2}" y="{:.2}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif"{}>{}</text>"#,
            escape_xml(&object.name),
            candidate.x,
            candidate.y,
            number(font_size),
            opts.object_stroke_color,
            transform,
            escape_xml(label)
        ));
    }

    out
}

fn best_area_label_candidate(
    label: &str,
    font_size: f64,
    polygon: &[Point],
    obstacles: &[LabelRect],
    placed: &[LabelRect],
    canvas_width: f64,
    canvas_height: f64,
) -> LabelCandidate {
    let candidates = area_label_candidates(label, font_size, polygon);
    let inside = candidates
        .iter()
        .copied()
        .filter(|candidate| rect_inside_polygon(candidate.rect, polygon))
        .collect::<Vec<_>>();
    let candidates = if inside.is_empty() {
        candidates
    } else {
        inside
    };

    candidates
        .into_iter()
        .min_by(|a, b| {
            let a_score = label_score(a, obstacles, placed, canvas_width, canvas_height);
            let b_score = label_score(b, obstacles, placed, canvas_width, canvas_height);
            a_score
                .partial_cmp(&b_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .expect("area label candidates")
}

fn area_label_candidates(label: &str, font_size: f64, polygon: &[Point]) -> Vec<LabelCandidate> {
    let text_width = estimate_text_width(label, font_size);
    let text_height = font_size * 1.15;
    if polygon.is_empty() {
        return vec![label_candidate(0.0, 0.0, 0.0, text_width, text_height, 0.0)];
    }

    let center = polygon_center(polygon);
    let (min_x, max_x, min_y, max_y) = bounds(polygon);
    let diagonal = distance(Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y }).max(1.0);

    let mut out = vec![label_candidate(
        center.x,
        center.y,
        0.0,
        text_width,
        text_height,
        0.0,
    )];

    let fractions = [0.5, 0.35, 0.65, 0.2, 0.8, 0.1, 0.9];
    for fy in fractions {
        for fx in fractions {
            let point = Point {
                x: min_x + (max_x - min_x) * fx,
                y: min_y + (max_y - min_y) * fy,
            };
            if !point_in_polygon_or_boundary(point, polygon) {
                continue;
            }
            let preference = 1.0 + distance(point, center) / diagonal * 4.0;
            out.push(label_candidate(
                point.x,
                point.y,
                0.0,
                text_width,
                text_height,
                preference,
            ));
        }
    }

    out
}

fn best_label_candidate(
    label: &str,
    font_size: f64,
    object_rect: LabelRect,
    obstacles: &[LabelRect],
    placed: &[LabelRect],
    canvas_width: f64,
    canvas_height: f64,
) -> LabelCandidate {
    let candidates = label_candidates(label, font_size, object_rect);
    candidates
        .into_iter()
        .min_by(|a, b| {
            let a_score = label_score(a, obstacles, placed, canvas_width, canvas_height);
            let b_score = label_score(b, obstacles, placed, canvas_width, canvas_height);
            a_score
                .partial_cmp(&b_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .expect("label candidates")
}

fn label_candidates(label: &str, font_size: f64, object_rect: LabelRect) -> Vec<LabelCandidate> {
    let text_width = estimate_text_width(label, font_size);
    let text_height = font_size * 1.15;
    let gap = (font_size * 0.45).max(4.0);
    let cx = (object_rect.min_x + object_rect.max_x) / 2.0;
    let cy = (object_rect.min_y + object_rect.max_y) / 2.0;

    let mut out = Vec::new();
    let horizontal = [
        (cx, object_rect.max_y + gap + text_height / 2.0, 0.0, 0.0),
        (cx, object_rect.min_y - gap - text_height / 2.0, 0.0, 1.0),
        (
            object_rect.min_x + text_width / 2.0,
            object_rect.max_y + gap + text_height / 2.0,
            0.0,
            2.0,
        ),
        (
            object_rect.max_x - text_width / 2.0,
            object_rect.max_y + gap + text_height / 2.0,
            0.0,
            2.2,
        ),
        (
            object_rect.min_x + text_width / 2.0,
            object_rect.min_y - gap - text_height / 2.0,
            0.0,
            3.0,
        ),
        (
            object_rect.max_x - text_width / 2.0,
            object_rect.min_y - gap - text_height / 2.0,
            0.0,
            3.2,
        ),
    ];
    for (x, y, rotation, preference) in horizontal {
        out.push(label_candidate(
            x,
            y,
            rotation,
            text_width,
            text_height,
            preference,
        ));
    }

    let rotated_width = text_height;
    let rotated_height = text_width;
    let vertical = [
        (object_rect.max_x + gap + rotated_width / 2.0, cy, 90.0, 4.0),
        (
            object_rect.min_x - gap - rotated_width / 2.0,
            cy,
            -90.0,
            4.2,
        ),
        (
            object_rect.max_x + gap + rotated_width / 2.0,
            object_rect.min_y + rotated_height / 2.0,
            90.0,
            5.0,
        ),
        (
            object_rect.max_x + gap + rotated_width / 2.0,
            object_rect.max_y - rotated_height / 2.0,
            90.0,
            5.2,
        ),
        (
            object_rect.min_x - gap - rotated_width / 2.0,
            object_rect.min_y + rotated_height / 2.0,
            -90.0,
            5.4,
        ),
        (
            object_rect.min_x - gap - rotated_width / 2.0,
            object_rect.max_y - rotated_height / 2.0,
            -90.0,
            5.6,
        ),
    ];
    for (x, y, rotation, preference) in vertical {
        out.push(LabelCandidate {
            x,
            y,
            rotation,
            rect: LabelRect {
                min_x: x - rotated_width / 2.0,
                max_x: x + rotated_width / 2.0,
                min_y: y - rotated_height / 2.0,
                max_y: y + rotated_height / 2.0,
            },
            preference,
        });
    }

    out
}

fn label_candidate(
    x: f64,
    y: f64,
    rotation: f64,
    width: f64,
    height: f64,
    preference: f64,
) -> LabelCandidate {
    LabelCandidate {
        x,
        y,
        rotation,
        rect: LabelRect {
            min_x: x - width / 2.0,
            max_x: x + width / 2.0,
            min_y: y - height / 2.0,
            max_y: y + height / 2.0,
        },
        preference,
    }
}

fn label_score(
    candidate: &LabelCandidate,
    obstacles: &[LabelRect],
    placed: &[LabelRect],
    canvas_width: f64,
    canvas_height: f64,
) -> f64 {
    let mut score = candidate.preference;
    score += overflow_penalty(candidate.rect, canvas_width, canvas_height) * 1000.0;
    for obstacle in obstacles {
        score += overlap_area(candidate.rect, *obstacle) * 250.0;
    }
    for label in placed {
        score += overlap_area(candidate.rect, *label) * 800.0;
    }
    score
}

fn label_obstacles(geometry: &GeometryIr, t: Transform) -> Vec<LabelRect> {
    let mut obstacles = Vec::new();
    for object in &geometry.objects {
        obstacles.push(rect_for_points(&transform_polygon(&object.polygon.points, t)).inflate(2.0));
    }
    for wall in &geometry.walls {
        let start = transform_point(wall.start, t);
        let end = transform_point(wall.end, t);
        obstacles.push(rect_for_line(start, end).inflate(3.0));
    }
    for opening in &geometry.openings {
        if let Some(wall) = geometry
            .walls
            .iter()
            .find(|wall| wall.id == opening.wall_id)
        {
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
            obstacles.push(rect_for_line(p1, p2).inflate(5.0));
        }
    }
    obstacles
}

impl LabelRect {
    fn inflate(self, amount: f64) -> Self {
        Self {
            min_x: self.min_x - amount,
            max_x: self.max_x + amount,
            min_y: self.min_y - amount,
            max_y: self.max_y + amount,
        }
    }
}

fn rect_for_points(points: &[Point]) -> LabelRect {
    let (min_x, max_x, min_y, max_y) = bounds(points);
    LabelRect {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

fn rect_for_line(a: Point, b: Point) -> LabelRect {
    LabelRect {
        min_x: a.x.min(b.x),
        max_x: a.x.max(b.x),
        min_y: a.y.min(b.y),
        max_y: a.y.max(b.y),
    }
}

fn rect_inside_polygon(rect: LabelRect, polygon: &[Point]) -> bool {
    if polygon.is_empty() {
        return false;
    }

    let center = Point {
        x: (rect.min_x + rect.max_x) / 2.0,
        y: (rect.min_y + rect.max_y) / 2.0,
    };
    let points = [
        center,
        Point {
            x: rect.min_x,
            y: rect.min_y,
        },
        Point {
            x: rect.max_x,
            y: rect.min_y,
        },
        Point {
            x: rect.max_x,
            y: rect.max_y,
        },
        Point {
            x: rect.min_x,
            y: rect.max_y,
        },
    ];

    points
        .iter()
        .all(|point| point_in_polygon_or_boundary(*point, polygon))
}

fn point_in_polygon_or_boundary(point: Point, polygon: &[Point]) -> bool {
    point_in_polygon(point, polygon) || point_on_polygon_boundary(point, polygon, 0.01)
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
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        if (distance(point, a) + distance(point, b) - distance(a, b)).abs() <= epsilon {
            return true;
        }
    }
    false
}

fn distance(a: Point, b: Point) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn overlap_area(a: LabelRect, b: LabelRect) -> f64 {
    let width = (a.max_x.min(b.max_x) - a.min_x.max(b.min_x)).max(0.0);
    let height = (a.max_y.min(b.max_y) - a.min_y.max(b.min_y)).max(0.0);
    width * height
}

fn overflow_penalty(rect: LabelRect, canvas_width: f64, canvas_height: f64) -> f64 {
    let left = (-rect.min_x).max(0.0);
    let right = (rect.max_x - canvas_width).max(0.0);
    let top = (-rect.min_y).max(0.0);
    let bottom = (rect.max_y - canvas_height).max(0.0);
    left + right + top + bottom
}

fn estimate_text_width(label: &str, font_size: f64) -> f64 {
    label.chars().count() as f64 * font_size * 0.56
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
    let arrow_width = size * 0.15;
    let label = size * 0.55;
    let (street_x, street_y, street_rotation) =
        street_label_position(site.street, center_x, center_y, arrow);
    let street_transform = if street_rotation != 0.0 {
        format!(
            r#" transform="rotate({}, {:.2}, {:.2})""#,
            number(street_rotation),
            street_x,
            street_y
        )
    } else {
        String::new()
    };
    format!(
        r#"<g class="compass">
    <circle cx="{cx:.2}" cy="{cy:.2}" r="{r:.2}" fill="none" stroke="{color}" stroke-width="1" opacity="0.3" />
    <polygon points="{cx:.2},{nt:.2} {nl:.2},{cy:.2} {nr:.2},{cy:.2}" fill="{color}" />
    <polygon points="{cx:.2},{st:.2} {nl:.2},{cy:.2} {nr:.2},{cy:.2}" fill="none" stroke="{color}" stroke-width="1" />
    <line x1="{wl:.2}" y1="{cy:.2}" x2="{er:.2}" y2="{cy:.2}" stroke="{color}" stroke-width="1" />
    <text x="{cx:.2}" y="{ny:.2}" font-size="{fs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif" font-weight="bold">N</text>
    <text x="{cx:.2}" y="{sy:.2}" font-size="{sfs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">S</text>
    <text x="{ex:.2}" y="{cy:.2}" font-size="{sfs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">E</text>
    <text x="{wx:.2}" y="{cy:.2}" font-size="{sfs:.2}" fill="{color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif">W</text>
    <text x="{sx:.2}" y="{sty:.2}" font-size="{stfs:.2}" fill="{street_color}" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif"{street_transform}>STREET</text>
  </g>"#,
        cx = center_x,
        cy = center_y,
        r = size / 2.0,
        color = opts.compass_color,
        nt = center_y - arrow,
        st = center_y + arrow,
        nl = center_x - arrow_width / 2.0,
        nr = center_x + arrow_width / 2.0,
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
        street_transform = street_transform,
    )
}

fn street_label_position(dir: CardinalDirection, x: f64, y: f64, arrow: f64) -> (f64, f64, f64) {
    match dir {
        CardinalDirection::North => (x, y - arrow - 5.0, 0.0),
        CardinalDirection::South => (x, y + arrow + 5.0, 0.0),
        CardinalDirection::East => (x + arrow + 5.0, y, 90.0),
        CardinalDirection::West => (x - arrow - 5.0, y, -90.0),
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
