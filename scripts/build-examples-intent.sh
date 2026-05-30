#!/bin/bash
# Generate .psc and .svg from all .intent.json files in examples/

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
EXAMPLES_DIR="$PROJECT_DIR/examples"
RUST_MANIFEST="$PROJECT_DIR/planscript-rust/Cargo.toml"
CLI="$PROJECT_DIR/planscript-rust/target/debug/planscript-rust"

echo "Building examples from intent files..."
cargo build --manifest-path "$RUST_MANIFEST"

for intent_file in "$EXAMPLES_DIR"/*.intent.json; do
  if [ -f "$intent_file" ]; then
    base_name=$(basename "$intent_file" .intent.json)
    psc_file="$EXAMPLES_DIR/$base_name.psc"
    svg_file="$EXAMPLES_DIR/$base_name.svg"
    
    echo "  $base_name.intent.json -> $base_name.psc, $base_name.svg"
    "$CLI" solve "$intent_file" --out "$psc_file" --svg "$svg_file"
  fi
done

echo "Done!"
