# Pocket door syntax gallery.

units m

defaults {
  door_width 0.9
  window_width 1.4
}

plan "Pocket Door Syntax Gallery" {
  footprint rect (0, 0) (12, 10)

  room hall {
    rect (4, 2) (8, 8)
    label "Hall"
  }

  room bath {
    rect (0, 2) (4, 5)
    label "Bath"
  }

  room closet {
    rect (0, 5) (4, 8)
    label "Closet"
  }

  room office {
    rect (8, 2) (12, 5)
    label "Office"
  }

  room laundry {
    rect (8, 5) (12, 8)
    label "Laundry"
  }

  opening pocket door d_bath_left {
    between hall and bath
    on shared_edge
    at 1.5
    width 0.9
    slide left
  }

  opening pocket_door d_closet_right {
    between hall and closet
    on shared_edge
    at 1.5
    width 0.8
    slide right
  }

  opening door d_office_block {
    between hall and office
    on shared_edge
    at 1.5
    width 0.9
    pocket
    slide left
  }

  opening pocket door d_laundry_default {
    between hall and laundry
    on shared_edge
    at 1.5
    width 0.9
  }

  opening pocket_door d_hall_entry {
    on hall.edge south
    at 50%
    width 1.1
    slide right
  }

  opening window w_bath {
    on bath.edge west
    at 50%
    width 1.2
    sill 1.0
  }

  opening window w_laundry {
    on laundry.edge east
    at 50%
    width 1.4
    sill 0.9
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
  assert rooms_connected
}
