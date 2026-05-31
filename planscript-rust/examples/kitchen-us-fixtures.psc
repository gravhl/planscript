units m

plan "US Kitchen Fixtures" {
  footprint rect (0, 0) (11, 6)

  room kitchen {
    rect (0, 0) (8, 6)
    label "Kitchen"
  }

  room pantry {
    rect (8, 0) (11, 6)
    label "Pantry"
  }

  object sink1 {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 1.0
    facing south
    label "Kitchen Sink"
  }

  object dishwasher24 {
    use builtin.kitchen.dishwasher.standard_24in
    in kitchen
    attach north wall
    at 1.75
    facing south
    label "24 in Dishwasher"
  }

  object prep_counter36 {
    use builtin.kitchen.counter.size_36in
    in kitchen
    attach north wall
    at 2.70
    facing south
    label "36 in Prep Counter"
  }

  object range_left_counter24 {
    use builtin.kitchen.counter.size_24in
    in kitchen
    attach north wall
    at 3.55
    facing south
    label "24 in Landing"
  }

  object range36 {
    use builtin.kitchen.range.size_36in
    in kitchen
    attach north wall
    at 4.42
    facing south
    label "36 in Range"
  }

  object range_right_counter36 {
    use builtin.kitchen.counter.size_36in
    in kitchen
    attach north wall
    at 5.35
    facing south
    label "36 in Landing"
  }

  object counter48 {
    use builtin.kitchen.counter.size_48in
    in kitchen
    attach north wall
    at 6.80
    facing south
    label "48 in Counter"
  }

  object fridge36cd {
    use builtin.kitchen.fridge.counter_depth_36in
    in kitchen
    attach east wall
    at 1.0
    facing west
    label "36 in Counter Depth Fridge"
  }

  opening door d_kitchen_pantry {
    between kitchen and pantry
    on shared_edge
    at 50%
    swing lh
  }

  assert objects_inside_rooms
  assert object_no_overlap
}
