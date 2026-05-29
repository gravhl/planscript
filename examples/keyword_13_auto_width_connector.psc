# Keyword coverage example 13: auto width with extend from right to left.

units m

plan "Auto Width Connector" {
  footprint rect (0, 0) (14, 6)

  room left_room {
    rect (0, 0) (4, 4)
    label "Left Room"
  }

  room right_room {
    rect (10, 0) (14, 4)
    label "Right Room"
  }

  room connector {
    rect size (auto, 4)
    attach east_of left_room
    align bottom
    extend from left_room.right to right_room.left
    gap 0
    label "Connector"
  }

  opening door d_left_connector { between left_room and connector on shared_edge at 50% width 0.9 swing lh }
  opening door d_connector_right { between connector and right_room on shared_edge at 50% width 0.9 swing rh }
  opening window w_connector { on connector.edge north at 50% width 2.0 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
