units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.5
  floor hardwood
}

plan "Real World 04 Split Ranch U Kitchen" {
  footprint rect (0,0) (19,13)
  legend {
    floor_materials auto
  }

  room den {
    rect (0,0) (7,4)
    floor hardwood
    label "Den"
  }

  room entry {
    rect (7,0) (10,4)
    floor hardwood
    label "Entry"
  }

  room bedroom1 {
    rect (10,0) (13,4)
    floor carpet
    label "Bedroom"
  }

  room shared_bath {
    rect (13,0) (15.5,4)
    floor tile
    label "Hall Bath"
  }

  room bedroom2 {
    rect (15.5,0) (19,4)
    floor carpet
    label "Bedroom"
  }

  room primary_bath {
    rect (0,4) (2.5,7)
    floor tile
    label "5-Fixture Bath"
  }

  room primary_closet {
    rect (2.5,4) (5,7)
    floor carpet
    label "WIC"
  }

  room primary_bed {
    rect (0,7) (5,13)
    floor carpet
    label "Primary"
  }

  room great_room {
    rect (5,4) (13,13)
    floor hardwood
    label "Great Room"
  }

  room kitchen_dining {
    rect (13,4) (19,13)
    floor tile
    label "Kitchen Dining"
  }

  object prep_island {
    use builtin.kitchen.counter.size_48in
    in kitchen_dining
    at (16.0,7.2)
    facing north
    label "Prep Island"
  }

  object kitchen_sink {
    use builtin.kitchen.sink
    in kitchen_dining
    attach north wall
    at 1.2
    facing south
    label "Sink"
  }

  object range_left_counter {
    use builtin.kitchen.counter.size_24in
    in kitchen_dining
    attach south wall
    at 1.738
    facing north
    label "Landing"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in kitchen_dining
    attach south wall
    at 2.5
    facing north
    label "Range"
  }

  object range_right_counter {
    use builtin.kitchen.counter.size_36in
    in kitchen_dining
    attach south wall
    at 3.4144
    facing north
    label "Landing"
  }

  object pantry_fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen_dining
    attach east wall
    at 2.0
    facing west
    label "Fridge"
  }

  object primary_tub {
    use builtin.sanitary.tub.size_1700
    in primary_bath
    attach north wall
    at 1.2
    facing south
    label "Tub"
  }

  object primary_shower {
    use builtin.sanitary.shower.size_900x900
    in primary_bath
    attach south wall
    at 1.75
    facing north
    label "Shower"
  }

  object primary_wc {
    use builtin.sanitary.toilet.floor_mounted
    in primary_bath
    attach west wall
    at 1.5
    facing east
    label "WC"
  }

  object primary_sink1 {
    use builtin.sanitary.sink.wall_hung
    in primary_bath
    attach east wall
    at 2.2
    facing west
    label "Vanity"
  }

  object hall_wc {
    use builtin.sanitary.toilet.floor_mounted
    in shared_bath
    attach west wall
    at 1.0
    facing east
    label "WC"
  }

  object hall_sink {
    use builtin.sanitary.sink.wall_hung
    in shared_bath
    attach north wall
    at 1.4
    facing south
    label "Sink"
  }

  object hall_tub {
    use builtin.sanitary.tub.size_1700
    in shared_bath
    attach south wall
    at 1.3
    facing north
    label "Tub"
  }

  opening door d_entry_great {
    between entry and great_room
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_den_entry {
    between den and entry
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_entry_bedroom1 {
    between entry and bedroom1
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

  opening door d_primary_great {
    between primary_bed and great_room
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

  opening door d_primary_closet {
    between primary_bed and primary_closet
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_great_kitchen {
    between great_room and kitchen_dining
    on shared_edge
    at 55%
    swing lh
  }

  opening double door d_dining_backyard {
    on kitchen_dining.edge north
    at 70%
    swing rhr
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
