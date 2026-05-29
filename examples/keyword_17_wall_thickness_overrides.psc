# Keyword coverage example 17: wall_thickness overrides for every edge direction.

units m

plan "Wall Thickness Overrides" {
  footprint rect (0, 0) (10, 10)

  room core {
    rect (2, 2) (8, 8)
    label "Core"
  }

  room south_room {
    rect (2, 0) (8, 2)
    label "South Room"
  }

  wall_thickness core.north 0.25
  wall_thickness core.south 0.18
  wall_thickness core.east 0.20
  wall_thickness core.west 0.20

  opening door d_core {
    between core and south_room
    on shared_edge
    at 50%
    width 0.9
    swing lh
  }

  opening window w_core {
    on core.edge north
    at 50%
    width 2.0
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
}
