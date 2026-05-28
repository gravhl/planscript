# Keyword coverage example 16: door swing and window sill.

units m

defaults {
  door_width 0.9
  window_width 1.5
}

plan "Openings Swing Sill" {
  footprint rect (0, 0) (12, 8)

  room foyer {
    rect (0, 0) (4, 8)
    label "Foyer"
  }

  room living {
    rect (4, 0) (12, 8)
    label "Living"
  }

  opening door d_entry {
    on foyer.edge south
    at 50%
    width 1.0
    swing foyer
  }

  opening door d_living {
    between foyer and living
    on shared_edge
    at 4.0
    swing living
  }

  opening window w_low {
    on living.edge south
    at 2.0
    sill 0.6
  }

  opening window w_high {
    on living.edge north
    at 50%
    width 2.0
    sill 1.4
  }

  assert no_overlap rooms
  assert openings_on_walls
  assert rooms_connected
}
