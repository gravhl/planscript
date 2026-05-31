units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.5
  floor hardwood
}

plan "Real World 11 L Shape Island Open Plan" {
  footprint rect (0,0) (6.6,6.4)
  legend {
    floor_materials auto
  }

  room living {
    rect (0,0) (6.6,2.8)
    floor hardwood
    label "Living"
  }

  room kitchen {
    rect (0,2.8) (3.6,6.4)
    floor tile
    label "L Kitchen"
  }

  room dining {
    rect (3.6,2.8) (6.6,6.4)
    floor hardwood
    label "Dining"
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
    at 1.2
    facing south
    label "Sink"
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in kitchen
    attach north wall
    at 2.1
    facing south
    label "DW"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach east wall
    at 1.35
    facing west
    label "Range"
  }

  object island_counter {
    use builtin.kitchen.counter.size_48in
    in kitchen
    at (2.3,4.2)
    facing north
    label "Island"
  }

  object dining_table {
    use builtin.furniture.table.dining_6
    in dining
    at (5.1,4.6)
    facing north
    label "Dining"
  }

  object sofa {
    use builtin.furniture.sofa.three_seat
    in living
    at (2.7,1.2)
    facing north
    label "Sofa"
  }

  opening double door d_living_kitchen {
    between living and kitchen
    on shared_edge
    at 50%
    width 1.6
    swing lh
  }

  opening door d_living_dining {
    between living and dining
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_kitchen_dining {
    between kitchen and dining
    on shared_edge
    at 50%
    swing lh
  }

  opening window w_kitchen {
    on kitchen.edge north
    at 50%
    width 1.8
  }

  opening window w_living {
    on living.edge south
    at 40%
    width 2.2
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
