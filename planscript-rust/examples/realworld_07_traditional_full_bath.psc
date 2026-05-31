units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.2
  floor tile
}

plan "Real World 07 Traditional Full Bath" {
  footprint rect (0,0) (6,4)
  legend {
    floor_materials auto
  }

  room hall {
    rect (0,0) (1.5,4)
    floor hardwood
    label "Hall"
  }

  room bath {
    rect (1.5,0) (6,4)
    floor tile
    label "Full Bath"
  }

  object vanity_left {
    use builtin.sanitary.sink.wall_hung
    in bath
    attach north wall
    at 1.3
    facing south
    label "Vanity"
  }

  object vanity_right {
    use builtin.sanitary.sink.wall_hung
    in bath
    attach north wall
    at 2.3
    facing south
    label "Vanity"
  }

  object tub {
    use builtin.sanitary.tub.size_1700
    in bath
    attach east wall
    at 1.0
    facing west
    label "Tub"
  }

  object shower {
    use builtin.sanitary.shower.size_900x900
    in bath
    attach south wall
    at 1.2
    facing north
    label "Walk-In Shower"
  }

  object wc {
    use builtin.sanitary.toilet.floor_mounted
    in bath
    attach south wall
    at 2.6
    facing north
    label "WC"
  }

  opening door d_hall_bath {
    between hall and bath
    on shared_edge
    at 50%
    swing lh
  }

  opening window w_bath {
    on bath.edge east
    at 70%
    width 1.0
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
