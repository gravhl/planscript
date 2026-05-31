# PlanScript Rust

Native Rust implementation of the PlanScript compiler, intent solver, catalog, validation, and exporters.

## Commands

```bash
cargo run -- compile ../examples/house.psc --svg /tmp/house.svg --json /tmp/house.json --dimensions
cargo run -- solve ../examples/simple-house.intent.json --out /tmp/simple-house.psc --svg /tmp/simple-house.svg
cargo run -- intent-schema
cargo run -- catalog list
cargo run -- catalog show builtin.sanitary.toilet.floor_mounted
cargo run -- catalog import-ifc ./toilet.ifc --id vendor.toilet.compact --category sanitary --out ./catalog
cargo run -- catalog import-manifest ./open-bim.json --out ./catalog/open
cargo run -- catalog lint ./catalog
cargo run -- catalog approve ./catalog/vendor.toilet.compact.psobj.json
```

## Fixtures and Objects

PlanScript Rust supports deterministic fixture/object placement through built-in and external catalogs:

```planscript
object wc1 {
  use builtin.sanitary.toilet.floor_mounted
  in bath
  attach west wall
  at 0.8
  facing east
}
```

Objects compile into SVG/JSON geometry and can be validated:

```planscript
assert objects_inside_rooms
assert object_no_overlap
assert object_clearances
```

Wall-backed fixture categories such as sanitary, kitchen, and laundry default to a wall placement when no `at` or `attach` is supplied. Explicit coordinate placement is still allowed, but if one of these fixtures is only slightly inset from the wall, compilation emits a layout warning and still generates output.

Built-in fixture IDs are legally clean generic objects created for PlanScript. They include BIM semantics such as IFC class and predefined type, but they do not depend on external BIM files.

Common US kitchen modules are available with inch-based IDs, including counter space widths (`builtin.kitchen.counter.size_36in`), refrigerators (`builtin.kitchen.fridge.standard_36in`, `builtin.kitchen.refrigerator.counter_depth_36in`), dishwashers (`builtin.kitchen.dishwasher.standard_24in`), cooktops/stove tops (`builtin.kitchen.cooktop.size_30in`, `builtin.kitchen.stovetop.size_30in`), and ranges (`builtin.kitchen.range.size_36in`).

External catalog items use `.psobj.json`:

```planscript
catalog "./catalog/residential"

plan "With Catalog" {
  footprint rect (0, 0) (5, 4)
  room bath { rect (0, 0) (5, 4) }
  object lav1 {
    use custom.sink.compact
    in bath
    at (1, 1)
  }
}
```

See [CATALOG.md](CATALOG.md) for the catalog format, BIM curation plan, and IFC import workflow.

## Floor Materials and Outdoor Areas

Indoor rooms and outdoor surfaces can carry semantic floor materials. The SVG exporter renders material patterns and an optional legend:

```planscript
render {
  mode draft
}

defaults {
  floor hardwood
  outdoor_floor pavers
}

plan "With Deck" {
  footprint rect (0, 0) (10, 8)
  legend { floor_materials auto }

  room living {
    rect (0, 0) (10, 8)
    floor tile
  }

  outdoor deck rear_deck {
    rect (0, 8) (10, 12)
    floor wood_deck
    label "Rear Deck"
  }
}
```

Supported materials include `hardwood`, `tile`, `carpet`, `concrete`, `polished_concrete`, `vinyl`, `stone`, `wood_deck`, `composite_deck`, `pavers`, `gravel`, `grass`, and `mulch`. Indoor/outdoor material mismatches produce non-fatal layout warnings.

Draft mode is optional and color remains the default. Use `render { mode draft }` in PlanScript or pass `--draft` to the CLI to render SVGs with black-and-white architectural hatches. The repository example builder emits paired color and draft SVGs for every `.psc` example.

## Dimension Controls

PlanScript can dimension every wall and fixture, or only selected room edges and object names:

```planscript
dimensions {
  walls bath.east kitchen.south
  fixtures wc lav range
}
```

Use `walls all`, `fixtures all`, or CLI shortcuts such as `--dimensions all`, `--wall-dimensions`, and `--fixture-dimensions` for broad annotation passes.

## Door Swing Conventions

Rust supports construction-style door handing on door openings:

```planscript
opening door d_entry {
  on foyer.edge south
  at 50%
  swing rhr
}

opening double door d_patio {
  between lounge and terrace
  on shared_edge
  at 50%
}
```

Supported swing values:

- `lh` - left hand
- `rh` - right hand
- `lhr` - left hand reverse
- `rhr` - right hand reverse

Single doors default to `door_width` from `defaults` or `0.9m`. Double doors default to two standard leaves, so their default total width is `2 * door_width`. Any door can override the total opening width with `width <value>`.

## Library

The crate exposes the main PlanScript pipeline:

- `parse`
- `lower`
- `generate_geometry`
- `validate`
- `export_svg`
- `export_json`
- `compile`
- `solve`

Run tests with:

```bash
cargo test
```
