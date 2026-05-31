units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.5
  floor hardwood
}

plan "Real World 05 Southern Jack Jill Floor" {
  footprint rect (0,0) (19,12)
  legend {
    floor_materials auto
  }

  room guest_suite {
    rect (0,0) (5,4)
    floor carpet
    label "Guest Suite"
  }

  room guest_bath {
    rect (5,0) (7.5,4)
    floor tile
    label "Guest Bath"
  }

  room mudroom {
    rect (7.5,0) (10,4)
    floor vinyl
    label "Mudroom"
  }

  room bedroom_a {
    rect (10,0) (13.5,4)
    floor carpet
    label "Bedroom"
  }

  room jack_jill_bath {
    rect (13.5,0) (15.5,4)
    floor tile
    label "Jack Jill"
  }

  room bedroom_b {
    rect (15.5,0) (19,4)
    floor carpet
    label "Bedroom"
  }

  room great_room {
    rect (0,4) (10,12)
    floor hardwood
    label "Great Room"
  }

  room kitchen {
    rect (10,4) (19,12)
    floor tile
    label "Chef Kitchen"
  }

  outdoor porch screened_porch {
    rect (0,12) (10,15)
    floor wood_deck
    label "Screened Porch"
  }

  object kitchen_island {
    use builtin.kitchen.counter.size_48in
    in kitchen
    at (14.5,7.0)
    facing north
    label "Island"
  }

  object sink {
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
    at 2.3
    facing south
    label "Dishwasher"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach south wall
    at 3.4
    facing north
    label "Range"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach east wall
    at 2.4
    facing west
    label "Fridge"
  }

  object jack_sink {
    use builtin.sanitary.sink.wall_hung
    in jack_jill_bath
    attach north wall
    at 0.65
    facing south
    label "Double Vanity"
  }

  object jack_wc {
    use builtin.sanitary.toilet.floor_mounted
    in jack_jill_bath
    attach west wall
    at 1.1
    facing east
    label "WC"
  }

  object jack_shower {
    use builtin.sanitary.shower.size_900x900
    in jack_jill_bath
    attach south wall
    at 1.35
    facing north
    label "Shower"
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
    at 1.0
    facing south
    label "Sink"
  }

  object guest_shower {
    use builtin.sanitary.shower.size_900x900
    in guest_bath
    attach south wall
    at 1.7
    facing north
    label "Shower"
  }

  opening door d_guest_bath {
    between guest_suite and guest_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_guest_great {
    between guest_suite and great_room
    on shared_edge
    at 60%
    swing rh
  }

  opening door d_guest_bath_mud {
    between guest_bath and mudroom
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_mud_great {
    between mudroom and great_room
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_great_kitchen {
    between great_room and kitchen
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bed_a_jack {
    between bedroom_a and jack_jill_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_jack_bed_b {
    between jack_jill_bath and bedroom_b
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bed_b_kitchen {
    between bedroom_b and kitchen
    on shared_edge
    at 50%
    swing lh
  }

  opening double door d_great_porch {
    on great_room.edge north
    at 50%
    swing lhr
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
