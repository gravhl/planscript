units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.4
  floor concrete
  outdoor_floor pavers
}

plan "Real World 02 Poolhouse BBQ Bath" {
  footprint rect (0,0) (8,7)
  legend {
    floor_materials auto
  }

  room bath {
    rect (0,0) (2.5,3.2)
    floor tile
    label "Pool Bath"
  }

  room storage {
    rect (0,3.2) (2.5,7)
    floor concrete
    label "Pool Storage"
  }

  room covered_patio {
    rect (2.5,0) (8,7)
    floor concrete
    label "Covered Patio"
  }

  outdoor patio pool_deck {
    rect (2.5,-4) (8,0)
    floor pavers
    label "Pool Deck"
  }

  object patio_sink {
    use builtin.kitchen.sink
    in covered_patio
    attach north wall
    at 0.8
    facing south
    label "Prep Sink"
  }

  object bbq_counter {
    use builtin.kitchen.counter.size_48in
    in covered_patio
    attach north wall
    at 2.0
    facing south
    label "BBQ Counter"
  }

  object grill_range {
    use builtin.kitchen.range.size_36in
    in covered_patio
    attach north wall
    at 3.1
    facing south
    label "Grill"
  }

  object patio_fridge {
    use builtin.kitchen.fridge.counter_depth_36in
    in covered_patio
    attach east wall
    at 1.2
    facing west
    label "Fridge"
  }

  object bath_shower {
    use builtin.sanitary.shower.size_900x900
    in bath
    attach south wall
    at 1.8
    facing north
    label "Walk-In Shower"
  }

  object bath_wc {
    use builtin.sanitary.toilet.floor_mounted
    in bath
    attach west wall
    at 1.05
    facing east
    label "WC"
  }

  object bath_sink {
    use builtin.sanitary.sink.wall_hung
    in bath
    attach north wall
    at 0.7
    facing south
    label "Sink"
  }

  opening door d_bath_patio {
    between bath and covered_patio
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_storage_patio {
    between storage and covered_patio
    on shared_edge
    at 50%
    swing rh
  }

  opening double door d_patio_pool {
    on covered_patio.edge south
    at 65%
    swing lhr
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert object_no_overlap
  assert openings_on_walls
}
