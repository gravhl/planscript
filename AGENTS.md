# AGENTS.md - Development Guide for AI Agents

This document provides context for AI agents (Claude, GPT, etc.) working on the PlanScript codebase.

## Project Overview

**PlanScript** is a deterministic, textual DSL for defining 2D architectural floor plans. It compiles human-readable code into precise geometry (SVG, JSON).

**Current product direction**: PlanScript is Rust-first. The canonical implementation is in `planscript-rust/`. The TypeScript implementation in `src/` is deprecated reference material; do not add new product functionality there unless explicitly asked.

Key design principles:
- **Deterministic**: Same input always produces same output
- **Compiler-based**: Parse → Lower → Generate → Validate → Export
- **LLM-friendly**: Simple vocabulary, repetitive syntax, clear errors

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| Parser | Hand-written deterministic parser |
| Testing | Cargo test |
| Build | Cargo |
| Package | Cargo crate / Rust CLI |

Deprecated reference implementation:
- `src/` TypeScript sources
- Peggy grammar and generated parser
- Vitest/npm scripts

## Project Structure

```
cado/
├── planscript-rust/          # CANONICAL Rust implementation
│   ├── src/
│   │   ├── parser.rs         # Rust parser
│   │   ├── ast.rs            # AST node type definitions
│   │   ├── lowering.rs       # AST → LoweredProgram
│   │   ├── geometry.rs       # Geometry IR generation
│   │   ├── validation.rs     # Validation
│   │   ├── exporters.rs      # SVG/JSON export
│   │   ├── catalog.rs        # Object catalog and BIM import normalization
│   │   ├── compiler.rs       # Main compilation pipeline
│   │   ├── solver.rs         # Intent solver
│   │   └── main.rs           # CLI entry point
│   ├── tests/
│   └── CATALOG.md
├── src/
│   └── ...                   # Deprecated TypeScript reference implementation
```

Deprecated TypeScript layout:

```
src/
│   ├── parser/
│   │   ├── grammar.pegjs     # PEG grammar definition (SOURCE OF TRUTH)
│   │   ├── grammar.ts        # Generated parser (DO NOT EDIT)
│   │   ├── index.ts          # Parser API (parse, tryParse)
│   │   └── parser.test.ts    # Parser tests
│   ├── ast/
│   │   └── types.ts          # AST node type definitions
│   ├── lowering/
│   │   ├── index.ts          # AST → LoweredProgram (polygon resolution)
│   │   └── lowering.test.ts  # Lowering tests
│   ├── geometry/
│   │   ├── index.ts          # Geometry IR generation (walls, openings)
│   │   ├── types.ts          # Geometry IR types
│   │   └── geometry.test.ts  # Geometry tests
│   ├── validation/
│   │   └── index.ts          # Validation (assertions, overlap detection)
│   ├── exporters/
│   │   ├── svg.ts            # SVG export
│   │   └── json.ts           # JSON export
│   ├── compiler.ts           # Main compilation pipeline
│   ├── cli.ts                # CLI entry point
│   └── index.ts              # Public API exports
├── examples/                 # Example .psc files
├── LANGUAGE_REFERENCE.md     # Complete language documentation
├── DESIGN.md                 # Design document and philosophy
├── README.md                 # User-facing documentation
└── package.json
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

# Catalog tools
cargo run -- catalog list
cargo run -- catalog show builtin.sanitary.toilet.floor_mounted
cargo run -- catalog import-ifc examples/ifc/minimal-toilet.ifc --id sample.toilet --category sanitary --out /tmp
cargo run -- catalog lint /tmp/sample.toilet.psobj.json
```

## Deprecated NPM Scripts

```bash
# Install dependencies
npm install

# Build the parser from grammar (REQUIRED after grammar changes)
npm run build:grammar

# Compile TypeScript
npm run build:ts

# Full build (grammar + TypeScript)
npm run build

# Development mode (rebuild grammar, then watch TypeScript)
npm run dev

# Run all tests
npm test

# Run tests in watch mode
npm run test:watch
```

## Compilation Pipeline

```
Source (.psc)
    ↓ parse()           # planscript-rust/src/parser.rs
AST (Program)
    ↓ lower()           # planscript-rust/src/lowering.rs
LoweredProgram          # All geometry resolved to polygons
    ↓ generate_geometry() # planscript-rust/src/geometry.rs
GeometryIR              # Walls, openings with coordinates
    ↓ validate()        # planscript-rust/src/validation.rs
Validation errors OR success
    ↓ export_svg() / export_json()  # planscript-rust/src/exporters.rs
Output
```

## Key Files When Making Changes

### Adding New Syntax

1. **`planscript-rust/src/parser.rs`** - Add parser rules
2. **`planscript-rust/src/ast.rs`** - Add AST type definitions
3. **`planscript-rust/src/lowering.rs`** - Add lowering logic to resolve geometry
4. **`planscript-rust/tests/*.rs`** - Add parser/lowering/compiler tests
5. **`LANGUAGE_REFERENCE.md`** and **`planscript-rust/README.md`** - Document the new syntax

### Modifying Geometry Generation

- **`planscript-rust/src/geometry.rs`** - Wall/opening/object generation logic
- **`planscript-rust/tests/core.rs`** and **`planscript-rust/tests/fixtures.rs`** - Geometry tests

### Modifying Validation

- **`planscript-rust/src/validation.rs`** - Assertion checking, overlap detection
- **`planscript-rust/tests/*.rs`** - Validation tests

### Modifying Export

- **`planscript-rust/src/exporters.rs`** - SVG rendering and JSON serialization
- **`planscript-rust/tests/*.rs`** - Export tests

## Testing

Tests use Cargo.

```bash
cd planscript-rust
cargo test
cargo test --test fixtures
```

### Test Structure

- **Parser tests**: Verify syntax is parsed correctly to AST
- **Lowering tests**: Verify AST is correctly resolved to polygons
- **Geometry tests**: Verify walls and openings are generated correctly
- **Validation tests**: Verify assertions catch errors
- **Exporter tests**: Verify output formats

## CLI Usage

```bash
cd planscript-rust
cargo run -- compile <input.psc> [options]

# Options:
#   --svg <file>      Output SVG file
#   --json <file>     Output JSON file
#   --dimensions      Include dimension lines in SVG

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

1. Add type to `planscript-rust/src/ast.rs`
2. Add to `RoomDefinition` interface if room-level
3. Parse it in `planscript-rust/src/parser.rs`
4. Handle it in lowering/geometry/validation as appropriate
5. Use in lowering logic
6. Add tests and documentation

## Language Reference

See `LANGUAGE_REFERENCE.md` for complete syntax documentation. Key constructs:

- **Zones**: Logical groupings of rooms that can be positioned as a unit
- **Rooms**: `rect`, `polygon`, `fill between`
- **Positioning**: `attach`, `align`, `gap`, `extend`
- **Openings**: `opening door`, `opening window`
- **Assertions**: `assert no_overlap`, `assert inside footprint`, etc.

## Error Handling

The compiler produces typed errors:

- **Parse errors**: Invalid syntax (from Peggy)
- **Lowering errors**: Invalid references, missing directives
- **Validation errors**: Assertion failures (E201 overlap, E130 outside footprint, etc.)

Errors include source locations when available.

## Dependencies

Minimal dependencies by design:

- **serde** and **serde_json**: JSON serialization/deserialization
- Rust standard library for parser/compiler/runtime

No runtime dependencies for the core library.

## Solver Development Philosophy

The solver (`planscript-rust/src/solver.rs`) converts high-level intent JSON into valid PlanScript. When working on the solver, follow these principles:

### The Solver Adapts to Intents, Not Vice Versa

**CRITICAL**: When the solver fails on a reasonable architectural intent, the problem is in the solver, not the intent. Do NOT modify intent files to work around solver limitations.

**Bad approach**:
```
Solver fails on shared bathroom → Change intent to make it an ensuite
```

**Good approach**:
```
Solver fails on shared bathroom → Fix solver to handle shared bathrooms
                                → OR fail with clear explanation if architecturally impossible
```

### When Intents Are Architecturally Impossible

Some intents are genuinely impossible due to architectural constraints. In these cases:

1. **The solver should FAIL**, not produce invalid layouts
2. **Error messages should explain WHY** it's impossible
3. **Suggest fixes** when possible

Example of an impossible intent:
- West wing (6m wide, linear) with: Kitchen (circulation), Shared Bath, Bedroom
- Shared bath needs circulation access, bedroom needs circulation access
- In a linear wing, only one can be adjacent to kitchen
- Bath can't be accessed through bedroom (not circulation)
- Bedroom can't be accessed through bath (architectural rule)

**Correct solver behavior**: Fail with message like:
> "Cannot place shared bath in west/back: requires circulation access but bedroom1 blocks the only path from kitchen. Consider adding a hall/corridor in the west wing."

### Debugging the Solver

Use `--inspect` flag to understand solver decisions:

```bash
node dist/cli.js input.intent.json --inspect
```

This shows:
- Room ordering with priority breakdown
- Candidate placements and rejection reasons
- Door placement decisions
- Access/reachability analysis

**Do NOT write throwaway debug scripts** - if you need visibility into solver behavior, use `--inspect` or improve its output.
