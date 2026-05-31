units m
dimension_units standard

defaults {
  door_width 1.1
  window_width 1.2
  floor tile
}

plan "Real World 09 Accessible Roll In Bath With Warning" {
  footprint rect (0,0) (5,3.2)
  legend {
    floor_materials auto
  }

  room hall {
    rect (0,0) (1.2,3.2)
    floor vinyl
    label "Hall"
  }

  room accessible_bath {
    rect (1.2,0) (5,3.2)
    floor tile
    label "Clear Turning Area"
  }

  object roll_under_sink {
    use builtin.sanitary.sink.wall_hung
    in accessible_bath
    attach north wall
    at 0.9
    facing south
    label "Roll-Under Sink"
  }

  object comfort_wc {
    use builtin.sanitary.toilet.floor_mounted
    in accessible_bath
    attach east wall
    at 1.15
    facing west
    label "Transfer WC"
  }

  object roll_in_shower {
    use builtin.sanitary.shower.size_900x900
    in accessible_bath
    attach south wall
    at 2.7
    facing north
    label "Roll-In Shower"
  }

  opening pocket door d_hall_bath {
    between hall and accessible_bath
    on shared_edge
    at 47%
    width 1.1
    slide right
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
