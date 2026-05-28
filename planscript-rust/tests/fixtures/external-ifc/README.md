# External IFC fixtures

These IFC files are regression fixtures from the buildingSMART `Sample-Test-Files`
repository:

- `basin-tessellation.ifc`
- `wall-with-opening-and-window.ifc`
- `column-straight-rectangle-tessellation.ifc`

Source: https://github.com/buildingSMART/Sample-Test-Files

License: Creative Commons Attribution 4.0 International, as provided in
`buildingsmart-LICENSE.txt`.

The files are intentionally small public examples that exercise common BIM import
cases: a sanitary terminal, an opening/window, and an imperial-unit structural
element.

`buildingsmart-open-manifest.json` is a PlanScript import manifest for these
fixtures and can be used with:

```bash
cargo run -- catalog import-manifest \
  tests/fixtures/external-ifc/buildingsmart-open-manifest.json \
  --out /tmp/planscript-open-ifc-catalog
```
