use planscript::catalog::{
    candidate_ifc_item_from_file, lint_catalog_item, Catalog, CatalogItem, CatalogLintSeverity,
};
use planscript::compiler::{compile, CompileOptions};
use planscript::exporters::JsonExportOptions;
use planscript::parse;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn parses_fixture_object_blocks() {
    let source = r#"
        plan "Fixture Parse" {
          footprint rect (0,0) (5,4)
          room bath { rect (0,0) (5,4) }
          object wc1 {
            use builtin.sanitary.toilet.floor_mounted
            in bath
            attach west wall
            at 0.8
            facing east
            clearance front 0.8
          }
        }
    "#;

    let ast = parse(source).expect("parse object");
    assert_eq!(ast.plan.objects.len(), 1);
    assert_eq!(
        ast.plan.objects[0].catalog_id,
        "builtin.sanitary.toilet.floor_mounted"
    );
}

#[test]
fn compiles_builtin_fixture_layout() {
    let source = r#"
        plan "Fixture Bath" {
          footprint rect (0,0) (5,4)
          room bath {
            rect (0,0) (5,4)
            label "Bath"
          }

          object wc1 {
            use builtin.sanitary.toilet.floor_mounted
            in bath
            attach west wall
            at 0.8
            facing east
            label "WC"
          }

          object lav1 {
            use builtin.sanitary.sink.wall_hung
            in bath
            attach north wall
            at 1.0
            facing south
            label "Sink"
          }

          object sh1 {
            use builtin.sanitary.shower.size_900x900
            in bath
            at (3.0, 0.2)
            facing north
            label "Shower"
          }

          assert objects_inside_rooms
          assert object_no_overlap
          assert object_clearances
        }
    "#;

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
    let geometry = result.geometry.expect("geometry");
    assert_eq!(geometry.objects.len(), 3);
    assert!(geometry.objects[0].clearance_polygons.len() >= 1);
    assert!(result.json.expect("json").contains("\"objects\""));
}

#[test]
fn builtins_include_bim_semantics() {
    let catalog = Catalog::builtins();
    let toilet: &CatalogItem = catalog
        .get("builtin.sanitary.toilet.floor_mounted")
        .expect("toilet builtin");

    assert_eq!(
        toilet.bim.as_ref().unwrap().ifc_class.as_deref(),
        Some("IfcSanitaryTerminal")
    );
    assert_eq!(
        toilet.bim.as_ref().unwrap().ifc_predefined_type.as_deref(),
        Some("TOILETPAN")
    );
}

#[test]
fn loads_catalog_items_from_psobj_json() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("planscript-catalog-{nonce}"));
    fs::create_dir_all(&dir).expect("create temp catalog");
    fs::write(
        dir.join("box.psobj.json"),
        r#"{
          "id": "custom.fixture.box",
          "name": "Catalog Box",
          "category": "test",
          "size": { "width": 0.5, "depth": 0.5 },
          "footprint": [
            { "x": -0.25, "y": 0.0 },
            { "x": 0.25, "y": 0.0 },
            { "x": 0.25, "y": 0.5 },
            { "x": -0.25, "y": 0.5 }
          ]
        }"#,
    )
    .expect("write catalog");

    let source = format!(
        r#"
        catalog "{}"
        plan "External Catalog" {{
          footprint rect (0,0) (3,3)
          room r {{ rect (0,0) (3,3) }}
          object box1 {{
            use custom.fixture.box
            in r
            at (1,1)
            facing north
          }}
          assert objects_inside_rooms
        }}
    "#,
        dir.display()
    );

    let result = compile(&source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert_eq!(
        result.geometry.unwrap().objects[0].catalog_id,
        "custom.fixture.box"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn imports_ifc_candidate_metadata_and_bounds() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("planscript-ifc-{nonce}"));
    fs::create_dir_all(&dir).expect("create temp ifc dir");
    let ifc = dir.join("toilet.ifc");
    fs::write(
        &ifc,
        r#"ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
ENDSEC;
DATA;
#1=IFCSIUNIT(*,.LENGTHUNIT.,.MILLI.,.METRE.);
#10=IFCSANITARYTERMINAL('abc',#2,'Compact WC',$,$,$,$,$,.TOILETPAN.);
#20=IFCCARTESIANPOINT((0.,0.,0.));
#21=IFCCARTESIANPOINT((380.,680.,780.));
ENDSEC;
END-ISO-10303-21;"#,
    )
    .expect("write ifc");

    let item = candidate_ifc_item_from_file(
        "vendor.compact_wc".to_string(),
        "sanitary".to_string(),
        &ifc,
        Some("https://example.com/wc".to_string()),
    )
    .expect("import ifc");

    assert_eq!(item.name, "Compact WC");
    assert_eq!(
        item.bim.as_ref().unwrap().ifc_class.as_deref(),
        Some("IfcSanitaryTerminal")
    );
    assert_eq!(
        item.bim.as_ref().unwrap().ifc_predefined_type.as_deref(),
        Some("TOILETPAN")
    );
    assert!((item.size.width - 0.38).abs() < 0.001);
    assert!((item.size.depth - 0.68).abs() < 0.001);
    assert!((item.size.height.unwrap() - 0.78).abs() < 0.001);
    assert!(item.needs_review.contains(&"unit-scale".to_string()));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn lints_candidate_catalog_items_before_approval() {
    let mut item = Catalog::builtins()
        .get("builtin.sanitary.toilet.floor_mounted")
        .expect("toilet builtin")
        .clone();
    item.status = Some("candidate".to_string());
    item.needs_review = vec!["license".to_string()];

    let issues = lint_catalog_item(&item);
    assert!(issues
        .iter()
        .any(|issue| issue.severity == CatalogLintSeverity::Warning));

    item.status = Some("approved".to_string());
    let issues = lint_catalog_item(&item);
    assert!(issues
        .iter()
        .any(|issue| issue.severity == CatalogLintSeverity::Error));
}
