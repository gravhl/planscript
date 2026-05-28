# Keyword coverage example 07: polygon courtyard.

units m

plan "Polygon Courtyard Atrium" {
  footprint rect (0, 0) (16, 16)

  room south_hall {
    rect (0, 0) (16, 5)
    label "South Hall"
  }

  room west_studio {
    rect (0, 5) (5, 16)
    label "West Studio"
  }

  room east_suite {
    rect (11, 5) (16, 16)
    label "East Suite"
  }

  room north_lounge {
    rect (5, 11) (11, 16)
    label "North Lounge"
  }

  courtyard atrium {
    polygon [
      (5, 5),
      (11, 5),
      (11, 11),
      (5, 11)
    ]
    label "Atrium"
  }

  opening door d_south_west { between south_hall and west_studio on shared_edge at 50% width 0.9 }
  opening door d_south_east { between south_hall and east_suite on shared_edge at 50% width 0.9 }
  opening door d_west_north { between west_studio and north_lounge on shared_edge at 50% width 0.9 }
  opening window w_lounge { on north_lounge.edge north at 50% width 2.0 sill 0.8 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
