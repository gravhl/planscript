units ft

defaults {
  door_width 3
  floor hardwood
  outdoor_floor pavers
}

plan "Floor Material Gallery" {
  footprint rect (0,0) (36,20)
  legend {
    floor_materials auto
  }

  room hall {
    rect (0,8) (36,12)
    floor polished_concrete
    label "Polished Concrete"
  }

  room hardwood_room {
    rect (0,12) (6,20)
    floor hardwood
    label "Hardwood"
  }

  room tile_room {
    rect (6,12) (12,20)
    floor tile
    label "Tile"
  }

  room carpet_room {
    rect (12,12) (18,20)
    floor carpet
    label "Carpet"
  }

  room vinyl_room {
    rect (18,12) (24,20)
    floor vinyl
    label "Vinyl"
  }

  room stone_room {
    rect (24,12) (30,20)
    floor stone
    label "Stone"
  }

  room concrete_room {
    rect (30,12) (36,20)
    floor concrete
    label "Concrete"
  }

  outdoor deck wood_deck_sample {
    rect (0,-5) (6,0)
    floor wood_deck
    label "Wood Deck"
  }

  outdoor deck composite_deck_sample {
    rect (6,-5) (12,0)
    floor composite_deck
    label "Composite Deck"
  }

  outdoor patio paver_sample {
    rect (12,-5) (18,0)
    floor pavers
    label "Pavers"
  }

  outdoor garden gravel_sample {
    rect (18,-5) (24,0)
    floor gravel
    label "Gravel"
  }

  outdoor garden grass_sample {
    rect (24,-5) (30,0)
    floor grass
    label "Grass"
  }

  outdoor garden mulch_sample {
    rect (30,-5) (36,0)
    floor mulch
    label "Mulch"
  }

  opening door d_hardwood {
    between hall and hardwood_room
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_tile {
    between hall and tile_room
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_carpet {
    between hall and carpet_room
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_vinyl {
    between hall and vinyl_room
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_stone {
    between hall and stone_room
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_concrete {
    between hall and concrete_room
    on shared_edge
    at 50%
    swing rh
  }

  opening door d_entry {
    on hall.edge south
    at 50%
    swing lh
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
}
