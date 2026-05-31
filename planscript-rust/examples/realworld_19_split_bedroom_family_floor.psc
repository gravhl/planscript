units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.4
  floor hardwood
}

plan "Real World 19 Split Bedroom Family Floor" {
  footprint rect (0,0) (16,12)
  legend {
    floor_materials auto
  }

  room primary_suite {
    rect (0,0) (5,2.5)
    floor carpet
    label "Primary"
  }

  room ensuite {
    rect (5,0) (8,2.5)
    floor tile
    label "Ensuite"
  }

  room closet {
    rect (8,0) (11,2.5)
    floor carpet
    label "Closet"
  }

  room bedroom_two {
    rect (11,0) (13.5,2.5)
    floor carpet
    label "Bedroom 2"
  }

  room bedroom_three {
    rect (13.5,0) (16,2.5)
    floor carpet
    label "Bedroom 3"
  }

  room office {
    rect (0,2.5) (3,5)
    floor hardwood
    label "Office"
  }

  room laundry {
    rect (8,2.5) (11,5)
    floor vinyl
    label "Laundry"
  }

  room shared_bath {
    rect (11,2.5) (16,5)
    floor tile
    label "Shared Bath"
  }

  room entry {
    rect (0,5) (3,8)
    floor hardwood
    label "Entry"
  }

  room formal_living {
    rect (0,8) (5,12)
    floor hardwood
    label "Formal Living"
  }

  room great_room {
    rect (5,5) (11,12)
    floor hardwood
    label "Great Room"
  }

  room kitchen {
    rect (11,5) (16,12)
    floor tile
    label "Kitchen"
  }

  object kitchen_sink {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 1.4
    facing south
    label "Sink"
  }

  object kitchen_dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in kitchen
    attach north wall
    at 2.3
    facing south
    label "DW"
  }

  object kitchen_range {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach east wall
    at 2.6
    facing west
    label "Range"
  }

  object kitchen_fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach south wall
    at 1.2
    facing north
    label "Fridge"
  }

  object island {
    use builtin.kitchen.counter.size_48in
    in kitchen
    at (13.5,8.0)
    facing north
    label "Island"
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
    at 0.8
    facing north
    label "Toilet"
  }

  object ensuite_shower {
    use builtin.sanitary.shower.size_900x900
    in ensuite
    attach east wall
    at 1.4
    facing west
    label "Shower"
  }

  object shared_sink {
    use builtin.sanitary.sink.wall_hung
    in shared_bath
    attach north wall
    at 1.0
    facing south
    label "Sink"
  }

  object shared_tub {
    use builtin.sanitary.tub.size_1700
    in shared_bath
    attach south wall
    at 1.1
    facing north
    label "Tub"
  }

  object shared_shower {
    use builtin.sanitary.shower.size_900x900
    in shared_bath
    attach east wall
    at 1.4
    facing west
    label "Shower"
  }

  object shared_wc {
    use builtin.sanitary.toilet.floor_mounted
    in shared_bath
    attach north wall
    at 3.8
    facing south
    label "Toilet"
  }

  object washer {
    use builtin.laundry.washer
    in laundry
    attach west wall
    at 0.8
    facing east
    label "Washer"
  }

  object dryer {
    use builtin.laundry.dryer
    in laundry
    attach west wall
    at 1.5
    facing east
    label "Dryer"
  }

  opening door d_primary_ensuite {
    between primary_suite and ensuite
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_ensuite_closet {
    between ensuite and closet
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_closet_laundry {
    between closet and laundry
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_laundry_great {
    between laundry and great_room
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bed_two_bath {
    between bedroom_two and shared_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_bed_three_bath {
    between bedroom_three and shared_bath
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bath_kitchen {
    between shared_bath and kitchen
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_office_entry {
    between office and entry
    on shared_edge
    at 50%
    swing rh
  }

  opening double door d_entry_formal {
    between entry and formal_living
    on shared_edge
    at 50%
    width 1.8
    swing lh
  }

  opening double door d_entry_great {
    between entry and great_room
    on shared_edge
    at 45%
    width 1.8
    swing rh
  }

  opening double door d_great_kitchen {
    between great_room and kitchen
    on shared_edge
    at 50%
    width 2.0
    swing lh
  }

  opening window w_great {
    on great_room.edge north
    at 50%
    width 2.5
  }

  opening window w_kitchen {
    on kitchen.edge north
    at 50%
    width 2.0
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
