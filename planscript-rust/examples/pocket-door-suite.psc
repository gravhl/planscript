# Compact suite using pocket doors where swing clearance matters.

units ft
dimension_units standard

defaults {
  door_width 3
  window_width 4
}

plan "Pocket Door Suite" {
  footprint rect (0, 0) (26, 16)

  room bedroom {
    rect (0, 0) (14, 16)
    label "Bedroom"
  }

  room dressing {
    rect (14, 0) (20, 8)
    label "Dressing"
  }

  room bath {
    rect (20, 0) (26, 8)
    label "Bath"
  }

  room closet {
    rect (14, 8) (26, 16)
    label "Closet"
  }

  opening pocket door d_bed_dressing {
    between bedroom and dressing
    on shared_edge
    at 5.5
    width 3
    slide right
  }

  opening pocket door d_dressing_bath {
    between dressing and bath
    on shared_edge
    at 2.5
    width 3
    slide left
  }

  opening door d_dressing_closet {
    between dressing and closet
    on shared_edge
    at 2
    width 2.5
    pocket
  }

  opening window w_bedroom {
    on bedroom.edge west
    at 50%
    width 5
    sill 3
  }

  opening window w_bath {
    on bath.edge east
    at 50%
    width 3
    sill 4
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
  assert rooms_connected
}
