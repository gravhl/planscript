# Keyword coverage example 03: millimeters and explicit room labels.

units mm
grid 100

defaults {
  door_width 850
  window_width 1400
}

plan "Millimeter Studio" {
  footprint rect (0, 0) (8000, 6000)

  room studio {
    rect (0, 0) (5000, 6000)
    label "Studio"
  }

  room bath {
    rect at (5000, 0) size (3000, 2500)
    label "Bath"
  }

  room storage {
    rect size (3000, 3500)
    attach north_of bath
    align left
    gap 0
    label "Storage"
  }

  opening door d_entry { on studio.edge south at 4000 width 900 }
  opening door d_bath { between studio and bath on shared_edge at 50% width 850 }
  opening window w_studio { on studio.edge west at 3000 width 1200 sill 900 }

  assert no_overlap rooms
  assert inside footprint studio
  assert min_room_area bath >= 7000000
}
