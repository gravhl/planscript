units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.5
  floor hardwood
}

plan "Real World 03 Dual Suite Corner Kitchen" {
  footprint rect (0,0) (20,14)
  legend {
    floor_materials auto
  }

  room left_suite {
    rect (0,7) (5,14)
    floor carpet
    label "Suite"
  }

  room left_bath {
    rect (0,4) (2.5,7)
    floor tile
    label "Bath"
  }

  room left_closet {
    rect (2.5,4) (5,7)
    floor carpet
    label "WIC"
  }

  room great_room {
    rect (5,4) (15,14)
    floor hardwood
    label "Great Room"
  }

  room kitchen {
    rect (15,4) (20,10)
    floor tile
    label "Corner Kitchen"
  }

  room pantry {
    rect (15,10) (20,14)
    floor vinyl
    label "Walk-In Pantry"
  }

  room guest_bed {
    rect (5,0) (10,4)
    floor carpet
    label "Guest Bed"
  }

  room guest_bath {
    rect (10,0) (12.5,4)
    floor tile
    label "Guest Bath"
  }

  room right_suite {
    rect (12.5,0) (17,4)
    floor carpet
    label "Suite"
  }

  room right_bath {
    rect (17,0) (20,4)
    floor tile
    label "Bath"
  }

  object island_counter {
    use builtin.kitchen.counter.size_48in
    in kitchen
    at (17.5,5.25)
    facing north
    label "Island Bar"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach east wall
    at 2.7
    facing west
    label "Range"
  }

  object sink {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 1.6
    facing south
    label "Sink"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach south wall
    at 3.7
    facing north
    label "Fridge"
  }

  object left_wc {
    use builtin.sanitary.toilet.floor_mounted
    in left_bath
    attach west wall
    at 1.0
    facing east
    label "WC"
  }

  object left_sink {
    use builtin.sanitary.sink.wall_hung
    in left_bath
    attach north wall
    at 1.0
    facing south
    label "Sink"
  }

  object left_shower {
    use builtin.sanitary.shower.size_900x900
    in left_bath
    attach south wall
    at 1.7
    facing north
    label "Shower"
  }

  object right_wc {
    use builtin.sanitary.toilet.floor_mounted
    in right_bath
    attach east wall
    at 1.0
    facing west
    label "WC"
  }

  object right_sink {
    use builtin.sanitary.sink.wall_hung
    in right_bath
    attach north wall
    at 1.0
    facing south
    label "Sink"
  }

  object right_shower {
    use builtin.sanitary.shower.size_900x900
    in right_bath
    attach south wall
    at 1.8
    facing north
    label "Shower"
  }

  opening door d_left_suite_great {
    between left_suite and great_room
    on shared_edge
    at 60%
    swing lh
  }

  opening door d_left_suite_bath {
    between left_suite and left_bath
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_left_suite_closet {
    between left_suite and left_closet
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_great_kitchen {
    between great_room and kitchen
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_kitchen_pantry {
    between kitchen and pantry
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_great_guest {
    between great_room and guest_bed
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_guest_bath {
    between guest_bed and guest_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_guest_right_suite {
    between guest_bath and right_suite
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_right_suite_bath {
    between right_suite and right_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_right_bath_kitchen {
    between right_bath and kitchen
    on shared_edge
    at 50%
    swing rh
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
