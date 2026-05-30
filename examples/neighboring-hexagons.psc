# Two regular hexagonal rooms, each with 5 ft sides.

units ft

defaults {
  door_width 3
}

plan "Neighboring Hexagons" {
  footprint rect (2, 1) (14, 20)

  room lower_hex {
    polygon [
      (13, 6),
      (10.5, 10.330127),
      (5.5, 10.330127),
      (3, 6),
      (5.5, 1.669873),
      (10.5, 1.669873)
    ]
    label "Lower Hex"
  }

  room upper_hex {
    polygon [
      (13, 14.660254),
      (10.5, 18.990381),
      (5.5, 18.990381),
      (3, 14.660254),
      (5.5, 10.330127),
      (10.5, 10.330127)
    ]
    label "Upper Hex"
  }

  opening door d_hexes {
    between lower_hex and upper_hex
    on shared_edge
    at 50%
    width 3
    swing lh
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
  assert rooms_connected
}
