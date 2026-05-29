# Keyword coverage example 08: zones attached to rooms and other zones.

units m

defaults {
  door_width 0.9
  window_width 1.5
}

plan "Attached Zones" {
  footprint rect (0, 0) (24, 14)

  room entry {
    rect (8, 0) (14, 3)
    label "Entry"
  }

  zone social {
    label "Social Zone"
    attach north_of entry
    align center
    gap 0

    room living {
      rect (0, 0) (6, 5)
      label "Living"
    }

    room dining {
      rect size (4, 5)
      attach east_of living
      align top
      gap 0
      label "Dining"
    }
  }

  zone private {
    label "Private Zone"
    attach east_of social
    align bottom
    gap 0

    room bedroom {
      rect (0, 0) (5, 5)
      label "Bedroom"
    }

    room bath {
      rect size (3, 5)
      attach east_of bedroom
      align bottom
      gap 0
      label "Bath"
    }
  }

  opening door d_front { on entry.edge south at 50% swing rhr }
  opening door d_entry_living { between entry and living on shared_edge at 50% swing lh }
  opening door d_living_dining { between living and dining on shared_edge at 50% swing rh }
  opening door d_dining_bedroom { between dining and bedroom on shared_edge at 50% swing lh }
  opening door d_bedroom_bath { between bedroom and bath on shared_edge at 50% swing rh }
  opening window w_bedroom { on bedroom.edge east at 50% }

  assert no_overlap rooms
  assert inside footprint all_rooms
}
