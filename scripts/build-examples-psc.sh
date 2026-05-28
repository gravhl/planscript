#!/bin/bash
# Generate .svg from all .psc files in examples/ and planscript-rust/examples/.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
EXAMPLES_DIR="$PROJECT_DIR/examples"
RUST_EXAMPLES_DIR="$PROJECT_DIR/planscript-rust/examples"
RUST_MANIFEST="$PROJECT_DIR/planscript-rust/Cargo.toml"
CLI="$PROJECT_DIR/planscript-rust/target/debug/planscript-rust"
IFC_MANIFEST="$RUST_EXAMPLES_DIR/catalog/buildingsmart-open/buildingsmart-open-manifest.json"
IFC_CATALOG_DIR="$RUST_EXAMPLES_DIR/catalog/buildingsmart-open"

echo "Building PlanScript Rust CLI..."
cargo build --manifest-path "$RUST_MANIFEST"

if [ -f "$IFC_MANIFEST" ]; then
  echo "Refreshing example IFC catalog candidates..."
  "$CLI" catalog import-manifest "$IFC_MANIFEST" --out "$IFC_CATALOG_DIR"
fi

build_psc_dir() {
  local dir="$1"
  if [ ! -d "$dir" ]; then
    return
  fi
  echo "Building SVGs from $dir..."
  for psc_file in "$dir"/*.psc; do
    if [ -f "$psc_file" ]; then
      local base_name
      local svg_file
      base_name=$(basename "$psc_file" .psc)
      svg_file="$dir/$base_name.svg"

      echo "  $base_name.psc -> $base_name.svg"
      "$CLI" compile "$psc_file" --svg "$svg_file"
    fi
  done
}

build_psc_dir "$EXAMPLES_DIR"

if [ -f "$EXAMPLES_DIR/house.psc" ]; then
  echo "  house.psc -> house-with-dimensions.svg"
  "$CLI" compile "$EXAMPLES_DIR/house.psc" --svg "$EXAMPLES_DIR/house-with-dimensions.svg" --dimensions
fi

build_psc_dir "$RUST_EXAMPLES_DIR"

echo "Done!"
