units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.2
  floor tile
}

plan "Real World 13 Two Row Galley Kitchen" {
  footprint rect (0,0) (5.8,4.6)
  legend {
    floor_materials auto
  }

  room dining {
    rect (0,0) (3.8,1.8)
    floor hardwood
    label "Dining"
  }

  room galley {
    rect (0,1.8) (3.8,4.6)
    floor tile
    label "Galley"
  }

  room pantry {
    rect (3.8,1.8) (5.8,4.6)
    floor vinyl
    label "Pantry"
  }

  object sink {
    use builtin.kitchen.sink
    in galley
    attach north wall
    at 0.9
    facing south
    label "Sink"
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in galley
    attach north wall
    at 1.8
    facing south
    label "DW"
  }

  object prep_counter {
    use builtin.kitchen.counter.size_48in
    in galley
    attach north wall
    at 2.8
    facing south
    label "Prep"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in galley
    attach south wall
    at 0.75
    facing north
    label "Fridge"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in galley
    attach south wall
    at 1.8
    facing north
    label "Range"
  }

  object range_counter {
    use builtin.kitchen.counter.size_36in
    in galley
    attach south wall
    at 2.8
    facing north
    label "Counter"
  }

  opening door d_dining_galley {
    between dining and galley
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_galley_pantry {
    between galley and pantry
    on shared_edge
    at 50%
    swing rh
  }

  opening window w_galley {
    on galley.edge north
    at 50%
    width 1.6
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
