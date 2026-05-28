use planscript::catalog::{
    candidate_ifc_item_from_file, lint_catalog_item, Catalog, CatalogItem, CatalogLintSeverity,
};
use planscript::compiler::{compile, CompileOptions};
use planscript::exporters::JsonExportOptions;
use planscript::parse;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn external_ifc_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/external-ifc")
        .join(name)
}

fn import_buildingsmart_item(id: &str, category: &str, filename: &str) -> CatalogItem {
    candidate_ifc_item_from_file(
        id.to_string(),
        category.to_string(),
        external_ifc_fixture(filename),
        Some("https://github.com/buildingSMART/Sample-Test-Files".to_string()),
    )
    .expect("import buildingSMART fixture")
}

fn write_catalog_item(dir: &Path, item: &CatalogItem) {
    let file = format!("{}.psobj.json", item.id.replace('.', "_"));
    let json = serde_json::to_string_pretty(item).expect("serialize catalog item");
    fs::write(dir.join(file), json).expect("write catalog item");
}

fn assert_close(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() < epsilon,
        "expected {actual} to be within {epsilon} of {expected}"
    );
}

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
fn imports_public_buildingsmart_ifc_fixtures() {
    let basin = import_buildingsmart_item(
        "external.buildingsmart.basin",
        "sanitary",
        "buildingsmart-basin-tessellation.ifc",
    );
    assert_eq!(
        basin.bim.as_ref().unwrap().ifc_class.as_deref(),
        Some("IfcSanitaryTerminal")
    );
    assert_eq!(
        basin.bim.as_ref().unwrap().ifc_predefined_type.as_deref(),
        Some("WASHHANDBASIN")
    );
    assert_close(basin.size.width, 0.602, 0.01);
    assert_close(basin.size.depth, 0.422, 0.01);
    assert_close(basin.size.height.unwrap(), 0.094, 0.001);

    let window = import_buildingsmart_item(
        "external.buildingsmart.window",
        "opening",
        "buildingsmart-wall-window.ifc",
    );
    assert_eq!(window.name, "Window for Test Example");
    assert_eq!(
        window.bim.as_ref().unwrap().ifc_class.as_deref(),
        Some("IfcWindow")
    );
    assert_close(window.size.width, 3.0, 0.001);
    assert_close(window.size.depth, 0.3, 0.001);
    assert_close(window.size.height.unwrap(), 0.5, 0.001);

    let column = import_buildingsmart_item(
        "external.buildingsmart.column",
        "structure",
        "buildingsmart-column.ifc",
    );
    assert_eq!(column.name, "Column #1");
    assert_eq!(
        column.bim.as_ref().unwrap().ifc_class.as_deref(),
        Some("IfcColumn")
    );
    assert_eq!(
        column.bim.as_ref().unwrap().ifc_predefined_type.as_deref(),
        Some("COLUMN")
    );
    assert_close(column.size.width, 0.2032, 0.0001);
    assert_close(column.size.depth, 0.2032, 0.0001);
    assert_close(column.size.height.unwrap(), 3.048, 0.0001);
    assert!(column.needs_review.contains(&"unit-scale".to_string()));
}

#[test]
fn compiles_and_draws_public_ifc_catalog_items() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("planscript-external-ifc-{nonce}"));
    fs::create_dir_all(&dir).expect("create temp external catalog");

    let items = [
        import_buildingsmart_item(
            "external.buildingsmart.basin",
            "sanitary",
            "buildingsmart-basin-tessellation.ifc",
        ),
        import_buildingsmart_item(
            "external.buildingsmart.window",
            "opening",
            "buildingsmart-wall-window.ifc",
        ),
        import_buildingsmart_item(
            "external.buildingsmart.column",
            "structure",
            "buildingsmart-column.ifc",
        ),
    ];
    for item in &items {
        write_catalog_item(&dir, item);
    }

    let source = format!(
        r#"
        catalog "{}"
        plan "Public IFC Catalog" {{
          footprint rect (0,0) (8,4)
          room lab {{
            rect (0,0) (8,4)
            label "Lab"
          }}

          object basin1 {{
            use external.buildingsmart.basin
            in lab
            at (1.0, 0.5)
            facing north
            label "Basin"
          }}

          object window1 {{
            use external.buildingsmart.window
            in lab
            at (3.2, 0.5)
            facing north
            label "Window"
          }}

          object column1 {{
            use external.buildingsmart.column
            in lab
            at (6.0, 0.5)
            facing north
            label "Column"
          }}

          assert objects_inside_rooms
          assert object_no_overlap
        }}
    "#,
        dir.display()
    );

    let result = compile(
        &source,
        CompileOptions {
            emit_svg: Some(true),
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
    assert!(geometry
        .objects
        .iter()
        .all(|object| !object.polygon.points.is_empty()));

    let svg = result.svg.expect("svg");
    assert!(svg.contains("<!-- Objects / Fixtures -->"));
    assert!(svg.contains(r#"data-object="basin1" data-catalog-id="external.buildingsmart.basin""#));
    assert!(
        svg.contains(r#"data-object="window1" data-catalog-id="external.buildingsmart.window""#)
    );
    assert!(
        svg.contains(r#"data-object="column1" data-catalog-id="external.buildingsmart.column""#)
    );

    let json = result.json.expect("json");
    assert!(json.contains("\"objects\""));
    assert!(json.contains("external.buildingsmart.basin"));
    assert!(json.contains("external.buildingsmart.window"));
    assert!(json.contains("external.buildingsmart.column"));

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
