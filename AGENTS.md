# AGENTS.md - Development Guide for AI Agents

This document provides context for AI agents working on the PlanScript codebase.

## Project Overview

**PlanScript** is a deterministic, textual DSL for defining 2D architectural floor plans. It compiles human-readable code into precise geometry outputs such as SVG and JSON.

PlanScript is now Rust-only. The canonical compiler, solver, catalog, validation, and exporters live in `planscript-rust/`.

Key design principles:
- **Deterministic**: Same input always produces the same output
- **Compiler-based**: Parse -> Lower -> Generate -> Validate -> Export
- **LLM-friendly**: Simple vocabulary, repetitive syntax, clear errors

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| Parser | Hand-written deterministic parser |
| Testing | Cargo test |
| Build | Cargo |
| Package | Cargo crate / Rust CLI |

## Project Structure

```
planscript/
├── planscript-rust/          # Canonical Rust implementation
│   ├── src/
│   │   ├── parser.rs         # Rust parser
│   │   ├── ast.rs            # AST type definitions
│   │   ├── lowering.rs       # AST -> LoweredProgram
│   │   ├── geometry.rs       # Geometry IR generation
│   │   ├── validation.rs     # Validation
│   │   ├── exporters.rs      # SVG/JSON export
│   │   ├── catalog.rs        # Object catalog and BIM import normalization
│   │   ├── compiler.rs       # Main compilation pipeline
│   │   ├── solver.rs         # Intent solver
│   │   └── main.rs           # CLI entry point
│   ├── examples/
│   ├── tests/
│   └── CATALOG.md
├── examples/                 # Top-level .psc and intent examples
├── scripts/                  # Rust-based example builders
├── LANGUAGE_REFERENCE.md     # Complete language documentation
├── INTENT_REFERENCE.md       # Intent JSON solver documentation
├── DESIGN.md                 # Design document and philosophy
└── README.md                 # User-facing documentation
```

## Rust Commands

```bash
cd planscript-rust

# Run tests
cargo test

# Compile PlanScript
cargo run -- compile ../examples/house.psc --svg /tmp/house.svg

# Compile fixture example
cargo run -- compile examples/fixture-bathroom.psc --svg /tmp/fixture.svg --json /tmp/fixture.json

# Generate PlanScript from intent JSON
cargo run -- solve ../examples/simple-house.intent.json --out /tmp/simple-house.psc --svg /tmp/simple-house.svg

# Catalog tools
cargo run -- catalog list
cargo run -- catalog show builtin.sanitary.toilet.floor_mounted
cargo run -- catalog import-ifc examples/ifc/minimal-toilet.ifc --id sample.toilet --category sanitary --out /tmp
cargo run -- catalog lint /tmp/sample.toilet.psobj.json
```

From the repository root:

```bash
./scripts/build-examples-psc.sh
./scripts/build-examples-intent.sh
cargo test --manifest-path planscript-rust/Cargo.toml
```

## Compilation Pipeline

```
Source (.psc)
    ↓ parse()             # planscript-rust/src/parser.rs
AST (Program)
    ↓ lower()             # planscript-rust/src/lowering.rs
LoweredProgram            # All geometry resolved to polygons
    ↓ generate_geometry() # planscript-rust/src/geometry.rs
GeometryIR                # Walls, openings, objects with coordinates
    ↓ validate()          # planscript-rust/src/validation.rs
Validation errors OR success
    ↓ export_svg() / export_json() # planscript-rust/src/exporters.rs
Output
```

## Key Files When Making Changes

### Adding New Syntax

1. `planscript-rust/src/parser.rs` - Add parser rules
2. `planscript-rust/src/ast.rs` - Add AST type definitions
3. `planscript-rust/src/lowering.rs` - Add lowering logic to resolve geometry
4. `planscript-rust/tests/*.rs` - Add parser/lowering/compiler tests
5. `LANGUAGE_REFERENCE.md` and `planscript-rust/README.md` - Document the new syntax

### Modifying Geometry Generation

- `planscript-rust/src/geometry.rs` - Wall/opening/object generation logic
- `planscript-rust/tests/core.rs` and `planscript-rust/tests/fixtures.rs` - Geometry tests

### Modifying Validation

- `planscript-rust/src/validation.rs` - Assertion checking, overlap detection, layout warnings
- `planscript-rust/tests/*.rs` - Validation tests

### Modifying Export

- `planscript-rust/src/exporters.rs` - SVG rendering and JSON serialization
- `planscript-rust/tests/*.rs` - Export tests

## Testing

Tests use Cargo.

```bash
cd planscript-rust
cargo test
cargo test --test fixtures
```

Test structure:

- **Parser tests**: Verify syntax parses correctly to AST
- **Lowering tests**: Verify AST resolves to polygons and object placements
- **Geometry tests**: Verify walls, openings, objects, and fixtures
- **Validation tests**: Verify assertions and warnings catch layout issues
- **Exporter tests**: Verify output formats

## CLI Usage

```bash
cd planscript-rust
cargo run -- compile <input.psc> [options]

# Options:
#   --svg <file>      Output SVG file
#   --json <file>     Output JSON file
#   --dimensions      Include dimension lines in SVG
#   --no-labels       Omit room labels in SVG

cargo run -- compile ../examples/house.psc --svg output.svg --dimensions
```

## Common Tasks

### Add a New Room Geometry Type

1. Add a variant to `RoomGeometry` in `planscript-rust/src/ast.rs`
2. Parse it in `planscript-rust/src/parser.rs`
3. Lower it in `planscript-rust/src/lowering.rs`
4. Add tests under `planscript-rust/tests/`
5. Update docs

### Add a New Directive

1. Add the type to `planscript-rust/src/ast.rs`
2. Add it to the relevant AST container
3. Parse it in `planscript-rust/src/parser.rs`
4. Handle it in lowering, geometry, validation, or export as appropriate
5. Add tests and documentation

## Language Reference

See `LANGUAGE_REFERENCE.md` for complete syntax documentation. Key constructs:

- **Zones**: Logical groupings of rooms that can be positioned as a unit
- **Rooms**: `rect`, `polygon`, `fill between`
- **Positioning**: `attach`, `align`, `gap`, `extend`
- **Openings**: `opening door`, `opening double door`, `opening window`
- **Objects**: Built-in and catalog-backed fixtures
- **Outdoor areas**: `outdoor deck`, `outdoor patio`, exterior surfaces
- **Floor materials**: `floor`, `outdoor_floor`, `legend { floor_materials ... }`
- **Rendering**: `render { mode color|draft }` for color or black-and-white SVG output
- **Assertions**: `assert no_overlap`, `assert inside footprint`, etc.

## Error Handling

The compiler produces typed errors:

- **Parse errors**: Invalid syntax from the Rust parser
- **Lowering errors**: Invalid references, missing directives, unresolved catalog items
- **Validation errors**: Assertion failures such as `E201` overlap or `E130` outside footprint

Layout warnings are returned in `CompileResult.warnings`. The CLI prints them to stdout while still generating requested output.

## Dependencies

Minimal dependencies by design:

- `serde` and `serde_json`: JSON serialization/deserialization
- Rust standard library for parser/compiler/runtime

## Solver Development Philosophy

The solver (`planscript-rust/src/solver.rs`) converts high-level intent JSON into valid PlanScript. When working on the solver, follow these principles.

### The Solver Adapts to Intents, Not Vice Versa

When the solver fails on a reasonable architectural intent, fix the solver or fail with a clear explanation. Do not modify intent files to work around solver limitations.

### When Intents Are Architecturally Impossible

Some intents are genuinely impossible due to architectural constraints. In these cases:

1. The solver should fail rather than produce invalid layouts
2. Error messages should explain why the intent is impossible
3. Suggested fixes are useful when they can be deterministic

Example of an impossible intent:
- West wing (6m wide, linear) with Kitchen, Shared Bath, Bedroom
- Shared bath needs circulation access, bedroom needs circulation access
- In a linear wing, only one can be adjacent to kitchen
- Bath cannot be accessed through bedroom, and bedroom cannot be accessed through bath

Correct solver behavior: fail with a message such as:

> Cannot place shared bath in west/back: requires circulation access but bedroom1 blocks the only path from kitchen. Consider adding a hall/corridor in the west wing.

### Debugging the Solver

Use `--inspect` to understand solver decisions:

```bash
cd planscript-rust
cargo run -- solve ../examples/simple-house.intent.json --inspect
```

This shows:
- Room ordering with priority breakdown
- Candidate placements and rejection reasons
- Door placement decisions
- Access/reachability analysis

Do not write throwaway debug scripts. If you need more visibility into solver behavior, improve `--inspect`.
