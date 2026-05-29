# Keyword coverage example 19: southern hemisphere good sun and street orientation.

units m

site {
  street north
  hemisphere south
}

plan "Southern Hemisphere Orientation" {
  footprint rect (0, 0) (14, 12)

  room suite {
    rect (0, 0) (6, 12)
    label "Suite"
  }

  room living {
    rect (6, 6) (14, 12)
    label "Living"
  }

  room kitchen {
    rect (6, 0) (14, 6)
    label "Kitchen"
  }

  opening door d_living_suite { between living and suite on shared_edge at 50% width 0.9 swing lh }
  opening door d_kitchen_suite { between kitchen and suite on shared_edge at 50% width 0.9 swing rh }
  opening window w_living_north { on living.edge north at 50% width 2.0 }
  opening window w_suite_south { on suite.edge south at 50% width 2.0 }
  opening window w_kitchen_east { on kitchen.edge east at 50% width 1.5 }

  assert orientation living has_window good_sun
  assert orientation living near street
  assert orientation suite away_from street
  assert orientation suite garden_view
  assert orientation kitchen has_window east
  assert no_overlap rooms
  assert inside footprint all_rooms
}
