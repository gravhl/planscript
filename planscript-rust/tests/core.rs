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
