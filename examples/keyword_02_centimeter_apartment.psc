# Keyword coverage example 02: centimeters and alternate axis directions.

units cm
origin (0, 0)
axis x: left y: down
grid 10

defaults {
  door_width 90
  window_width 120
}

plan "Centimeter Apartment" {
  footprint rect (0, 0) (900, 600)

  room living {
    rect (0, 0) (450, 600)
    label "Living"
  }

  room kitchen {
    rect at (450, 0) size (250, 300)
    label "Kitchen"
  }

  room bath {
    rect size (200, 300)
    attach east_of kitchen
    align top
    gap 0
    label "Bath"
  }

  room bedroom {
    rect (450, 300) (900, 600)
    label "Bedroom"
  }

  opening door d_living_kitchen { between living and kitchen on shared_edge at 50% }
  opening door d_kitchen_bath { between kitchen and bath on shared_edge at 50% }
  opening door d_living_bedroom { between living and bedroom on shared_edge at 50% }
  opening window w_bedroom { on bedroom.edge north at 50% }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
}
