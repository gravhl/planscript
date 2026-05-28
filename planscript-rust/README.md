# PlanScript Rust

Native Rust implementation of the PlanScript compiler and intent solver.

The Rust crate is the forward path for PlanScript. The earlier TypeScript implementation is deprecated.

## Commands

```bash
cargo run -- compile ../examples/house.psc --svg /tmp/house.svg --json /tmp/house.json --dimensions
cargo run -- solve ../examples/simple-house.intent.json --out /tmp/simple-house.psc --svg /tmp/simple-house.svg
cargo run -- intent-schema
cargo run -- catalog list
cargo run -- catalog show builtin.sanitary.toilet.floor_mounted
cargo run -- catalog import-ifc ./toilet.ifc --id vendor.toilet.compact --category sanitary --out ./catalog
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

Built-in fixture IDs are legally clean generic objects created for PlanScript. They include BIM semantics such as IFC class and predefined type, but they do not depend on external BIM files.

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

## Library

The crate exposes the same main pipeline shape as the TypeScript implementation:

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
