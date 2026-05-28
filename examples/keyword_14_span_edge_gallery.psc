# Keyword coverage example 14: rect span x from one edge to another.

units m

plan "Span Edge Gallery" {
  footprint rect (0, 0) (18, 10)

  room studio_west {
    rect (0, 0) (6, 5)
    label "West Studio"
  }

  room studio_east {
    rect (12, 0) (18, 5)
    label "East Studio"
  }

  room gallery {
    rect span x from studio_west.left to studio_east.right y (5, 8)
    label "Gallery"
  }

  room storage {
    rect (6, 0) (12, 5)
    label "Storage"
  }

  opening door d_west_gallery { between studio_west and gallery on shared_edge at 50% width 0.9 }
  opening door d_east_gallery { between studio_east and gallery on shared_edge at 50% width 0.9 }
  opening door d_west_storage { between studio_west and storage on shared_edge at 50% width 0.9 }
  opening window w_gallery { on gallery.edge north at 8 width 3.0 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
