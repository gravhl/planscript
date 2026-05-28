pub mod ast;
pub mod catalog;
pub mod compiler;
pub mod exporters;
pub mod geometry;
pub mod lowering;
pub mod parser;
pub mod solver;
pub mod validation;

pub use catalog::{
    builtin_items, candidate_ifc_item, candidate_ifc_item_from_file, candidate_ifc_item_from_spec,
    import_ifc_manifest, lint_catalog_item, Catalog, CatalogError, CatalogImportManifest,
    CatalogImportSpec, CatalogItem, CatalogLintIssue, CatalogLintSeverity,
};
pub use compiler::{compile, CompileError, CompileOptions, CompileResult};
pub use exporters::{export_json, export_svg, JsonExportOptions, SvgExportOptions};
pub use geometry::{generate_geometry, GeometryIr};
pub use lowering::{lower, LoweredProgram, LoweringError};
pub use parser::{parse, try_parse, ParseError};
pub use solver::{solve, validate_intent, LayoutIntent, SolveOptions, SolverResult};
pub use validation::{validate, ErrorCode, ValidationError};
