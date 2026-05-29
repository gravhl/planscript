units m

plan "Fixture Bathroom" {
  footprint rect (0, 0) (6.5, 4)

  room bath {
    rect (0, 0) (5, 4)
    label "Bath"
  }

  room hall {
    rect (5, 0) (6.5, 4)
    label "Hall"
  }

  object wc1 {
    use builtin.sanitary.toilet.floor_mounted
    in bath
    attach west wall
    at 0.8
    facing east
    label "WC"
  }

  object lav1 {
    use builtin.sanitary.sink.wall_hung
    in bath
    attach north wall
    at 1.0
    facing south
    label "Sink"
  }

  object shower1 {
    use builtin.sanitary.shower.size_900x900
    in bath
    attach south wall
    at 3.0
    facing north
    label "Shower"
  }

  opening door d_bath_hall {
    between bath and hall
    on shared_edge
    at 50%
    swing lh
  }

  assert objects_inside_rooms
  assert object_no_overlap
  assert object_clearances
}
