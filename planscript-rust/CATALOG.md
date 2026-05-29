# PlanScript Object Catalog

PlanScript Rust uses a normalized object catalog for fixtures, furniture, equipment, and future BIM-derived items.

Raw BIM files are source material, not the runtime compiler format. The compiler consumes compact `.psobj.json` items so plans remain deterministic, fast, and easy to author.

## Object Syntax

```planscript
catalog "./catalog/residential"

object wc1 {
  use builtin.sanitary.toilet.floor_mounted
  in bath
  attach west wall
  at 0.8
  facing east
  clearance front 0.8
}
```

Supported placement fields:

- `use <catalog.id>` references a built-in or external catalog item.
- `in <room>` places the object in a resolved room.
- `at (<x>, <y>)` places the object by world coordinates.
- `attach <north|south|east|west> wall` attaches the object anchor to a room edge.
- `at <distance>` or `at <percent>%` places along an attached wall.
- `facing <north|south|east|west>` controls orientation.
- `rotate <degrees>` adds a deterministic rotation.
- `mirror x` or `mirror y` mirrors local geometry.
- `clearance <front|back|left|right> <value>` overrides catalog clearance.

Sanitary, kitchen, laundry, plumbing, appliance, and fixture categories are treated as wall-backed by default. If no `at` or `attach` is supplied, the compiler places the fixture against the wall opposite its facing direction, centered on that wall. If explicit coordinates place a wall-backed fixture near, but not on, that wall, compilation returns a non-fatal layout warning.

## Built-In Catalog

Built-ins ship with the Rust compiler and are always available:

```bash
cargo run -- catalog list
cargo run -- catalog show builtin.sanitary.toilet.floor_mounted
```

Initial built-ins include:

- `builtin.sanitary.toilet.floor_mounted`
- `builtin.sanitary.sink.wall_hung`
- `builtin.sanitary.shower.size_900x900`
- `builtin.sanitary.tub.size_1700`
- `builtin.kitchen.sink`
- `builtin.kitchen.range.size_600`
- `builtin.kitchen.fridge.size_900`
- `builtin.kitchen.counter.size_<12|15|18|21|24|27|30|33|36|42|48>in`
- `builtin.kitchen.fridge.standard_<24|30|33|36>in`
- `builtin.kitchen.fridge.counter_depth_<30|33|36>in`
- `builtin.kitchen.refrigerator.standard_<24|30|33|36>in`
- `builtin.kitchen.refrigerator.counter_depth_<30|33|36>in`
- `builtin.kitchen.cooktop.size_<15|24|30|36|48>in`
- `builtin.kitchen.stovetop.size_<15|24|30|36|48>in`
- `builtin.kitchen.range.size_<24|30|36|48>in`
- `builtin.laundry.washer`
- `builtin.laundry.dryer`
- `builtin.furniture.bed.queen`
- `builtin.furniture.sofa.three_seat`
- `builtin.furniture.table.dining_6`

These items are generic PlanScript-owned assets. They include BIM alignment fields like `ifcClass` and `ifcPredefinedType`, but are not copied from manufacturer files.

US nominal kitchen sizes are stored in meters internally while preserving inch-based IDs for authoring. Counter modules use 25.5 in depth and 36 in height; refrigerator variants distinguish standard-depth and counter-depth footprints; cooktop and stovetop IDs are aliases for stove tops that sit in counter space. A cooktop/stovetop can be placed at the same wall station as a counter module; `object_no_overlap` treats it as an embedded insert when the cooktop footprint is fully inside the counter.

## `.psobj.json` Format

```json
{
  "id": "custom.sanitary.toilet.compact",
  "name": "Compact Toilet",
  "category": "sanitary",
  "source": {
    "format": "IFC",
    "provider": "manufacturer",
    "sourceUrl": "https://example.com/product",
    "license": "manufacturer terms",
    "redistributable": false
  },
  "bim": {
    "ifcClass": "IfcSanitaryTerminal",
    "ifcPredefinedType": "TOILETPAN"
  },
  "size": {
    "width": 0.38,
    "depth": 0.68,
    "height": 0.78
  },
  "footprint": [
    { "x": -0.19, "y": 0.0 },
    { "x": 0.19, "y": 0.0 },
    { "x": 0.19, "y": 0.68 },
    { "x": -0.19, "y": 0.68 }
  ],
  "clearances": {
    "front": 0.75,
    "left": 0.2,
    "right": 0.2
  },
  "anchors": {
    "wall": { "x": 0.0, "y": 0.0 },
    "waste": { "x": 0.0, "y": 0.22 },
    "cold_water": { "x": 0.15, "y": 0.25 }
  },
  "assets": {
    "svg": "symbols/toilet.svg",
    "glb": "models/toilet.glb",
    "ifc": "source/toilet.ifc"
  },
  "status": "approved",
  "needsReview": []
}
```

Coordinate convention:

- Local positive `y` is the object's front/facing direction.
- The wall anchor is usually `(0, 0)`.
- Footprints should be simple plan-view polygons.
- Clearances are local rectangles generated from `front`, `back`, `left`, and `right`.

## BIM and IFC Workflow

The long-term BIM strategy is curated, not raw-at-runtime:

```text
IFC/RFA/DXF/SVG/glTF source
  -> offline import and review
  -> .psobj.json candidate
  -> approved curated catalog item
  -> deterministic PlanScript object placement
```

The first import scaffold creates a review candidate:

```bash
cargo run -- catalog import-ifc ./toilet.ifc \
  --id vendor.toilet.compact \
  --category sanitary \
  --source-url https://example.com/toilet \
  --out ./catalog/vendor
```

For open BIM libraries or a curated source list, use a manifest so source,
license, and redistribution metadata are preserved consistently:

```json
{
  "items": [
    {
      "id": "open.buildingsmart.basin",
      "category": "sanitary",
      "file": "source/basin.ifc",
      "name": "buildingSMART Basin",
      "provider": "buildingSMART Sample-Test-Files",
      "sourceUrl": "https://github.com/buildingSMART/Sample-Test-Files",
      "license": "CC-BY-4.0",
      "redistributable": true
    }
  ]
}
```

Batch-import the manifest:

```bash
cargo run -- catalog import-manifest ./open-bim.json --out ./catalog/open
```

Manifest `file` paths are resolved relative to the manifest file. Generated
items are still candidates because dimensions, footprints, facing, anchors, and
clearances need review before curation.

Review and approve candidates with:

```bash
cargo run -- catalog lint ./catalog/vendor.toilet.compact.psobj.json
cargo run -- catalog approve ./catalog/vendor.toilet.compact.psobj.json
```

This command intentionally writes a candidate with `needsReview` fields. The current Rust-native importer reads IFC STEP text and attempts to extract:

- IFC class and predefined type
- units
- product name
- conservative bounding box from `IFCCARTESIANPOINTLIST2D`,
  `IFCCARTESIANPOINTLIST3D`, or `IFCCARTESIANPOINT`
- simple rectangular plan footprint

Because raw IFC representation geometry varies widely, imported items should still be reviewed before becoming curated catalog entries. A later IfcOpenShell-backed enrichment pass should add:

- manufacturer/model metadata
- true representation-derived top projection footprint
- optional SVG and GLB generated through `IfcConvert`
- anchors for plumbing, electrical, exhaust, or wall mounting

## Curation Tiers

PlanScript catalogs should distinguish redistributable assets from local imports:

- Tier A: PlanScript-owned built-ins, bundled with the compiler.
- Tier B: Open-license BIM-derived objects, redistributable when license allows.
- Tier C: Manufacturer BIM references, imported locally by users and not redistributed.

Every curated item should preserve `sourceUrl`, `license`, and `redistributable` metadata.

## Validation

Object assertions currently supported:

```planscript
assert objects_inside_rooms
assert object_no_overlap
assert object_clearances
```

Future validation should cover door swing/object conflicts, plumbing wall proximity, service clearance by category, and MEP anchor compatibility.
