units m
dimension_units standard

defaults {
  door_width 0.9
  window_width 1.2
  floor tile
}

plan "Real World 15 Modern Primary Bath Laundry" {
  footprint rect (0,0) (9.0,3.6)
  legend {
    floor_materials auto
  }

  room bedroom {
    rect (0,0) (3.0,3.6)
    floor hardwood
    label "Bedroom"
  }

  room primary_bath {
    rect (3.0,0) (7.4,3.6)
    floor tile
    label "Primary Bath"
  }

  room water_closet {
    rect (7.4,0) (9.0,1.8)
    floor tile
    label "WC"
  }

  room laundry_niche {
    rect (7.4,1.8) (9.0,3.6)
    floor vinyl
    label "Laundry"
  }

  object vanity_left {
    use builtin.sanitary.sink.wall_hung
    in primary_bath
    attach north wall
    at 1.1
    facing south
    label "Vanity"
  }

  object vanity_right {
    use builtin.sanitary.sink.wall_hung
    in primary_bath
    attach north wall
    at 2.2
    facing south
    label "Vanity"
  }

  object tub {
    use builtin.sanitary.tub.size_1700
    in primary_bath
    attach south wall
    at 1.1
    facing north
    label "Tub"
  }

  object shower {
    use builtin.sanitary.shower.size_900x900
    in primary_bath
    attach south wall
    at 3.1
    facing north
    label "Shower"
  }

  object toilet {
    use builtin.sanitary.toilet.floor_mounted
    in water_closet
    attach east wall
    at 0.9
    facing west
    label "Toilet"
  }

  object washer {
    use builtin.laundry.washer
    in laundry_niche
    attach east wall
    at 0.65
    facing west
    label "Washer"
  }

  object dryer {
    use builtin.laundry.dryer
    in laundry_niche
    attach east wall
    at 1.25
    facing west
    label "Dryer"
  }

  opening door d_bed_bath {
    between bedroom and primary_bath
    on shared_edge
    at 50%
    swing lh
  }

  opening door d_bath_wc {
    between primary_bath and water_closet
    on shared_edge
    at 45%
    swing rh
  }

  opening door d_bath_laundry {
    between primary_bath and laundry_niche
    on shared_edge
    at 55%
    swing lh
  }

  opening window w_bath {
    on primary_bath.edge south
    at 75%
    width 1.2
  }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert objects_inside_rooms
  assert openings_on_walls
  assert rooms_connected
}
