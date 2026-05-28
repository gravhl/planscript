# Keyword coverage example 11: fill between, width, and height directives.

units m

plan "Fill Between Bridge" {
  footprint rect (0, 0) (12, 8)

  room west_room {
    rect (0, 0) (4, 8)
    label "West Room"
  }

  room east_room {
    rect (8, 0) (12, 8)
    label "East Room"
  }

  room bridge {
    fill between west_room and east_room
    width 4
    height 3
    label "Bridge"
  }

  opening door d_west_bridge { between west_room and bridge on shared_edge at 50% width 0.9 }
  opening door d_bridge_east { between bridge and east_room on shared_edge at 50% width 0.9 }
  opening window w_west { on west_room.edge west at 4 width 1.4 }
  opening window w_east { on east_room.edge east at 4 width 1.4 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
