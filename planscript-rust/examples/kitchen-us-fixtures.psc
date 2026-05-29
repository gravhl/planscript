units m

plan "US Kitchen Fixtures" {
  footprint rect (0, 0) (13, 5)

  room kitchen {
    rect (0, 0) (10, 5)
    label "Kitchen"
  }

  room pantry {
    rect (10, 0) (13, 5)
    label "Pantry"
  }

  object counter12 {
    use builtin.kitchen.counter.size_12in
    in kitchen
    attach north wall
    at 0.3
    facing south
    label "12 in Counter"
  }

  object counter24 {
    use builtin.kitchen.counter.size_24in
    in kitchen
    attach north wall
    at 0.9
    facing south
    label "24 in Counter"
  }

  object counter36 {
    use builtin.kitchen.counter.size_36in
    in kitchen
    attach north wall
    at 1.7
    facing south
    label "Stovetop Counter"
  }

  object cooktop30 {
    use builtin.kitchen.stovetop.size_30in
    in kitchen
    attach north wall
    at 1.7
    facing south
    label "30 in Stovetop"
  }

  object counter48 {
    use builtin.kitchen.counter.size_48in
    in kitchen
    attach north wall
    at 3.7
    facing south
    label "48 in Counter"
  }

  object range30 {
    use builtin.kitchen.range.size_30in
    in kitchen
    attach north wall
    at 5.0
    facing south
    label "30 in Range"
  }

  object fridge36 {
    use builtin.kitchen.fridge.standard_36in
    in kitchen
    attach east wall
    at 1.0
    facing west
    label "36 in Fridge"
  }

  object fridge33cd {
    use builtin.kitchen.refrigerator.counter_depth_33in
    in kitchen
    attach south wall
    at 7.8
    facing north
    label "33 in Counter Depth Fridge"
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
