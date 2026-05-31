units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.4
  floor hardwood
}

plan "Real World 18 Two Bed Two Bath Home" {
  footprint rect (0,0) (11,8)
  legend {
    floor_materials auto
  }

  room primary_bedroom {
    rect (0,0) (4,3)
    floor carpet
    label "Primary"
  }

  room ensuite {
    rect (4,0) (5.8,3)
    floor tile
    label "Ensuite"
  }

  room bedroom_two {
    rect (5.8,0) (9,3)
    floor carpet
    label "Bedroom 2"
  }

  room hall_bath {
    rect (9,0) (11,3)
    floor tile
    label "Hall Bath"
  }

  room foyer {
    rect (0,3) (2,8)
    floor hardwood
    label "Foyer"
  }

  room living_dining {
    rect (2,3) (7,8)
    floor hardwood
    label "Living Dining"
  }

  room l_kitchen {
    rect (7,3) (11,8)
    floor tile
    label "L Kitchen"
  }

  object ensuite_sink {
    use builtin.sanitary.sink.wall_hung
    in ensuite
    attach north wall
    at 0.8
    facing south
    label "Sink"
  }

  object ensuite_wc {
    use builtin.sanitary.toilet.floor_mounted
    in ensuite
    attach south wall
    at 0.7
    facing north
    label "Toilet"
  }

  object ensuite_shower {
    use builtin.sanitary.shower.size_900x900
    in ensuite
    attach east wall
    at 1.5
    facing west
    label "Shower"
  }

  object bath_sink {
    use builtin.sanitary.sink.wall_hung
    in hall_bath
    attach north wall
    at 0.8
    facing south
    label "Sink"
  }

  object bath_wc {
    use builtin.sanitary.toilet.floor_mounted
    in hall_bath
    attach east wall
    at 0.9
    facing west
    label "Toilet"
  }

  object bath_tub {
    use builtin.sanitary.tub.size_1700
    in hall_bath
    attach south wall
    at 1.1
    facing north
    label "Tub"
  }

  object kitchen_sink {
    use builtin.kitchen.sink
    in l_kitchen
    attach north wall
    at 1.2
    facing south
    label "Sink"
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in l_kitchen
    attach north wall
    at 2.1
    facing south
    label "DW"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in l_kitchen
    attach east wall
    at 1.8
    facing west
    label "Range"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in l_kitchen
    attach south wall
    at 1.0
    facing north
    label "Fridge"
  }

  object sofa {
    use builtin.furniture.sofa.three_seat
    in living_dining
    at (4.2,5.6)
    facing east
    label "Sofa"
  }

  object dining_table {
    use builtin.furniture.table.dining_6
    in living_dining
    at (6.0,4.2)
    facing north
    label "Dining"
  }

  opening door d_primary_ensuite {
    between primary_bedroom and ensuite
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_foyer_primary {
    between foyer and primary_bedroom
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_living_bed_two {
    between living_dining and bedroom_two
    on shared_edge
    at 45%
    swing lh
  }

  opening door d_kitchen_bath {
    between l_kitchen and hall_bath
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bed_two_bath {
    between bedroom_two and hall_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_foyer_living {
    between foyer and living_dining
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_living_kitchen {
    between living_dining and l_kitchen
    on shared_edge
    at 50%
    width 1.4
    swing rh
  }

  opening window w_primary {
    on primary_bedroom.edge south
    at 50%
    width 1.6
  }

  opening window w_living {
    on living_dining.edge north
    at 50%
    width 2.2
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
