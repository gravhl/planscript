# Keyword coverage example 04: feet.

units ft

defaults {
  door_width 3
  window_width 5
}

plan "Feet Cabin" {
  footprint rect (0, 0) (32, 24)

  room great_room {
    rect (0, 0) (18, 24)
    label "Great Room"
  }

  room bedroom {
    rect (18, 10) (32, 24)
    label "Bedroom"
  }

  room bath {
    rect size (14, 10)
    attach south_of bedroom
    align right
    gap 0
    label "Bath"
  }

  opening door d_front { on great_room.edge south at 50% swing rhr }
  opening door d_bed { between great_room and bedroom on shared_edge at 50% swing lh }
  opening door d_bath { between great_room and bath on shared_edge at 50% swing rh }
  opening window w_great { on great_room.edge west at 12 width 6 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
