# Keyword coverage example 05: inches and rect center size.

units in
grid 6

defaults {
  door_width 32
  window_width 48
}

plan "Inches Micro Suite" {
  footprint rect (0, 0) (480, 360)

  room lounge {
    rect center (180, 180) size (240, 240)
    label "Lounge"
  }

  room bath {
    rect size (120, 120)
    attach east_of lounge
    align center
    gap 0
    label "Bath"
  }

  room closet {
    rect size (120, 120)
    attach south_of bath
    align right
    gap 0
    label "Closet"
  }

  opening door d_lounge { on lounge.edge south at 50% swing rhr }
  opening door d_bath { between lounge and bath on shared_edge at 50% swing lh }
  opening door d_closet { between bath and closet on shared_edge at 50% swing rh }
  opening window w_lounge { on lounge.edge north at 50% }

  assert no_overlap rooms
  assert inside footprint all_rooms
}
