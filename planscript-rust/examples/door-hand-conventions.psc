# Door hand conventions and double doors.

units m

defaults {
  door_width 0.9
  window_width 1.5
}

plan "Door Hand Conventions" {
  footprint rect (0, 0) (16, 10)

  room hall {
    rect (0, 0) (4, 10)
    label "Hall"
  }

  room suite {
    rect (4, 0) (10, 5)
    label "Suite"
  }

  room studio {
    rect (4, 5) (10, 10)
    label "Studio"
  }

  room lounge {
    rect (10, 0) (16, 10)
    label "Lounge"
  }

  opening door d_lh {
    on hall.edge west
    at 2.0
    swing lh
  }

  opening door d_rh {
    on hall.edge west
    at 8.0
    swing rh
  }

  opening door d_lhr {
    between hall and suite
    on shared_edge
    at 2.5
    swing lhr
  }

  opening door d_rhr {
    between hall and studio
    on shared_edge
    at 2.5
    width 1.0
    swing rhr
  }

  opening double door d_double {
    between lounge and suite
    on shared_edge
    at 2.5
    swing lh
  }

  opening door d_custom_double {
    between lounge and studio
    on shared_edge
    at 2.5
    width 1.6
    double
    swing rh
  }

  opening window w_lounge {
    on lounge.edge east
    at 50%
    width 2.4
    sill 0.9
  }

  assert no_overlap rooms
  assert openings_on_walls
  assert rooms_connected
}
