units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.4
  floor hardwood
}

plan "Real World 16 Functional Two Bed Apartment" {
  footprint rect (0,0) (10,8)
  legend {
    floor_materials auto
  }

  room bath {
    rect (0,0) (2,2.5)
    floor tile
    label "Bath"
  }

  room entry_hall {
    rect (0,2.5) (2,8)
    floor hardwood
    label "Entry Hall"
  }

  room bedroom_two {
    rect (2,0) (5,3.5)
    floor carpet
    label "Bedroom 2"
  }

  room bedroom_one {
    rect (5,0) (10,3.5)
    floor carpet
    label "Bedroom 1"
  }

  room living_dining {
    rect (2,3.5) (7,8)
    floor hardwood
    label "Living Dining"
  }

  room peninsula_kitchen {
    rect (7,3.5) (10,8)
    floor tile
    label "Peninsula Kitchen"
  }

  outdoor deck balcony {
    rect (2,8) (7,9.2)
    floor composite_deck
    label "Balcony"
  }

  object tub {
    use builtin.sanitary.tub.size_1700
    in bath
    attach south wall
    at 1.0
    facing north
    label "Tub"
  }

  object toilet {
    use builtin.sanitary.toilet.floor_mounted
    in bath
    attach east wall
    at 1.7
    facing west
    label "Toilet"
  }

  object lav {
    use builtin.sanitary.sink.wall_hung
    in bath
    attach north wall
    at 0.8
    facing south
    label "Sink"
  }

  object kitchen_sink {
    use builtin.kitchen.sink
    in peninsula_kitchen
    attach north wall
    at 1.0
    facing south
    label "Sink"
  }

  object range {
    use builtin.kitchen.range.size_36in
    in peninsula_kitchen
    attach east wall
    at 1.5
    facing west
    label "Range"
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in peninsula_kitchen
    attach south wall
    at 0.8
    facing north
    label "Fridge"
  }

  object peninsula {
    use builtin.kitchen.counter.size_48in
    in peninsula_kitchen
    attach west wall
    at 2.6
    facing east
    label "Peninsula"
  }

  object bed_one {
    use builtin.furniture.bed.queen
    in bedroom_one
    attach north wall
    at 2.5
    facing south
    label "Queen"
  }

  object bed_two {
    use builtin.furniture.bed.queen
    in bedroom_two
    attach north wall
    at 1.5
    facing south
    label "Guest"
  }

  object sofa {
    use builtin.furniture.sofa.three_seat
    in living_dining
    at (4.6,5.3)
    facing east
    label "Sofa"
  }

  object dining_table {
    use builtin.furniture.table.dining_6
    in living_dining
    at (6.0,6.7)
    facing north
    label "Dining"
  }

  opening door d_hall_bath {
    between entry_hall and bath
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_hall_living {
    between entry_hall and living_dining
    on shared_edge
    at 55%
    swing lh
  }

  opening door d_living_bed_two {
    between living_dining and bedroom_two
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_living_bed_one {
    between living_dining and bedroom_one
    on shared_edge
    at 35%
    swing lh
  }

  opening door d_living_kitchen {
    between living_dining and peninsula_kitchen
    on shared_edge
    at 50%
    width 1.4
    swing lh
  }

  opening double door d_living_balcony {
    between living_dining and balcony
    on shared_edge
    at 50%
    width 1.8
    swing lh
  }

  opening window w_bed_one {
    on bedroom_one.edge south
    at 50%
    width 2.0
  }

  opening window w_bed_two {
    on bedroom_two.edge south
    at 50%
    width 1.5
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
