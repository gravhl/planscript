# Keyword coverage example 06: bracketed polygon footprint and polygon room.

units m

plan "Polygon Footprint Gallery" {
  footprint polygon [
    (0, 0),
    (14, 0),
    (14, 6),
    (8, 6),
    (8, 12),
    (0, 12)
  ]

  room entry {
    rect (0, 0) (6, 6)
    label "Entry"
  }

  room connector {
    rect (6, 0) (8, 6)
    label "Connector"
  }

  room office {
    rect (8, 0) (14, 6)
    label "Office"
  }

  room gallery {
    polygon [
      (0, 6),
      (8, 6),
      (8, 12),
      (0, 12)
    ]
    label "Gallery"
  }

  opening door d_entry { on entry.edge south at 50% width 1.0 }
  opening door d_entry_connector { between entry and connector on shared_edge at 50% width 0.9 }
  opening door d_connector_office { between connector and office on shared_edge at 50% width 0.9 }
  opening door d_entry_gallery { between entry and gallery on shared_edge at 50% width 0.9 }
  opening window w_gallery { on gallery.edge north at 50% width 2.0 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert openings_on_walls
}
