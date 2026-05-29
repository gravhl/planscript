# Keyword coverage example 20: comment forms and compact inline syntax.
// This line uses slash comments.
/*
  This block comment verifies block comment examples stay covered.
*/

units m

plan "Comments And Compact Syntax" {
  footprint rect (0, 0) (10, 6)

  room living { rect (0, 0) (6, 6) label "Living" }
  room kitchen { rect (6, 0) (10, 3) label "Kitchen" }
  room bath { rect (6, 3) (10, 6) label "Bath" }

  opening door d_living_kitchen { between living and kitchen on shared_edge at 50% width 0.9 swing lh }
  opening door d_living_bath { between living and bath on shared_edge at 50% width 0.8 swing rh }
  opening window w_living { on living.edge west at 50% width 1.5 sill 0.9 }

  assert no_overlap rooms
  assert inside footprint all_rooms
  assert min_room_area living >= 30
  assert rooms_connected
}
