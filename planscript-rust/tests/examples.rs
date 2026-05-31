use planscript::ast::DoorSlideDirection;
use planscript::compiler::{compile, CompileOptions};
use planscript::geometry::OpeningPlacementType;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn example_files() -> Vec<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_dir = manifest_dir.parent().expect("repo root");
    let dirs = [repo_dir.join("examples"), manifest_dir.join("examples")];
    let mut files = Vec::new();

    for dir in dirs {
        let entries = fs::read_dir(&dir).unwrap_or_else(|error| {
            panic!(
                "failed to read example directory {}: {error}",
                dir.display()
            )
        });
        for entry in entries {
            let path = entry.expect("example dir entry").path();
            if path.extension().is_some_and(|extension| extension == "psc") {
                files.push(path);
            }
        }
    }

    files.sort();
    files
}

fn compile_example(path: &Path) -> planscript::CompileResult {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    compile(
        &source,
        CompileOptions {
            emit_svg: Some(false),
            emit_json: Some(false),
            catalog_base_dir: path
                .parent()
                .map(|parent| parent.to_string_lossy().to_string()),
            ..Default::default()
        },
    )
}

#[test]
fn examples_have_color_and_draft_svg_artifacts() {
    let mut missing = Vec::new();

    for path in example_files() {
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("example file stem");
        let dir = path.parent().expect("example parent dir");
        for file_name in [
            format!("{stem}.svg"),
            format!("{stem}_draft.svg"),
            format!("{stem}_warnings.txt"),
            format!("{stem}_draft_warnings.txt"),
        ] {
            let artifact = dir.join(file_name);
            if !artifact.exists() {
                missing.push(artifact.display().to_string());
            }
        }
    }

    assert!(
        missing.is_empty(),
        "missing generated example artifact(s):\n{}",
        missing.join("\n")
    );
}

#[test]
fn examples_compile_with_handed_doors_and_room_connections() {
    let mut failures = Vec::new();
    let mut handed_doors = 0;
    let mut double_doors = 0;
    let mut pocket_doors = 0;

    for path in example_files() {
        let result = compile_example(&path);
        if !result.success {
            let messages = result
                .errors
                .iter()
                .map(|error| format!("{:?}: {}", error.phase, error.message))
                .collect::<Vec<_>>()
                .join("; ");
            failures.push(format!("{} failed to compile: {messages}", path.display()));
            continue;
        }

        let geometry = result.geometry.expect("geometry");
        let wall_rooms: HashMap<_, _> = geometry
            .walls
            .iter()
            .map(|wall| (wall.id.as_str(), wall.rooms.as_slice()))
            .collect();
        let mut interroom_door_counts: HashMap<_, usize> = geometry
            .rooms
            .iter()
            .map(|room| (room.name.as_str(), 0))
            .collect();

        for opening in &geometry.openings {
            if opening.opening_type != OpeningPlacementType::Door {
                continue;
            }

            if opening.pocket {
                pocket_doors += 1;
            } else if opening.swing.is_some() {
                handed_doors += 1;
            } else {
                failures.push(format!(
                    "{} door {} is missing a handed swing",
                    path.display(),
                    opening.id
                ));
            }
            if opening.double {
                double_doors += 1;
            }

            let rooms = wall_rooms
                .get(opening.wall_id.as_str())
                .copied()
                .unwrap_or_default();
            if rooms.len() == 2 {
                for room in rooms {
                    *interroom_door_counts.entry(room.as_str()).or_default() += 1;
                }
            }
        }

        let missing = interroom_door_counts
            .iter()
            .filter_map(|(room, count)| (*count == 0).then_some(*room))
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            failures.push(format!(
                "{} has rooms without inter-room doors: {}",
                path.display(),
                missing.join(", ")
            ));
        }
    }

    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(
        handed_doors > 0,
        "examples should exercise handed door swings"
    );
    assert!(double_doors > 0, "examples should exercise double doors");
    assert!(pocket_doors > 0, "examples should exercise pocket doors");
}

#[test]
fn pocket_door_examples_cover_syntax_variants() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let gallery_path = manifest_dir
        .join("examples")
        .join("pocket-door-syntax-gallery.psc");
    let source = fs::read_to_string(&gallery_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", gallery_path.display()));

    assert!(source.contains("opening pocket door"));
    assert!(source.contains("opening pocket_door"));
    assert!(source.contains("\n    pocket\n"));

    let result = compile_example(&gallery_path);
    assert!(result.success, "{:?}", result.errors);
    let geometry = result.geometry.expect("geometry");
    let pockets = geometry
        .openings
        .iter()
        .filter(|opening| opening.opening_type == OpeningPlacementType::Door && opening.pocket)
        .collect::<Vec<_>>();

    assert_eq!(
        pockets.len(),
        5,
        "syntax gallery should render every pocket-door form"
    );
    assert!(
        pockets
            .iter()
            .any(|opening| opening.slide == Some(DoorSlideDirection::Left)),
        "syntax gallery should include slide left"
    );
    assert!(
        pockets
            .iter()
            .any(|opening| opening.slide == Some(DoorSlideDirection::Right)),
        "syntax gallery should include slide right"
    );
    assert!(
        pockets.iter().any(|opening| opening.slide.is_none()),
        "syntax gallery should include default slide direction"
    );
}
