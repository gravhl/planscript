units m
dimension_units standard

defaults {
  door_width 0.8
  window_width 1.0
  floor tile
}

plan "Real World 20 Square Industrial Three Quarter Bath" {
  footprint rect (0,0) (3.9,2.6)
  legend {
    floor_materials auto
  }

  room hall {
    rect (0,0) (1.2,2.6)
    floor concrete
    label "Hall"
  }

  room three_quarter_bath {
    rect (1.2,0) (3.9,2.6)
    floor tile
    label "3/4 Bath"
  }

  object vanity {
    use builtin.sanitary.sink.wall_hung
    in three_quarter_bath
    attach north wall
    at 0.8
    facing south
    label "Vanity"
  }

  object toilet {
    use builtin.sanitary.toilet.floor_mounted
    in three_quarter_bath
    attach east wall
    at 0.85
    facing west
    label "Toilet"
  }

  object shower {
    use builtin.sanitary.shower.size_900x900
    in three_quarter_bath
    attach south wall
    at 1.8
    facing north
    label "Shower"
  }

  opening door d_hall_bath {
    between hall and three_quarter_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening window w_bath {
    on three_quarter_bath.edge north
    at 75%
    width 1.0
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
