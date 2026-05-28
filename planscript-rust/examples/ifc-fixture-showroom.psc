catalog "./catalog/buildingsmart-open"

units m

plan "IFC Fixture Showroom" {
  footprint rect (0, 0) (10, 6)

  room gallery {
    rect (0, 0) (10, 6)
    label "Fixture Showroom"
  }

  object ifc_basin1 {
    use open.buildingsmart.basin
    in gallery
    at (1.0, 0.4)
    facing north
    label "IFC Basin"
  }

  object ifc_window1 {
    use open.buildingsmart.window
    in gallery
    attach north wall
    at 5.0
    facing south
    label "IFC Window"
  }

  object ifc_column1 {
    use open.buildingsmart.column
    in gallery
    at (8.5, 0.5)
    facing north
    label "IFC Column"
  }

  object builtin_sink1 {
    use builtin.sanitary.sink.wall_hung
    in gallery
    attach west wall
    at 3.0
    facing east
    label "Built-In Sink"
  }

  object builtin_sofa1 {
    use builtin.furniture.sofa.three_seat
    in gallery
    attach south wall
    at 5.0
    facing north
    label "Built-In Sofa"
  }

  assert objects_inside_rooms
  assert object_no_overlap
  assert object_clearances
}
