units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.5
  floor hardwood
  outdoor_floor wood_deck
}

plan "Real World 06 Galley Lanai Suite" {
  footprint rect (0,0) (14,16)
  legend {
    floor_materials auto
  }

  room foyer {
    rect (0,0) (4,4)
    floor hardwood
    label "Long Foyer"
  }

  room study_guest {
    rect (4,0) (8,4)
    floor carpet
    label "Study Guest"
  }

  room laundry {
    rect (8,0) (10,4)
    floor vinyl
    label "Laundry"
  }

  room guest_bath {
    rect (10,0) (12,4)
    floor tile
    label "Bath"
  }

  room guest_bed {
    rect (12,0) (14,4)
    floor carpet
    label "Guest Bed"
  }

  room great_room {
    rect (0,4) (8,12)
    floor hardwood
    label "Great Room"
  }

  room dining {
    rect (8,4) (14,8)
    floor hardwood
    label "Dining"
  }

  room galley_kitchen {
    rect (8,8) (14,12)
    floor tile
    label "Galley Kitchen"
  }

  room primary_bed {
    rect (0,12) (6,16)
    floor carpet
    label "Primary"
  }

  room primary_bath {
    rect (6,12) (10,16)
    floor tile
    label "Primary Bath"
  }

  room primary_closet {
    rect (10,12) (14,16)
    floor carpet
    label "WIC"
  }

  outdoor patio lanai {
    rect (14,8) (20,12)
    floor wood_deck
    label "Covered Lanai"
  }

  object island_sink {
    use builtin.kitchen.sink
    in galley_kitchen
    at (11.0,9.0)
    facing north
    label "Island Sink"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in galley_kitchen
    attach north wall
    at 2.2
    facing south
    label "Range"
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in galley_kitchen
    attach south wall
    at 1.0
    facing north
    label "Dishwasher"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in galley_kitchen
    attach east wall
    at 1.0
    facing west
    label "Fridge"
  }

  object primary_sink1 {
    use builtin.sanitary.sink.wall_hung
    in primary_bath
    attach north wall
    at 1.0
    facing south
    label "Vanity"
  }

  object primary_sink2 {
    use builtin.sanitary.sink.wall_hung
    in primary_bath
    attach north wall
    at 2.0
    facing south
    label "Vanity"
  }

  object primary_shower {
    use builtin.sanitary.shower.size_900x900
    in primary_bath
    attach south wall
    at 2.8
    facing north
    label "Walk-In Shower"
  }

  object primary_wc {
    use builtin.sanitary.toilet.floor_mounted
    in primary_bath
    attach east wall
    at 1.1
    facing west
    label "WC"
  }

  object guest_wc {
    use builtin.sanitary.toilet.floor_mounted
    in guest_bath
    attach west wall
    at 1.0
    facing east
    label "WC"
  }

  object guest_sink {
    use builtin.sanitary.sink.wall_hung
    in guest_bath
    attach north wall
    at 0.9
    facing south
    label "Sink"
  }

  object guest_shower {
    use builtin.sanitary.shower.size_900x900
    in guest_bath
    attach south wall
    at 1.3
    facing north
    label "Shower"
  }

  opening door d_foyer_great {
    between foyer and great_room
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_foyer_study {
    between foyer and study_guest
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_study_laundry {
    between study_guest and laundry
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_laundry_bath {
    between laundry and guest_bath
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bath_guest {
    between guest_bath and guest_bed
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_great_dining {
    between great_room and dining
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_dining_kitchen {
    between dining and galley_kitchen
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_great_primary {
    between great_room and primary_bed
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_primary_bath {
    between primary_bed and primary_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_bath_closet {
    between primary_bath and primary_closet
    on shared_edge
    at 50%
    swing rh
  }

  opening double door d_kitchen_lanai {
    on galley_kitchen.edge east
    at 50%
    swing lhr
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
