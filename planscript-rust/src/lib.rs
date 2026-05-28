pub mod ast;
pub mod compiler;
pub mod exporters;
pub mod geometry;
pub mod lowering;
pub mod parser;
pub mod solver;
pub mod validation;

pub use compiler::{compile, CompileError, CompileOptions, CompileResult};
pub use exporters::{export_json, export_svg, JsonExportOptions, SvgExportOptions};
pub use geometry::{generate_geometry, GeometryIr};
pub use lowering::{lower, LoweredProgram, LoweringError};
pub use parser::{parse, try_parse, ParseError};
pub use solver::{solve, validate_intent, LayoutIntent, SolveOptions, SolverResult};
pub use validation::{validate, ErrorCode, ValidationError};
