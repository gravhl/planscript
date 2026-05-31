units m
dimension_units standard

defaults {
  door_width 0.8
  window_width 1.0
  floor tile
}

plan "Real World 14 Rectangular Bath Laundry" {
  footprint rect (0,0) (4.4,3.2)
  legend {
    floor_materials auto
  }

  room hall {
    rect (0,0) (1.2,3.2)
    floor hardwood
    label "Hall"
  }

  room bath_laundry {
    rect (1.2,0) (4.4,3.2)
    floor tile
    label "Bath Laundry"
  }

  object vanity {
    use builtin.sanitary.sink.wall_hung
    in bath_laundry
    attach west wall
    at 0.8
    facing east
    label "Vanity"
  }

  object toilet {
    use builtin.sanitary.toilet.floor_mounted
    in bath_laundry
    attach east wall
    at 0.85
    facing west
    label "Toilet"
  }

  object tub {
    use builtin.sanitary.tub.size_1700
    in bath_laundry
    attach north wall
    at 2.2
    facing south
    label "Tub Shower"
  }

  object washer {
    use builtin.laundry.washer
    in bath_laundry
    attach south wall
    at 1.2
    facing north
    label "Washer"
  }

  object dryer {
    use builtin.laundry.dryer
    in bath_laundry
    attach south wall
    at 2.0
    facing north
    label "Dryer"
  }

  opening door d_hall_bath {
    between hall and bath_laundry
    on shared_edge
    at 50%
    swing rh
  }

  opening window w_bath {
    on bath_laundry.edge north
    at 80%
    width 1.0
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
