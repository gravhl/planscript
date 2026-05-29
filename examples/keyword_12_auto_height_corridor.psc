# Keyword coverage example 12: auto height with extend from bottom to top.

units m

plan "Auto Height Corridor" {
  footprint rect (0, 0) (8, 12)

  room living {
    rect (0, 0) (5, 4)
    label "Living"
  }

  room bedroom {
    rect (0, 8) (5, 12)
    label "Bedroom"
  }

  room hall {
    rect size (1.5, auto)
    attach east_of living
    align bottom
    extend from living.bottom to bedroom.top
    gap 0
    label "Hall"
  }

  opening door d_living_hall { between living and hall on shared_edge at 50% width 0.9 swing lh }
  opening door d_hall_bedroom { between hall and bedroom on shared_edge at 50% width 0.9 swing rh }
  opening window w_bedroom { on bedroom.edge north at 50% width 1.5 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
