units ft

defaults {
  door_width 3
}

plan "Dimension Controls" {
  footprint rect (0,0) (18,12)

  dimensions {
    walls bath.east kitchen.south
    fixtures wc lav range
  }

  room bath {
    rect (0,0) (7,12)
    label "Bath"
  }

  room kitchen {
    rect (7,0) (18,12)
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
    use builtin.kitchen.range.size_600
    in kitchen
    attach south wall
    at 30%
    facing north
  }

  object fridge {
    use builtin.kitchen.fridge.size_900
    in kitchen
    attach east wall
    at 70%
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
    at 70%
    swing rh
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
}
