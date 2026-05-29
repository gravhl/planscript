# Keyword coverage example 09: west_of zone placement.

units m

plan "Zone Service Band" {
  footprint rect (0, 0) (18, 12)

  room living {
    rect (6, 0) (18, 8)
    label "Living"
  }

  zone service {
    label "Service Band"
    attach west_of living
    align top
    gap 0

    room kitchen {
      rect (0, 0) (6, 4)
      label "Kitchen"
    }

    room pantry {
      rect size (3, 4)
      attach north_of kitchen
      align left
      gap 0
      label "Pantry"
    }

    room laundry {
      rect size (3, 4)
      attach east_of pantry
      align top
      gap 0
      label "Laundry"
    }
  }

  room bedroom {
    rect (6, 8) (18, 12)
    label "Bedroom"
  }

  opening door d_living_kitchen { between living and kitchen on shared_edge at 50% width 0.9 swing lh }
  opening door d_living_bedroom { between living and bedroom on shared_edge at 50% width 0.9 swing rh }
  opening door d_kitchen_pantry { between kitchen and pantry on shared_edge at 50% width 0.9 swing lh }
  opening door d_pantry_laundry { between pantry and laundry on shared_edge at 50% width 0.9 swing rh }
  opening window w_living { on living.edge east at 4 width 2.0 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
