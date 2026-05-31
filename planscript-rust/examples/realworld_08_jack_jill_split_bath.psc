units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.2
  floor carpet
}

plan "Real World 08 Jack Jill Split Bath" {
  footprint rect (0,0) (12,5)
  legend {
    floor_materials auto
  }

  room bedroom_a {
    rect (0,0) (3,5)
    floor carpet
    label "Bedroom A"
  }

  room vanity_room {
    rect (3,0) (9,3)
    floor tile
    label "Shared Vanity"
  }

  room wet_room {
    rect (3,3) (9,5)
    floor tile
    label "Wet Room"
  }

  room bedroom_b {
    rect (9,0) (12,5)
    floor carpet
    label "Bedroom B"
  }

  object sink_a {
    use builtin.sanitary.sink.wall_hung
    in vanity_room
    attach north wall
    at 1.9
    facing south
    label "Sink"
  }

  object sink_b {
    use builtin.sanitary.sink.wall_hung
    in vanity_room
    attach north wall
    at 3.1
    facing south
    label "Sink"
  }

  object shower {
    use builtin.sanitary.shower.size_900x900
    in wet_room
    attach north wall
    at 1.8
    facing south
    label "Dual Shower"
  }

  object wc {
    use builtin.sanitary.toilet.floor_mounted
    in wet_room
    attach east wall
    at 0.7
    facing west
    label "WC"
  }

  opening door d_bed_a_vanity {
    between bedroom_a and vanity_room
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_bed_b_vanity {
    between bedroom_b and vanity_room
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_vanity_wet {
    between vanity_room and wet_room
    on shared_edge
    at 50%
    swing lh
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
