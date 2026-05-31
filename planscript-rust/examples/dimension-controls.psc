units m
dimension_units standard

defaults {
  door_width 0.9
}

plan "Dimension Controls" {
  footprint rect (0,0) (9,5)

  dimensions {
    walls bath.east kitchen.south
    fixtures wc lav range
  }

  room bath {
    rect (0,0) (3,5)
    label "Bath"
  }

  room kitchen {
    rect (3,0) (9,5)
    label "Kitchen"
  }

  object wc {
    use builtin.sanitary.toilet.floor_mounted
    in bath
    attach west wall
    at 30%
    facing east
  }

  object lav {
    use builtin.sanitary.sink.wall_hung
    in bath
    attach north wall
    at 65%
    facing south
  }

  object range {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach south wall
    at 2.35
    facing north
  }

  object range_left_counter {
    use builtin.kitchen.counter.size_24in
    in kitchen
    attach south wall
    at 1.588
    facing north
  }

  object range_right_counter {
    use builtin.kitchen.counter.size_36in
    in kitchen
    attach south wall
    at 3.2644
    facing north
  }

  object dishwasher {
    use builtin.kitchen.dishwasher.standard_24in
    in kitchen
    attach north wall
    at 4.10
    facing south
  }

  object kitchen_sink {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 4.85
    facing south
  }

  object fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach east wall
    at 3.75
    facing west
  }

  opening door d_bath_kitchen {
    between bath and kitchen
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_entry {
    on kitchen.edge south
    at 78%
    swing rh
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
}
