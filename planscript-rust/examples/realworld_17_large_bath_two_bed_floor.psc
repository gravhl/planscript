units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.4
  floor hardwood
}

plan "Real World 17 Large Bath Two Bed Floor" {
  footprint rect (0,0) (12,9)
  legend {
    floor_materials auto
  }

  room laundry {
    rect (0,0) (2,2)
    floor vinyl
    label "Laundry"
  }

  room powder {
    rect (0,2) (2,4)
    floor tile
    label "Powder"
  }

  room foyer {
    rect (0,4) (2,9)
    floor hardwood
    label "Foyer"
  }

  room primary_bedroom {
    rect (2,0) (6,3.5)
    floor carpet
    label "Primary"
  }

  room bedroom_two {
    rect (6,0) (9,3.5)
    floor carpet
    label "Bedroom 2"
  }

  room spa_bath {
    rect (9,0) (12,3.5)
    floor tile
    label "Large Bath"
  }

  room living_dining {
    rect (2,3.5) (8,9)
    floor hardwood
    label "Living Dining"
  }

  room u_kitchen {
    rect (8,3.5) (12,9)
    floor tile
    label "U Kitchen"
  }

  object powder_sink {
    use builtin.sanitary.sink.wall_hung
    in powder
    attach north wall
    at 0.8
    facing south
    label "Sink"
  }

  object powder_wc {
    use builtin.sanitary.toilet.floor_mounted
    in powder
    attach south wall
    at 1.2
    facing north
    label "WC"
  }

  object washer {
    use builtin.laundry.washer
    in laundry
    attach west wall
    at 0.65
    facing east
    label "Washer"
  }

  object dryer {
    use builtin.laundry.dryer
    in laundry
    attach west wall
    at 1.25
    facing east
    label "Dryer"
  }

  object kitchen_sink {
    use builtin.kitchen.sink
    in u_kitchen
    attach north wall
    at 1.1
    facing south
    label "Sink"
  }

  object kitchen_range {
    use builtin.kitchen.range.size_36in
    in u_kitchen
    attach east wall
    at 2.2
    facing west
    label "Range"
  }

  object kitchen_fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in u_kitchen
    attach south wall
    at 1.0
    facing north
    label "Fridge"
  }

  object bath_double_lav_left {
    use builtin.sanitary.sink.wall_hung
    in spa_bath
    attach north wall
    at 0.8
    facing south
    label "Sink"
  }

  object bath_double_lav_right {
    use builtin.sanitary.sink.wall_hung
    in spa_bath
    attach north wall
    at 1.8
    facing south
    label "Sink"
  }

  object bath_tub {
    use builtin.sanitary.tub.size_1700
    in spa_bath
    attach south wall
    at 0.95
    facing north
    label "Tub"
  }

  object bath_shower {
    use builtin.sanitary.shower.size_900x900
    in spa_bath
    attach east wall
    at 2.4
    facing west
    label "Shower"
  }

  object bath_wc {
    use builtin.sanitary.toilet.floor_mounted
    in spa_bath
    attach east wall
    at 1.0
    facing west
    label "Toilet"
  }

  opening door d_laundry_powder {
    between laundry and powder
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_powder_foyer {
    between powder and foyer
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_foyer_living {
    between foyer and living_dining
    on shared_edge
    at 55%
    swing lh
  }

  opening door d_living_primary {
    between living_dining and primary_bedroom
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_primary_bed_two {
    between primary_bedroom and bedroom_two
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bed_two_bath {
    between bedroom_two and spa_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening double door d_living_kitchen {
    between living_dining and u_kitchen
    on shared_edge
    at 50%
    width 1.8
    swing lh
  }

  opening window w_living {
    on living_dining.edge north
    at 50%
    width 2.5
  }

  opening window w_primary {
    on primary_bedroom.edge south
    at 50%
    width 1.8
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
