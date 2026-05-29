# Keyword coverage example 15: explicit align my edge with room edge.

units m

plan "Explicit Alignment Edges" {
  footprint rect (0, 0) (16, 12)

  room base {
    rect (4, 4) (10, 8)
    label "Base"
  }

  room porch {
    rect size (6, 2)
    attach south_of base
    align my right with base.right
    gap 0
    label "Porch"
  }

  room loft {
    rect size (4, 2)
    attach north_of base
    align my left with base.left
    gap 0
    label "Loft"
  }

  room side {
    rect size (3, 4)
    attach west_of base
    align my top with base.top
    gap 0
    label "Side"
  }

  opening door d_base_porch { between base and porch on shared_edge at 50% width 0.9 swing lh }
  opening door d_base_loft { between base and loft on shared_edge at 50% width 0.9 swing rh }
  opening door d_base_side { between base and side on shared_edge at 50% width 0.9 swing lhr }
  opening window w_side { on side.edge west at 50% width 1.2 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert rooms_connected
}
