units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.5
  floor hardwood
}

plan "Real World 10 Galley Kitchen Island" {
  footprint rect (0,0) (10,8)
  legend {
    floor_materials auto
  }

  room dining {
    rect (0,0) (4,8)
    floor hardwood
    label "Dining"
  }

  room pantry {
    rect (4,0) (6,2)
    floor vinyl
    label "Pantry"
  }

  room breakfast_nook {
    rect (6,0) (10,2)
    floor hardwood
    label "Breakfast Nook"
  }

  room kitchen {
    rect (4,2) (10,8)
    floor tile
    label "Galley Kitchen"
  }

  object wall_sink {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 1.3
    facing south
    label "Wall Sink"
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in kitchen
    attach north wall
    at 2.2
    facing south
    label "Dishwasher"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach east wall
    at 1.6
    facing west
    label "Fridge"
  }

  object island_counter {
    use builtin.kitchen.counter.size_48in
    in kitchen
    at (7.0,4.1)
    facing north
    label "Island"
  }

  object island_cooktop {
    use builtin.kitchen.cooktop.size_30in
    in kitchen
    at (7.0,4.1)
    facing north
    label "Cooktop"
  }

  object coffee_counter {
    use builtin.kitchen.counter.size_36in
    in kitchen
    attach south wall
    at 1.4
    facing north
    label "Coffee Bar"
  }

  opening door d_dining_kitchen {
    between dining and kitchen
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_pantry_kitchen {
    between pantry and kitchen
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_nook_kitchen {
    between breakfast_nook and kitchen
    on shared_edge
    at 50%
    swing lh
  }

  opening window w_kitchen_north {
    on kitchen.edge north
    at 25%
    width 2.0
  }

  opening window w_nook_south {
    on breakfast_nook.edge south
    at 50%
    width 1.8
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
