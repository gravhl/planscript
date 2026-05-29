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
    attach south wall
    at 3.2
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
    use builtin.kitchen.range.size_30in
    in kitchen
    attach north wall
    at 3.0
    facing south
    label "Range"
  }

  object fridge1 {
    use builtin.kitchen.fridge.counter_depth_36in
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

  opening door d_bath_kitchen {
    between bath and kitchen
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_bath_laundry {
    between bath and laundry
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_kitchen_bedroom {
    between kitchen and bedroom
    on shared_edge
    at 50%
    swing lh
  }

  opening double door d_kitchen_living {
    between kitchen and living
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_bedroom_living {
    between bedroom and living
    on shared_edge
    at 50%
    width 1.0
    swing rhr
  }

  assert objects_inside_rooms
  assert object_no_overlap
  assert object_clearances
}
