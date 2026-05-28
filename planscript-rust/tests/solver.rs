use planscript::compiler::{compile, CompileOptions};
use planscript::solver::{
    solve, EdgeDirection, HardConstraints, IntentFootprint, LayoutIntent, OpeningDefaults,
    RoomSpec, SolveOptions, SolverResult,
};

fn room(id: &str, room_type: &str, min_area: f64) -> RoomSpec {
    RoomSpec {
        id: id.to_string(),
        room_type: room_type.to_string(),
        label: None,
        min_area,
        target_area: None,
        max_area: None,
        min_width: None,
        min_height: None,
        max_width: None,
        max_height: None,
        aspect: None,
        fill_cell: None,
        preferred_bands: None,
        preferred_depths: None,
        must_touch_exterior: None,
        must_touch_edge: None,
        adjacent_to: None,
        avoid_adjacent_to: None,
        needs_access_from: None,
        is_circulation: None,
        has_exterior_door: None,
        is_ensuite: None,
    }
}

fn base_intent(rooms: Vec<RoomSpec>) -> LayoutIntent {
    LayoutIntent {
        units: "m".to_string(),
        footprint: IntentFootprint::Rect {
            min: [0.0, 0.0],
            max: [12.0, 8.0],
        },
        bands: None,
        depths: None,
        front_edge: EdgeDirection::South,
        garden_edge: Some(EdgeDirection::North),
        defaults: OpeningDefaults {
            door_width: 0.9,
            window_width: 1.5,
            exterior_door_width: Some(1.1),
            corridor_width: Some(1.2),
        },
        rooms,
        hard: HardConstraints {
            no_overlap: true,
            inside_footprint: true,
            all_rooms_reachable: Some(false),
        },
        access_rule_preset: None,
        access_rules: None,
        weights: None,
    }
}

#[test]
fn solves_simple_two_room_intent() {
    let mut living = room("living", "living", 25.0);
    living.preferred_bands = Some(vec!["left".to_string()]);
    living.must_touch_exterior = Some(true);

    let mut bedroom = room("bedroom", "bedroom", 20.0);
    bedroom.preferred_bands = Some(vec!["right".to_string()]);
    bedroom.must_touch_exterior = Some(true);

    let mut intent = base_intent(vec![living, bedroom]);
    intent.bands = Some(vec![
        planscript::solver::BandSpec {
            id: "left".to_string(),
            target_width: Some(6.0),
            ..Default::default()
        },
        planscript::solver::BandSpec {
            id: "right".to_string(),
            target_width: Some(6.0),
            ..Default::default()
        },
    ]);

    let result = solve(intent, SolveOptions::default());
    match result {
        SolverResult::Success {
            plan_script, state, ..
        } => {
            assert!(state.placed.contains_key("living"));
            assert!(state.placed.contains_key("bedroom"));
            assert!(plan_script.contains("room living"));
            let compiled = compile(&plan_script, CompileOptions::default());
            assert!(compiled.success, "{:?}", compiled.errors);
        }
        SolverResult::Failure {
            error, violations, ..
        } => {
            panic!("solve failed: {error} {violations:?}");
        }
    }
}

#[test]
fn solver_places_entry_door_and_windows() {
    let mut hall = room("hall", "hall", 8.0);
    hall.has_exterior_door = Some(true);
    hall.is_circulation = Some(true);
    hall.must_touch_edge = Some(EdgeDirection::South);

    let mut living = room("living", "living", 20.0);
    living.adjacent_to = Some(vec!["hall".to_string()]);
    living.must_touch_exterior = Some(true);

    let intent = base_intent(vec![hall, living]);
    let result = solve(intent, SolveOptions::default());
    let SolverResult::Success { state, .. } = result else {
        panic!("solve failed: {result:?}");
    };
    assert!(state
        .openings
        .iter()
        .any(|o| o.is_exterior && o.room_id == "hall"));
    assert!(state.openings.iter().any(|o| matches!(
        o.opening_type,
        planscript::solver::PlacedOpeningType::Window
    )));
}
