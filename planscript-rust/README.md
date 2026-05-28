# PlanScript Rust

Native Rust implementation of the PlanScript compiler and intent solver.

## Commands

```bash
cargo run -- compile ../examples/house.psc --svg /tmp/house.svg --json /tmp/house.json --dimensions
cargo run -- solve ../examples/simple-house.intent.json --out /tmp/simple-house.psc --svg /tmp/simple-house.svg
cargo run -- intent-schema
```

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

