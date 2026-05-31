use planscript::ast::DoorSwing;
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
