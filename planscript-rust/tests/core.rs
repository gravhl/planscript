use planscript::ast::{
    DimensionFixtureSelection, DimensionRoomSelection, DimensionUnitSystem, DimensionWallSelection,
    DoorSlideDirection, DoorSwing, Opening, RenderMode,
};
use planscript::compiler::{compile, CompileOptions};
use planscript::exporters::{JsonExportOptions, SvgExportOptions};
use planscript::geometry::{calculate_polygon_area, generate_geometry};
use planscript::lowering::lower;
use planscript::parser::parse;

#[test]
fn parses_and_compiles_basic_program() {
    let source = r#"
        units m
        plan "Test" {
          footprint rect (0,0) (10,10)
          room living {
            rect (1,1) (9,9)
            label "Living Room"
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    assert_eq!(ast.plan.name, "Test");
    assert_eq!(ast.plan.rooms.len(), 1);

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert!(result.svg.unwrap().contains("<svg"));
}

#[test]
fn lowers_attached_room_with_auto_dimension() {
    let source = r#"
        plan {
          footprint rect (0,0) (20,20)
          room living { rect (1,1) (9,7) }
          room hall {
            rect size (1.5, auto)
            attach east_of living
            align top
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    let lowered = lower(&ast).expect("lower");
    let hall = lowered.rooms.iter().find(|r| r.name == "hall").unwrap();
    let min_y = hall
        .polygon
        .iter()
        .map(|p| p.y)
        .fold(f64::INFINITY, f64::min);
    let max_y = hall
        .polygon
        .iter()
        .map(|p| p.y)
        .fold(f64::NEG_INFINITY, f64::max);
    assert_eq!(max_y - min_y, 6.0);
}

#[test]
fn generates_partial_shared_wall_opening() {
    let source = r#"
        plan {
          footprint rect (0,0) (20,20)
          room hall { rect (1,7) (13,9) }
          room bedroom { rect (1,9) (5,13) }
          opening door d1 {
            between hall and bedroom
            on shared_edge
            at 50%
            width 0.9
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    let lowered = lower(&ast).expect("lower");
    let geometry = generate_geometry(&lowered);
    assert_eq!(geometry.openings.len(), 1);
    let door = &geometry.openings[0];
    let wall = geometry
        .walls
        .iter()
        .find(|w| w.id == door.wall_id)
        .unwrap();
    assert!((wall.end.x - wall.start.x).abs() == 4.0 || (wall.end.y - wall.start.y).abs() == 4.0);
    assert_eq!(door.position, 2.0);
}

#[test]
fn validates_inside_footprint_assertion() {
    let source = r#"
        plan {
          footprint rect (0,0) (10,10)
          room living { rect (15,15) (20,20) }
          assert inside footprint all_rooms
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(!result.success);
    assert_eq!(result.errors[0].code.as_deref(), Some("E130"));
}

#[test]
fn exports_pretty_json() {
    let source = r#"
        plan {
          footprint rect (0,0) (10,10)
          room r { rect (1,1) (9,9) }
        }
    "#;

    let result = compile(
        source,
        CompileOptions {
            emit_json: Some(true),
            json_options: Some(JsonExportOptions {
                pretty: Some(true),
                include_ast: Some(true),
            }),
            ..Default::default()
        },
    );
    assert!(result.success, "{:?}", result.errors);
    let json = result.json.unwrap();
    assert!(json.contains('\n'));
    assert!(json.contains("\"ast\""));
}

#[test]
fn compiles_floor_materials_outdoor_areas_and_legend() {
    let source = r#"
        defaults {
          floor hardwood
          outdoor_floor pavers
        }

        plan "Floors" {
          footprint rect (0,0) (8,6)
          legend {
            floor_materials auto
          }

          room living {
            rect (0,0) (8,6)
            floor tile
            label "Living"
          }

          outdoor deck rear_deck {
            rect (0,-3) (8,0)
            floor wood_deck
            label "Rear Deck"
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    assert_eq!(ast.plan.outdoor_areas.len(), 1);
    assert!(ast.plan.legend.is_some());

    let result = compile(
        source,
        CompileOptions {
            emit_json: Some(true),
            json_options: Some(JsonExportOptions {
                pretty: Some(true),
                include_ast: Some(false),
            }),
            ..Default::default()
        },
    );
    assert!(result.success, "{:?}", result.errors);
    let geometry = result.geometry.as_ref().expect("geometry");
    assert_eq!(geometry.rooms[0].floor_material.as_deref(), Some("tile"));
    assert_eq!(geometry.outdoor_areas.len(), 1);
    assert_eq!(
        geometry.outdoor_areas[0].floor_material.as_deref(),
        Some("wood_deck")
    );

    let svg = result.svg.expect("svg");
    assert!(svg.contains(r#"class="floor-material-legend""#));
    assert!(svg.contains("floor-material-tile"));
    assert!(svg.contains("floor-material-wood-deck"));
    assert!(svg.contains(r#"data-outdoor="rear_deck""#));

    let json = result.json.expect("json");
    assert!(json.contains("\"outdoorAreas\""));
    assert!(json.contains("\"floorMaterial\": \"wood_deck\""));
}

#[test]
fn renders_draft_mode_as_black_and_white_hatches() {
    let source = r#"
        render {
          mode draft
        }

        defaults {
          floor hardwood
          outdoor_floor wood_deck
        }

        plan "Draft Floors" {
          footprint rect (0,0) (8,6)
          legend { floor_materials auto }

          room living {
            rect (0,0) (8,6)
            floor tile
          }

          outdoor deck rear_deck {
            rect (0,6) (8,9)
            floor wood_deck
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    assert_eq!(
        ast.render.as_ref().map(|render| render.mode),
        Some(RenderMode::Draft)
    );

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.expect("svg");
    assert!(svg.contains(r#"class="floor-material-legend""#));
    assert!(svg.contains(r##"fill="#ffffff""##));
    assert!(svg.contains(r##"stroke="#111111""##));
    assert!(svg.contains("floor-material-wood-deck"));
    assert!(!svg.contains("#d8b47a"));
    assert!(!svg.contains("#8f6632"));
    assert!(!svg.contains("#e74c3c"));
    assert!(!svg.contains("#3498db"));
}

#[test]
fn svg_export_options_can_force_draft_mode() {
    let source = r#"
        defaults {
          floor hardwood
        }

        plan "Draft Option" {
          footprint rect (0,0) (8,6)
          legend { floor_materials auto }
          room living { rect (0,0) (8,6) }
        }
    "#;

    let result = compile(
        source,
        CompileOptions {
            svg_options: Some(SvgExportOptions {
                render_mode: Some(RenderMode::Draft),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.expect("svg");
    assert!(svg.contains(r##"stroke="#111111""##));
    assert!(!svg.contains("#f2dfbf"));
    assert!(!svg.contains("#b98d56"));
}

#[test]
fn renders_wall_and_fixture_dimensions_from_plan_settings() {
    let source = r#"
        plan "Dimension Controls" {
          footprint rect (0,0) (12,8)
          dimensions {
            walls all
            fixtures all
          }

          room bath {
            rect (0,0) (6,8)
          }

          room hall {
            rect (6,0) (12,8)
          }

          object wc {
            use builtin.sanitary.toilet.floor_mounted
            in bath
            at (1,1)
          }

          opening door d_bath_hall {
            between bath and hall
            on shared_edge
            at 50%
            swing lh
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    let dimensions = ast.plan.dimensions.as_ref().expect("dimensions");
    assert!(matches!(dimensions.walls, DimensionWallSelection::All));
    assert!(matches!(
        dimensions.fixtures,
        DimensionFixtureSelection::All
    ));

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.expect("svg");
    assert!(svg.contains(r#"class="dimension dimension-wall""#));
    assert!(svg.contains(r#"class="dimension dimension-fixture""#));
    assert!(svg.contains(r#"data-fixture-dimension="wc""#));
}

#[test]
fn renders_targeted_wall_and_fixture_dimensions() {
    let source = r#"
        plan "Targeted Dimensions" {
          footprint rect (0,0) (12,8)
          dimensions {
            rooms none
            footprint off
            walls bath.east
            fixtures sink
          }

          room bath {
            rect (0,0) (6,8)
          }

          room hall {
            rect (6,0) (12,8)
          }

          object wc {
            use builtin.sanitary.toilet.floor_mounted
            in bath
            at (1,1)
          }

          object sink {
            use builtin.sanitary.sink.wall_hung
            in bath
            at (3,1)
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    let dimensions = ast.plan.dimensions.as_ref().expect("dimensions");
    assert_eq!(dimensions.rooms, DimensionRoomSelection::None);
    assert!(matches!(
        dimensions.walls,
        DimensionWallSelection::Only { .. }
    ));

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.expect("svg");
    assert_eq!(
        svg.matches(r#"class="dimension dimension-wall""#).count(),
        1
    );
    assert_eq!(
        svg.matches(r#"class="dimension dimension-fixture""#)
            .count(),
        2
    );
    assert!(svg.contains(r#"data-fixture-dimension="sink""#));
    assert!(!svg.contains(r#"data-fixture-dimension="wc""#));
}

#[test]
fn renders_standard_dimension_units_from_global_setting() {
    let source = r#"
        dimension_units standard

        plan "Standard Dimension Units" {
          footprint rect (0,0) (5,4)
          dimensions {
            fixtures range
          }

          room kitchen {
            rect (0,0) (5,4)
          }

          object range {
            use builtin.kitchen.range.size_36in
            in kitchen
            attach south wall
            at 50%
            facing north
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    assert_eq!(
        ast.dimension_units.as_ref().map(|units| units.units),
        Some(DimensionUnitSystem::Standard)
    );

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let geometry = result.geometry.as_ref().expect("geometry");
    assert_eq!(
        geometry.dimensions.unit_system,
        Some(DimensionUnitSystem::Standard)
    );
    let svg = result.svg.expect("svg");
    assert!(svg.contains(">3ft<"), "{svg}");
    assert!(svg.contains(">2ft 5in<"), "{svg}");
    assert!(!svg.contains(">91cm<"));
}

#[test]
fn fixture_labels_avoid_dimension_text() {
    let source = r#"
        units ft

        plan "Dimension Label Collision" {
          footprint rect (0,0) (12,8)

          dimensions {
            fixtures range
          }

          room kitchen {
            rect (0,0) (12,8)
            label "Kitchen"
          }

          object range {
            use builtin.kitchen.range.size_36in
            in kitchen
            attach south wall
            at 50%
            facing north
          }
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.expect("svg");
    assert!(svg.contains(r#"data-object-label="range""#));
    assert!(svg.contains(r#"data-fixture-dimension="range""#));
    assert_svg_text_does_not_overlap(&svg);
}

#[test]
fn omits_floor_legend_when_no_floor_materials_are_declared() {
    let source = r#"
        plan "No Floor Legend" {
          footprint rect (0,0) (8,6)
          room living { rect (0,0) (8,6) }
          outdoor patio rear_patio {
            rect (0,-3) (8,0)
          }
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.expect("svg");
    assert!(!svg.contains(r#"class="floor-material-legend""#));
    assert!(!svg.contains("floor-material-pavers"));
}

#[test]
fn warns_on_unknown_or_mismatched_floor_materials() {
    let source = r#"
        plan "Floor Warnings" {
          footprint rect (0,0) (8,6)
          room living {
            rect (0,0) (8,6)
            floor grass
          }
          outdoor patio rear_patio {
            rect (0,-3) (8,0)
            floor carpet
          }
          outdoor deck side_deck {
            rect (8,0) (10,4)
            floor moon_dust
          }
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("outdoor floor material \"grass\"")),
        "{:?}",
        result.warnings
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("indoor floor material \"carpet\"")),
        "{:?}",
        result.warnings
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("unknown floor material \"moon_dust\"")),
        "{:?}",
        result.warnings
    );
}

#[test]
fn area_matches_shoelace_geometry() {
    let points = [
        planscript::ast::Point { x: 0.0, y: 0.0 },
        planscript::ast::Point { x: 4.0, y: 0.0 },
        planscript::ast::Point { x: 4.0, y: 3.0 },
        planscript::ast::Point { x: 0.0, y: 3.0 },
    ];
    assert_eq!(calculate_polygon_area(&points), 12.0);
}

#[test]
fn renders_compass_reference_geometry() {
    let source = r#"
        site {
          street east
          hemisphere north
        }

        plan {
          footprint rect (0,0) (10,10)
          room r { rect (1,1) (9,9) }
        }
    "#;

    let result = compile(
        source,
        CompileOptions {
            svg_options: Some(SvgExportOptions {
                show_compass: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.unwrap();
    assert!(svg.contains(
        r##"<polygon points="915.00,105.00 911.25,85.00 918.75,85.00" fill="none" stroke="#2c3e50" stroke-width="1" />"##
    ));
    assert!(svg.contains(
        r##"<text x="940.00" y="85.00" font-size="6.00" fill="#e74c3c" text-anchor="middle" dominant-baseline="middle" font-family="Arial, sans-serif" transform="rotate(90, 940.00, 85.00)">STREET</text>"##
    ));
}

#[test]
fn renders_handed_and_double_door_swings() {
    let source = r#"
        units m
        defaults {
          door_width 0.9
        }

        plan {
          footprint rect (0,0) (12,8)
          room foyer { rect (0,0) (4,8) }
          room living { rect (4,0) (12,8) }

          opening door d_entry {
            on foyer.edge south
            at 50%
            swing rhr
          }

          opening double door d_living {
            between foyer and living
            on shared_edge
            at 50%
            swing lh
          }

          opening door d_custom {
            on living.edge north
            at 50%
            width 1.2
            swing rh
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    let doors = &ast.plan.openings;
    assert_eq!(doors.len(), 3);

    let lowered = lower(&ast).expect("lower");
    let geometry = generate_geometry(&lowered);
    let entry = geometry
        .openings
        .iter()
        .find(|opening| opening.id == "d_entry")
        .unwrap();
    assert_eq!(entry.width, 0.9);
    assert_eq!(entry.swing, Some(DoorSwing::RightHandReverse));
    assert!(!entry.double);

    let double = geometry
        .openings
        .iter()
        .find(|opening| opening.id == "d_living")
        .unwrap();
    assert_eq!(double.width, 1.8);
    assert_eq!(double.swing, Some(DoorSwing::LeftHand));
    assert!(double.double);

    let custom = geometry
        .openings
        .iter()
        .find(|opening| opening.id == "d_custom")
        .unwrap();
    assert_eq!(custom.width, 1.2);

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.unwrap();
    assert_eq!(svg.matches(r#"class="door-swing""#).count(), 4);
    assert_eq!(svg.matches(r#"class="door-leaf""#).count(), 4);
}

#[test]
fn renders_pocket_door_without_swing_arc() {
    let source = r#"
        units m

        plan {
          footprint rect (0,0) (5,3)
          room hall { rect (0,0) (1,3) }
          room bath { rect (1,0) (5,3) }

          opening pocket door d_bath {
            between hall and bath
            on shared_edge
            at 50%
            width 1.1
            slide right
          }
        }
    "#;

    let ast = parse(source).expect("parse");
    let Opening::DoorOpening(door) = &ast.plan.openings[0] else {
        panic!("expected door");
    };
    assert!(door.pocket);
    assert_eq!(door.slide, Some(DoorSlideDirection::Right));
    assert!(door.swing.is_none());

    let lowered = lower(&ast).expect("lower");
    let geometry = generate_geometry(&lowered);
    let pocket = geometry
        .openings
        .iter()
        .find(|opening| opening.id == "d_bath")
        .unwrap();
    assert!(pocket.pocket);
    assert_eq!(pocket.slide, Some(DoorSlideDirection::Right));

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert!(
        result.warnings.is_empty(),
        "pocket doors should not get swing warnings: {:?}",
        result.warnings
    );
    let svg = result.svg.unwrap();
    assert!(svg.contains(r#"class="pocket-door""#));
    assert!(svg.contains(r#"class="door-leaf pocket-door-leaf""#));
    assert!(svg.contains(r#"class="pocket-door-slide""#));
    assert!(!svg.contains(r#"class="door-swing""#));
}

#[test]
fn rejects_door_width_that_exceeds_wall() {
    let source = r#"
        plan {
          footprint rect (0,0) (4,4)
          room r { rect (0,0) (4,4) }
          opening door d_too_wide {
            on r.edge south
            at 50%
            width 5.0
            swing lh
          }
          assert openings_on_walls
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(!result.success);
    assert!(result
        .errors
        .iter()
        .any(|error| error.code.as_deref() == Some("E311")));
}

#[test]
fn warns_when_door_swing_intersects_wall_but_still_renders() {
    let source = r#"
        units m

        plan {
          footprint rect (0,0) (4,5)
          room living { rect (0,0) (4,4) }
          room hall { rect (0,4) (4,5) }

          opening door d_living_hall {
            between living and hall
            on shared_edge
            at 50%
            width 1.5
            swing lh
          }
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert!(result.svg.is_some(), "warning should not block SVG output");
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("d_living_hall")
                && warning.contains("swing intersects wall")),
        "{:?}",
        result.warnings
    );
}

#[derive(Debug)]
struct SvgTextBox {
    label: String,
    rect: TestRect,
}

#[derive(Debug, Clone, Copy)]
struct TestRect {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

fn assert_svg_text_does_not_overlap(svg: &str) {
    let boxes = svg_text_boxes(svg);
    for (i, a) in boxes.iter().enumerate() {
        for b in boxes.iter().skip(i + 1) {
            let overlap = rect_overlap(a.rect, b.rect);
            assert!(
                overlap <= 0.25,
                "SVG text overlaps by {overlap:.2}: '{}' {:?} and '{}' {:?}",
                a.label,
                a.rect,
                b.label,
                b.rect
            );
        }
    }
}

fn svg_text_boxes(svg: &str) -> Vec<SvgTextBox> {
    svg.lines()
        .filter(|line| line.contains("<text"))
        .filter_map(|line| {
            let x = extract_svg_attr(line, "x")?.parse::<f64>().ok()?;
            let y = extract_svg_attr(line, "y")?.parse::<f64>().ok()?;
            let font_size = extract_svg_attr(line, "font-size")?.parse::<f64>().ok()?;
            let label = svg_text_content(line);
            let rotation = extract_svg_attr(line, "transform")
                .and_then(parse_svg_rotation)
                .unwrap_or(0.0);
            Some(SvgTextBox {
                rect: rotated_text_rect(&label, font_size, x, y, rotation),
                label,
            })
        })
        .collect()
}

fn extract_svg_attr<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!(r#"{name}=""#);
    let start = line.find(&needle)? + needle.len();
    let end = line[start..].find('"')? + start;
    Some(&line[start..end])
}

fn svg_text_content(line: &str) -> String {
    let Some(start) = line.find('>').map(|index| index + 1) else {
        return String::new();
    };
    let end = line[start..]
        .find("</text>")
        .map(|index| start + index)
        .unwrap_or(line.len());
    line[start..end].to_string()
}

fn parse_svg_rotation(transform: &str) -> Option<f64> {
    let body = transform.strip_prefix("rotate(")?.strip_suffix(')')?;
    body.split(',').next()?.trim().parse::<f64>().ok()
}

fn rotated_text_rect(label: &str, font_size: f64, x: f64, y: f64, rotation: f64) -> TestRect {
    let width = label.chars().count() as f64 * font_size * 0.56;
    let height = font_size * 1.15;
    let radians = rotation.to_radians();
    let cos = radians.cos();
    let sin = radians.sin();
    let mut rect = TestRect {
        min_x: f64::INFINITY,
        max_x: f64::NEG_INFINITY,
        min_y: f64::INFINITY,
        max_y: f64::NEG_INFINITY,
    };
    for (local_x, local_y) in [
        (-width / 2.0, -height / 2.0),
        (width / 2.0, -height / 2.0),
        (width / 2.0, height / 2.0),
        (-width / 2.0, height / 2.0),
    ] {
        let px = x + local_x * cos - local_y * sin;
        let py = y + local_x * sin + local_y * cos;
        rect.min_x = rect.min_x.min(px);
        rect.max_x = rect.max_x.max(px);
        rect.min_y = rect.min_y.min(py);
        rect.max_y = rect.max_y.max(py);
    }
    rect
}

fn rect_overlap(a: TestRect, b: TestRect) -> f64 {
    let width = (a.max_x.min(b.max_x) - a.min_x.max(b.min_x)).max(0.0);
    let height = (a.max_y.min(b.max_y) - a.min_y.max(b.min_y)).max(0.0);
    width * height
}
