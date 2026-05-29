use planscript::catalog::{
    candidate_ifc_item_from_file, import_ifc_manifest, lint_catalog_item, Catalog, CatalogItem,
    CatalogLintSeverity,
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

#[derive(Debug, Clone, Copy)]
struct SvgRect {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

fn svg_text_position(svg: &str, marker: &str) -> Option<(f64, f64)> {
    let tag = svg_tag_containing(svg, "text", marker)?;
    Some((
        svg_attr(tag, "x")?.parse().ok()?,
        svg_attr(tag, "y")?.parse().ok()?,
    ))
}

fn svg_path_bounds(svg: &str, marker: &str) -> Option<SvgRect> {
    let tag = svg_tag_containing(svg, "path", marker)?;
    let d = svg_attr(tag, "d")?;
    let coords = d
        .split(|ch: char| !(ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+'))
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse::<f64>().ok())
        .collect::<Vec<_>>();
    if coords.len() < 2 {
        return None;
    }

    let mut rect = SvgRect {
        min_x: f64::INFINITY,
        max_x: f64::NEG_INFINITY,
        min_y: f64::INFINITY,
        max_y: f64::NEG_INFINITY,
    };
    for pair in coords.chunks(2) {
        if let [x, y] = pair {
            rect.min_x = rect.min_x.min(*x);
            rect.max_x = rect.max_x.max(*x);
            rect.min_y = rect.min_y.min(*y);
            rect.max_y = rect.max_y.max(*y);
        }
    }
    Some(rect)
}

fn svg_tag_containing<'a>(svg: &'a str, tag_name: &str, marker: &str) -> Option<&'a str> {
    let marker_index = svg.find(marker)?;
    let tag_open = format!("<{tag_name}");
    let tag_start = svg[..marker_index].rfind(&tag_open)?;
    let tag_end = tag_start + svg[tag_start..].find('>')? + 1;
    Some(&svg[tag_start..tag_end])
}

fn svg_attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!(r#"{name}=""#);
    let start = tag.find(&needle)? + needle.len();
    let end = start + tag[start..].find('"')?;
    Some(&tag[start..end])
}

fn point_in_rect(x: f64, y: f64, rect: SvgRect) -> bool {
    x >= rect.min_x && x <= rect.max_x && y >= rect.min_y && y <= rect.max_y
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
            attach south wall
            at 3.0
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
    let shower = geometry
        .objects
        .iter()
        .find(|object| object.name == "sh1")
        .expect("shower object");
    assert_close(shower.origin.y, 0.0, 1e-9);
    assert!(result.json.expect("json").contains("\"objects\""));
    let svg = result.svg.expect("svg");
    assert!(svg.contains(r#"class="fixture fixture-toilet""#));
    assert!(svg.contains(r#"class="fixture-detail fixture-toilet-bowl""#));
    assert!(svg.contains(r#"class="fixture-detail fixture-sink-basin""#));
    assert!(svg.contains(r#"class="fixture-detail fixture-shower-slope""#));
    assert!(svg.contains(r#"class="fixture-label" data-object-label="wc1""#));
    assert!(svg.contains(r#"class="fixture-label" data-object-label="lav1""#));
    assert!(svg.contains(r#"class="fixture-label" data-object-label="sh1""#));
}

#[test]
fn fixture_labels_do_not_move_fixture_geometry() {
    let source_with_label = r#"
        plan "Fixture Label Geometry" {
          footprint rect (0,0) (5,4)
          room bath { rect (0,0) (5,4) }
          object sh1 {
            use builtin.sanitary.shower.size_900x900
            in bath
            attach south wall
            at 3.0
            facing north
            label "Shower"
          }
        }
    "#;
    let source_without_label = r#"
        plan "Fixture Label Geometry" {
          footprint rect (0,0) (5,4)
          room bath { rect (0,0) (5,4) }
          object sh1 {
            use builtin.sanitary.shower.size_900x900
            in bath
            attach south wall
            at 3.0
            facing north
          }
        }
    "#;

    let with_label = compile(source_with_label, CompileOptions::default());
    let without_label = compile(source_without_label, CompileOptions::default());
    assert!(with_label.success, "{:?}", with_label.errors);
    assert!(without_label.success, "{:?}", without_label.errors);

    let with_object = with_label
        .geometry
        .expect("geometry")
        .objects
        .into_iter()
        .find(|object| object.name == "sh1")
        .expect("labeled shower");
    let without_object = without_label
        .geometry
        .expect("geometry")
        .objects
        .into_iter()
        .find(|object| object.name == "sh1")
        .expect("unlabeled shower");

    assert_eq!(with_object.origin, without_object.origin);
    assert_eq!(with_object.facing, without_object.facing);
    assert_eq!(with_object.rotation, without_object.rotation);
    assert_eq!(with_object.polygon, without_object.polygon);
    assert_eq!(
        with_object.clearance_polygons,
        without_object.clearance_polygons
    );
}

#[test]
fn wall_backed_fixtures_default_to_room_wall() {
    let source = r#"
        plan "Default Fixture Wall" {
          footprint rect (0,0) (5,4)
          room bath { rect (0,0) (5,4) }
          object lav1 {
            use builtin.sanitary.sink.wall_hung
            in bath
          }
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert_eq!(result.warnings, Vec::<String>::new());

    let geometry = result.geometry.expect("geometry");
    let sink = geometry
        .objects
        .iter()
        .find(|object| object.name == "lav1")
        .expect("sink object");
    assert_close(sink.origin.x, 2.5, 1e-9);
    assert_close(sink.origin.y, 0.0, 1e-9);
    let min_y = sink
        .polygon
        .points
        .iter()
        .map(|point| point.y)
        .fold(f64::INFINITY, f64::min);
    assert_close(min_y, 0.0, 1e-9);
}

#[test]
fn warns_when_wall_backed_fixture_is_near_wall_but_not_attached() {
    let source = r#"
        plan "Fixture Near Wall" {
          footprint rect (0,0) (5,4)
          room bath { rect (0,0) (5,4) }
          object sh1 {
            use builtin.sanitary.shower.size_900x900
            in bath
            at (3.0, 0.2)
            facing north
            label "Shower"
          }
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert!(result.svg.is_some(), "warning should not block SVG output");
    assert!(
        result.warnings.iter().any(|warning| warning.contains("sh1")
            && warning.contains("0.20m")
            && warning.contains("south wall")),
        "{:?}",
        result.warnings
    );
}

#[test]
fn room_labels_move_to_empty_room_area_when_objects_cover_center() {
    let source = r#"
        plan "Room Label Avoidance" {
          footprint rect (0,0) (6,4)
          room living {
            rect (0,0) (6,4)
            label "Living"
          }
          object dining1 {
            use builtin.furniture.table.dining_6
            in living
            at (3.0, 1.55)
            facing north
            label "Dining"
          }
        }
    "#;

    let result = compile(source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    let svg = result.svg.expect("svg");
    let (label_x, label_y) =
        svg_text_position(&svg, r#"class="room-label" data-room-label="living""#)
            .expect("living room label");
    let table_rect = svg_path_bounds(&svg, r#"class="fixture-base fixture-dining-table""#)
        .expect("dining table fixture");

    assert!(
        !point_in_rect(label_x, label_y, table_rect),
        "room label ({label_x}, {label_y}) should move away from table {table_rect:?}"
    );
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
fn import_manifest_batches_open_ifc_candidates() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("planscript-ifc-manifest-{nonce}"));
    let source_dir = dir.join("source");
    let out_dir = dir.join("catalog");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::copy(
        external_ifc_fixture("buildingsmart-basin-tessellation.ifc"),
        source_dir.join("basin.ifc"),
    )
    .expect("copy basin ifc");
    fs::copy(
        external_ifc_fixture("buildingsmart-column.ifc"),
        source_dir.join("column.ifc"),
    )
    .expect("copy column ifc");

    let manifest_path = dir.join("buildingsmart-open.json");
    fs::write(
        &manifest_path,
        r#"{
          "items": [
            {
              "id": "open.buildingsmart.basin",
              "category": "sanitary",
              "file": "source/basin.ifc",
              "name": "buildingSMART Basin",
              "provider": "buildingSMART Sample-Test-Files",
              "sourceUrl": "https://github.com/buildingSMART/Sample-Test-Files",
              "license": "CC-BY-4.0",
              "redistributable": true
            },
            {
              "id": "open.buildingsmart.column",
              "category": "structure",
              "file": "source/column.ifc",
              "provider": "buildingSMART Sample-Test-Files",
              "sourceUrl": "https://github.com/buildingSMART/Sample-Test-Files",
              "license": "CC-BY-4.0",
              "redistributable": true
            }
          ]
        }"#,
    )
    .expect("write import manifest");

    let written = import_ifc_manifest(&manifest_path, &out_dir).expect("import manifest");
    assert_eq!(written.len(), 2);
    assert!(out_dir.join("open.buildingsmart.basin.psobj.json").exists());
    assert!(out_dir
        .join("open.buildingsmart.column.psobj.json")
        .exists());

    let basin_json =
        fs::read_to_string(out_dir.join("open.buildingsmart.basin.psobj.json")).expect("read item");
    let basin: CatalogItem = serde_json::from_str(&basin_json).expect("parse item");
    assert_eq!(basin.name, "buildingSMART Basin");
    let source = basin.source.as_ref().expect("source");
    assert_eq!(
        source.provider.as_deref(),
        Some("buildingSMART Sample-Test-Files")
    );
    assert_eq!(source.license.as_deref(), Some("CC-BY-4.0"));
    assert!(source.redistributable);
    assert_eq!(basin.assets.ifc.as_deref(), Some("source/basin.ifc"));
    assert!(!basin.needs_review.contains(&"license".to_string()));

    let mut catalog = Catalog::builtins();
    catalog.load_path(&out_dir).expect("load generated catalog");
    assert!(catalog.get("open.buildingsmart.basin").is_some());
    assert!(catalog.get("open.buildingsmart.column").is_some());

    let source = format!(
        r#"
        catalog "{}"
        plan "Manifest Catalog" {{
          footprint rect (0,0) (5,3)
          room lab {{ rect (0,0) (5,3) }}

          object basin1 {{
            use open.buildingsmart.basin
            in lab
            at (1.0, 0.5)
            facing north
          }}

          object column1 {{
            use open.buildingsmart.column
            in lab
            at (3.5, 0.5)
            facing north
          }}

          assert objects_inside_rooms
          assert object_no_overlap
        }}
    "#,
        out_dir.display()
    );
    let result = compile(&source, CompileOptions::default());
    assert!(result.success, "{:?}", result.errors);
    assert_eq!(result.geometry.unwrap().objects.len(), 2);

    let _ = fs::remove_dir_all(&dir);
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
