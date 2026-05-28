# Keyword coverage example 10: centered rooms and simple align right.

units m

defaults {
  door_width 0.85
  window_width 1.2
}

plan "Centered Suite" {
  footprint rect (0, 0) (14, 10)

  room suite {
    rect center (7, 6) size (6, 4)
    label "Suite"
  }

  room bath {
    rect size (3, 3)
    attach south_of suite
    align right
    gap 0
    label "Bath"
  }

  room closet {
    rect size (3, 3)
    attach south_of suite
    align left
    gap 0
    label "Closet"
  }

  opening door d_suite { on suite.edge north at 50% }
  opening door d_bath { between suite and bath on shared_edge at 50% }
  opening door d_closet { between suite and closet on shared_edge at 50% }
  opening window w_suite { on suite.edge west at 3 width 1.5 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
