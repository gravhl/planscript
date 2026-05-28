units m

plan "Built-In Fixture House" {
  footprint rect (0, 0) (12, 10)

  room bath {
    rect (0, 0) (4, 4)
    label "Bath"
  }

  room kitchen {
    rect (4, 0) (12, 4)
    label "Kitchen"
  }

  room laundry {
    rect (0, 4) (4, 7)
    label "Laundry"
  }

  room bedroom {
    rect (4, 4) (8, 10)
    label "Bedroom"
  }

  room living {
    rect (8, 4) (12, 10)
    label "Living"
  }

  object wc1 {
    use builtin.sanitary.toilet.floor_mounted
    in bath
    attach west wall
    at 1.0
    facing east
    label "WC"
  }

  object lav1 {
    use builtin.sanitary.sink.wall_hung
    in bath
    attach north wall
    at 1.0
    facing south
    label "Sink"
  }

  object shower1 {
    use builtin.sanitary.shower.size_900x900
    in bath
    at (3.2, 0.2)
    facing north
    label "Shower"
  }

  object tub1 {
    use builtin.sanitary.tub.size_1700
    in bath
    attach east wall
    at 2.8
    facing west
    label "Tub"
  }

  object kitchen_sink1 {
    use builtin.kitchen.sink
    in kitchen
    attach north wall
    at 1.0
    facing south
    label "Kitchen Sink"
  }

  object range1 {
    use builtin.kitchen.range.size_600
    in kitchen
    attach north wall
    at 3.0
    facing south
    label "Range"
  }

  object fridge1 {
    use builtin.kitchen.fridge.size_900
    in kitchen
    attach east wall
    at 1.0
    facing west
    label "Fridge"
  }

  object washer1 {
    use builtin.laundry.washer
    in laundry
    attach north wall
    at 1.0
    facing south
    label "Washer"
  }

  object dryer1 {
    use builtin.laundry.dryer
    in laundry
    attach north wall
    at 2.4
    facing south
    label "Dryer"
  }

  object bed1 {
    use builtin.furniture.bed.queen
    in bedroom
    attach north wall
    at 2.0
    facing south
    label "Queen Bed"
  }

  object sofa1 {
    use builtin.furniture.sofa.three_seat
    in living
    attach south wall
    at 2.0
    facing north
    label "Sofa"
  }

  object dining1 {
    use builtin.furniture.table.dining_6
    in living
    at (10.0, 7.0)
    facing north
    label "Dining"
  }

  assert objects_inside_rooms
  assert object_no_overlap
  assert object_clearances
}
