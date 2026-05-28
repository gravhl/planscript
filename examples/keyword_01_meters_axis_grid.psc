# Keyword coverage example 01: meters, origin, axis, grid, defaults.

units meters
origin (-2, -2)
axis x: right y: up
grid 0.5

defaults {
  door_width 0.9
  window_width 1.4
}

plan "Meters Axis Grid" {
  footprint rect (0, 0) (12, 8)

  room studio {
    rect (0, 0) (6, 8)
    label "Studio"
  }

  room service {
    rect size (3, 3)
    attach east_of studio
    align bottom
    gap 0
    label "Service"
  }

  room nook {
    rect size (3, 5)
    attach north_of service
    align left
    gap 0
    label "Nook"
  }

  opening door d_front {
    on studio.edge south
    at 50%
  }

  opening door d_service {
    between studio and service
    on shared_edge
    at 50%
  }

  opening window w_nook {
    on nook.edge east
    at 50%
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
