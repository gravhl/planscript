use crate::ast::Program;
use crate::catalog::{resolve_catalog_path, Catalog};
use crate::exporters::{export_json, export_svg, JsonExportOptions, SvgExportOptions};
use crate::geometry::{generate_geometry, GeometryIr};
use crate::lowering::lower_with_catalog;
use crate::parser::try_parse;
use crate::warnings::layout_warnings;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompilePhase {
    Parse,
    Lower,
    Validate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompileError {
    pub phase: CompilePhase,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SourceLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileOptions {
    pub emit_svg: Option<bool>,
    pub emit_json: Option<bool>,
    pub svg_options: Option<SvgExportOptions>,
    pub json_options: Option<JsonExportOptions>,
    pub catalog_base_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompileResult {
    pub success: bool,
    pub errors: Vec<CompileError>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<Program>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeometryIr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,
}

pub fn compile(source: &str, options: CompileOptions) -> CompileResult {
    let ast = match try_parse(source) {
        Ok(program) => program,
        Err(error) => {
            return CompileResult {
                success: false,
                errors: vec![CompileError {
                    phase: CompilePhase::Parse,
                    message: error.message,
                    location: Some(SourceLocation {
                        line: error.line,
                        column: error.column,
                    }),
                    code: None,
                    details: None,
                }],
                warnings: Vec::new(),
                ast: None,
                geometry: None,
                svg: None,
                json: None,
            }
        }
    };

    let mut catalog = Catalog::builtins();
    let base_dir = options.catalog_base_dir.as_deref().map(Path::new);
    for declaration in &ast.catalogs {
        let path = resolve_catalog_path(&declaration.path, base_dir);
        if let Err(error) = catalog.load_path(&path) {
            return CompileResult {
                success: false,
                errors: vec![CompileError {
                    phase: CompilePhase::Lower,
                    message: error.message,
                    location: None,
                    code: None,
                    details: None,
                }],
                warnings: Vec::new(),
                ast: Some(ast),
                geometry: None,
                svg: None,
                json: None,
            };
        }
    }

    let lowered = match lower_with_catalog(&ast, &catalog) {
        Ok(lowered) => lowered,
        Err(error) => {
            let mut details = None;
            if let Some(room) = error.room_name {
                let mut map = HashMap::new();
                map.insert("room".to_string(), Value::String(room));
                details = Some(map);
            }
            return CompileResult {
                success: false,
                errors: vec![CompileError {
                    phase: CompilePhase::Lower,
                    message: error.message,
                    location: None,
                    code: None,
                    details,
                }],
                warnings: Vec::new(),
                ast: Some(ast),
                geometry: None,
                svg: None,
                json: None,
            };
        }
    };

    let geometry = generate_geometry(&lowered);
    let warnings = layout_warnings(&geometry);
    let validation_errors = crate::validation::validate(&lowered, &geometry);
    if !validation_errors.is_empty() {
        return CompileResult {
            success: false,
            errors: validation_errors
                .into_iter()
                .map(|error| CompileError {
                    phase: CompilePhase::Validate,
                    message: error.message,
                    location: None,
                    code: Some(error.code),
                    details: error.details,
                })
                .collect(),
            warnings,
            ast: Some(ast),
            geometry: Some(geometry),
            svg: None,
            json: None,
        };
    }

    let emit_svg = options.emit_svg.unwrap_or(true);
    let emit_json = options.emit_json.unwrap_or(false);
    let svg = emit_svg.then(|| {
        export_svg(
            &geometry,
            options.svg_options.clone().unwrap_or_default(),
            lowered.site,
        )
    });
    let json = if emit_json {
        Some(
            export_json(
                &geometry,
                options.json_options.clone().unwrap_or_default(),
                Some(&ast),
            )
            .unwrap_or_else(|error| {
                serde_json::json!({
                    "version": "1.0.0",
                    "error": error.to_string()
                })
                .to_string()
            }),
        )
    } else {
        None
    };

    CompileResult {
        success: true,
        errors: Vec::new(),
        warnings,
        ast: Some(ast),
        geometry: Some(geometry),
        svg,
        json,
    }
}
