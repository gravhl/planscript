# Keyword coverage example 18: orientation targets in the northern hemisphere.

units m

defaults {
  door_width 0.9
  window_width 1.5
}

site {
  street east
  hemisphere north
}

plan "Northern Good Sun" {
  footprint rect (0, 0) (16, 10)

  room bedroom {
    rect (0, 0) (5, 10)
    label "Bedroom"
  }

  room living {
    rect (5, 0) (12, 10)
    label "Living"
  }

  room foyer {
    rect (12, 0) (16, 10)
    label "Foyer"
  }

  opening door d_entry { on foyer.edge east at 50% width 1.0 }
  opening door d_living_foyer { between living and foyer on shared_edge at 50% }
  opening door d_bed_living { between bedroom and living on shared_edge at 50% }
  opening window w_living_south { on living.edge south at 50% width 2.5 }
  opening window w_foyer_east { on foyer.edge east at 2.0 width 1.2 }
  opening window w_bed_west { on bedroom.edge west at 50% width 1.5 }

  assert orientation living has_window good_sun
  assert orientation foyer has_window street
  assert orientation foyer near street
  assert orientation bedroom away_from street
  assert orientation bedroom garden_view
  assert no_overlap rooms
  assert inside footprint all_rooms
}
