units ft
render {
  mode draft
}

defaults {
  door_width 3
  window_width 4
  floor hardwood
  outdoor_floor pavers
}

plan "Outdoor Floor Materials Draft" {
  footprint rect (0,0) (28,18)
  legend {
    floor_materials auto
  }

  room living {
    rect (0,0) (16,18)
    floor hardwood
    label "Living"
  }

  room kitchen {
    rect (16,0) (28,18)
    floor tile
    label "Kitchen"
  }

  outdoor deck rear_deck {
    rect (0,18) (16,26)
    floor wood_deck
    label "Rear Deck"
  }

  outdoor patio kitchen_patio {
    rect (16,18) (28,26)
    floor pavers
    label "Patio"
  }

  outdoor walkway entry_walk {
    rect (6,-5) (12,0)
    floor concrete
    label "Entry Walk"
  }

  opening door d_living_kitchen {
    between living and kitchen
    on shared_edge
    at 50%
    swing rh
  }

  opening double door d_deck {
    on living.edge north
    at 50%
    swing rhr
  }

  opening door d_entry {
    on living.edge south
    at 50%
    swing lh
  }

  opening window w_kitchen {
    on kitchen.edge east
    at 50%
    width 5
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
}
