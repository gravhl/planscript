units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.4
  floor hardwood
}

plan "Real World 01 Family U Kitchen Floor" {
  footprint rect (0,0) (18,12)
  legend {
    floor_materials auto
  }

  room foyer {
    rect (0,0) (4,4)
    floor hardwood
    label "Foyer"
  }

  room study {
    rect (4,0) (8,4)
    floor carpet
    label "Study"
  }

  room bedroom1 {
    rect (8,0) (12,4)
    floor carpet
    label "Bedroom"
  }

  room shared_bath {
    rect (12,0) (14,4)
    floor tile
    label "Shared Bath"
  }

  room bedroom2 {
    rect (14,0) (18,4)
    floor carpet
    label "Bedroom"
  }

  room family {
    rect (0,4) (10,12)
    floor hardwood
    label "Family Room"
  }

  room kitchen {
    rect (10,4) (18,12)
    floor tile
    label "U Kitchen"
  }

  outdoor porch screened_porch {
    rect (0,12) (10,15)
    floor composite_deck
    label "Screened Porch"
  }

  object kitchen_sink {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 1.4
    facing south
    label "Sink"
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in kitchen
    attach north wall
    at 2.35
    facing south
    label "Dishwasher"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach east wall
    at 2.0
    facing west
    label "Fridge"
  }

  object range_left_counter {
    use builtin.kitchen.counter.size_24in
    in kitchen
    attach south wall
    at 3.238
    facing north
    label "Landing"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach south wall
    at 4.0
    facing north
    label "Range"
  }

  object range_right_counter {
    use builtin.kitchen.counter.size_36in
    in kitchen
    attach south wall
    at 4.9144
    facing north
    label "Landing"
  }

  object bath_wc {
    use builtin.sanitary.toilet.floor_mounted
    in shared_bath
    attach west wall
    at 1.1
    facing east
    label "WC"
  }

  object bath_sink {
    use builtin.sanitary.sink.wall_hung
    in shared_bath
    attach north wall
    at 0.65
    facing south
    label "Sink"
  }

  object bath_shower {
    use builtin.sanitary.shower.size_900x900
    in shared_bath
    attach south wall
    at 1.5
    facing north
    label "Shower"
  }

  opening door d_foyer_family {
    between foyer and family
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_foyer_study {
    between foyer and study
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_family_bedroom1 {
    between family and bedroom1
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_bedroom1_bath {
    between bedroom1 and shared_bath
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bath_bedroom2 {
    between shared_bath and bedroom2
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_family_kitchen {
    between family and kitchen
    on shared_edge
    at 45%
    swing rh
  }

  opening double door d_family_porch {
    on family.edge north
    at 50%
    swing lhr
  }

  opening window w_kitchen_north {
    on kitchen.edge north
    at 45%
    width 2.0
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
