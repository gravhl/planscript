units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.5
  floor tile
}

plan "Real World 12 U Shape Breakfast Kitchen" {
  footprint rect (0,0) (6.2,5.4)
  legend {
    floor_materials auto
  }

  room breakfast_nook {
    rect (0,0) (4.2,1.8)
    floor hardwood
    label "Breakfast"
  }

  room kitchen {
    rect (0,1.8) (4.2,5.4)
    floor tile
    label "U Kitchen"
  }

  room pantry {
    rect (4.2,1.8) (6.2,5.4)
    floor vinyl
    label "Pantry"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach west wall
    at 0.8
    facing east
    label "Fridge"
  }

  object sink {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 1.6
    facing south
    label "Sink"
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in kitchen
    attach north wall
    at 2.45
    facing south
    label "DW"
  }

  object range_left_counter {
    use builtin.kitchen.counter.size_24in
    in kitchen
    attach east wall
    at 1.3
    facing west
    label "Counter"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach east wall
    at 2.1
    facing west
    label "Range"
  }

  object range_right_counter {
    use builtin.kitchen.counter.size_36in
    in kitchen
    attach east wall
    at 3.05
    facing west
    label "Counter"
  }

  object table {
    use builtin.furniture.table.dining_6
    in breakfast_nook
    at (2.1,0.9)
    facing north
    label "Nook"
  }

  opening door d_nook_kitchen {
    between breakfast_nook and kitchen
    on shared_edge
    at 50%
    width 1.2
    swing lh
  }

  opening door d_kitchen_pantry {
    between kitchen and pantry
    on shared_edge
    at 50%
    swing rh
  }

  opening window w_sink {
    on kitchen.edge north
    at 45%
    width 1.8
  }

  opening window w_nook {
    on breakfast_nook.edge south
    at 50%
    width 2.0
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
