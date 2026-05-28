use planscript::catalog::{candidate_ifc_item, Catalog};
use planscript::compiler::{compile, CompileOptions, CompilePhase};
use planscript::exporters::{JsonExportOptions, SvgExportOptions};
use planscript::solver::{
    get_layout_intent_json_schema_string, parse_intent, solve, validate_intent, SolveOptions,
    SolverResult,
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        show_help();
        return;
    }

    let command = args[0].as_str();
    let result = if command == "compile" {
        run_compile(&args[1..])
    } else if command == "solve" {
        run_solve(&args[1..])
    } else if command == "catalog" {
        run_catalog(&args[1..])
    } else if command == "intent-schema" {
        run_intent_schema(&args[1..])
    } else if command.ends_with(".json") {
        run_solve(&args)
    } else {
        run_compile(&args)
    };

    if let Err(message) = result {
        eprintln!("{message}");
        process::exit(1);
    }
}

fn show_help() {
    println!(
        r#"PlanScript Rust - A DSL for defining floor plans

Commands:
  planscript-rust compile <input.psc> [options]   Compile PlanScript to SVG/JSON
  planscript-rust solve <intent.json> [options]   Generate PlanScript from intent
  planscript-rust catalog <subcommand> [options]  Inspect or create object catalog items
  planscript-rust intent-schema [options]         Output JSON Schema for intent format

Compile Options:
  --svg <output.svg>   Write SVG output to file
  --json <output.json> Write JSON output to file
  --dimensions         Include dimension lines in SVG
  --no-labels          Don't show room labels in SVG
  --no-svg             Don't generate SVG

Solve Options:
  --out <output.psc>   Write generated PlanScript to file
  --svg <output.svg>   Also compile and write SVG
  --inspect            Show solver inspection summary
  --variants <n>       Accepted for CLI compatibility

Intent Schema Options:
  --out <file.json>    Write schema to file (default: stdout)

Catalog Subcommands:
  catalog list                              List built-in object IDs
  catalog show <id>                         Print a built-in object as .psobj.json
  catalog import-ifc <file.ifc> --id <id> --category <name> [--out <path>]
"#
    );
}

fn run_intent_schema(args: &[String]) -> Result<(), String> {
    let mut out = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--out" && i + 1 < args.len() {
            out = Some(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    let schema = get_layout_intent_json_schema_string();
    if let Some(path) = out {
        fs::write(&path, schema).map_err(|e| format!("{path}: error: {e}"))?;
        println!("JSON Schema written to: {path}");
    } else {
        println!("{schema}");
    }
    Ok(())
}

fn run_compile(args: &[String]) -> Result<(), String> {
    if args.is_empty() || args[0].starts_with('-') {
        return Err(
            "Error: No input file specified\nUsage: planscript-rust compile <input.psc> [options]"
                .to_string(),
        );
    }
    let input = &args[0];
    let absolute = absolute_path(input);
    let mut svg_out = None;
    let mut json_out = None;
    let mut emit_svg = true;
    let mut emit_json = false;
    let mut show_dimensions = false;
    let mut show_labels = true;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--svg" if i + 1 < args.len() => {
                svg_out = Some(args[i + 1].clone());
                i += 2;
            }
            "--json" if i + 1 < args.len() => {
                json_out = Some(args[i + 1].clone());
                emit_json = true;
                i += 2;
            }
            "--no-svg" => {
                emit_svg = false;
                i += 1;
            }
            "--dimensions" => {
                show_dimensions = true;
                i += 1;
            }
            "--no-labels" => {
                show_labels = false;
                i += 1;
            }
            _ => i += 1,
        }
    }

    let source =
        fs::read_to_string(input).map_err(|e| format!("{}: error: {e}", absolute.display()))?;
    let result = compile(
        &source,
        CompileOptions {
            emit_svg: Some(emit_svg),
            emit_json: Some(emit_json),
            svg_options: Some(SvgExportOptions {
                show_dimensions: Some(show_dimensions),
                show_labels: Some(show_labels),
                ..Default::default()
            }),
            json_options: Some(JsonExportOptions {
                pretty: Some(true),
                include_ast: Some(false),
            }),
            catalog_base_dir: absolute.parent().map(|p| p.to_string_lossy().to_string()),
        },
    );

    if !result.success {
        eprintln!();
        for error in &result.errors {
            let phase = match error.phase {
                CompilePhase::Parse => "parse",
                CompilePhase::Lower => "lower",
                CompilePhase::Validate => "validate",
            };
            if let Some(location) = &error.location {
                eprintln!(
                    "{}:{}:{}: error: {}",
                    absolute.display(),
                    location.line,
                    location.column,
                    error.message
                );
            } else if let Some(code) = &error.code {
                eprintln!("{}: error[{}]: {}", absolute.display(), code, error.message);
            } else {
                eprintln!("{}: error[{phase}]: {}", absolute.display(), error.message);
            }
        }
        eprintln!(
            "\nCompilation failed with {} error(s).",
            result.errors.len()
        );
        return Err(String::new());
    }

    println!("Compilation successful!");
    if let (Some(path), Some(svg)) = (svg_out, result.svg.as_ref()) {
        fs::write(&path, svg).map_err(|e| format!("{path}: error: {e}"))?;
        println!("  SVG written to: {path}");
    } else if let Some(svg) = result.svg.as_ref() {
        println!("  SVG: {} bytes generated", svg.len());
    }
    if let (Some(path), Some(json)) = (json_out, result.json.as_ref()) {
        fs::write(&path, json).map_err(|e| format!("{path}: error: {e}"))?;
        println!("  JSON written to: {path}");
    }
    if let Some(geometry) = &result.geometry {
        println!("  Rooms: {}", geometry.rooms.len());
        println!("  Objects: {}", geometry.objects.len());
        println!("  Walls: {}", geometry.walls.len());
        println!("  Openings: {}", geometry.openings.len());
    }
    Ok(())
}

fn run_catalog(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err(
            "Error: No catalog subcommand specified\nUsage: planscript-rust catalog <list|show|import-ifc>"
                .to_string(),
        );
    }

    match args[0].as_str() {
        "list" => {
            let catalog = Catalog::builtins();
            for id in catalog.ids() {
                println!("{id}");
            }
            Ok(())
        }
        "show" => {
            if args.len() < 2 {
                return Err("Usage: planscript-rust catalog show <id>".to_string());
            }
            let catalog = Catalog::builtins();
            let item = catalog
                .get(&args[1])
                .ok_or_else(|| format!("Unknown built-in catalog item: {}", args[1]))?;
            let json = serde_json::to_string_pretty(item)
                .map_err(|e| format!("Failed to serialize catalog item: {e}"))?;
            println!("{json}");
            Ok(())
        }
        "import-ifc" => run_catalog_import_ifc(&args[1..]),
        other => Err(format!("Unknown catalog subcommand: {other}")),
    }
}

fn run_catalog_import_ifc(args: &[String]) -> Result<(), String> {
    if args.is_empty() || args[0].starts_with('-') {
        return Err(
            "Usage: planscript-rust catalog import-ifc <file.ifc> --id <id> --category <name> [--out <path>] [--source-url <url>]"
                .to_string(),
        );
    }
    let ifc_path = args[0].clone();
    let mut id = None;
    let mut category = None;
    let mut out = None;
    let mut source_url = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--id" if i + 1 < args.len() => {
                id = Some(args[i + 1].clone());
                i += 2;
            }
            "--category" if i + 1 < args.len() => {
                category = Some(args[i + 1].clone());
                i += 2;
            }
            "--out" if i + 1 < args.len() => {
                out = Some(args[i + 1].clone());
                i += 2;
            }
            "--source-url" if i + 1 < args.len() => {
                source_url = Some(args[i + 1].clone());
                i += 2;
            }
            _ => i += 1,
        }
    }

    let id = id.ok_or_else(|| "--id is required".to_string())?;
    let category = category.ok_or_else(|| "--category is required".to_string())?;
    let item = candidate_ifc_item(id.clone(), category, ifc_path, source_url);
    let json = serde_json::to_string_pretty(&item)
        .map_err(|e| format!("Failed to serialize candidate catalog item: {e}"))?;

    if let Some(out) = out {
        let mut path = PathBuf::from(out);
        if path.is_dir() {
            path = path.join(format!("{id}.psobj.json"));
        }
        fs::write(&path, json).map_err(|e| format!("{}: error: {e}", path.display()))?;
        println!("Catalog candidate written to: {}", path.display());
    } else {
        println!("{json}");
    }
    Ok(())
}

fn run_solve(args: &[String]) -> Result<(), String> {
    if args.is_empty() || args[0].starts_with('-') {
        return Err(
            "Error: No input file specified\nUsage: planscript-rust solve <intent.json> [options]"
                .to_string(),
        );
    }
    let input = &args[0];
    let absolute = absolute_path(input);
    let mut psc_out = None;
    let mut svg_out = None;
    let mut inspect = false;
    let mut variants = 1usize;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" if i + 1 < args.len() => {
                psc_out = Some(args[i + 1].clone());
                i += 2;
            }
            "--svg" if i + 1 < args.len() => {
                svg_out = Some(args[i + 1].clone());
                i += 2;
            }
            "--inspect" => {
                inspect = true;
                i += 1;
            }
            "--variants" if i + 1 < args.len() => {
                variants = args[i + 1].parse().unwrap_or(1);
                i += 2;
            }
            _ => i += 1,
        }
    }

    let json_source =
        fs::read_to_string(input).map_err(|e| format!("{}: error: {e}", absolute.display()))?;
    let intent = parse_intent(&json_source)
        .map_err(|e| format!("{}: error: Invalid intent format: {e}", absolute.display()))?;
    let validation = validate_intent(&intent);
    if !validation.is_empty() {
        let mut message = String::from("Intent validation errors:\n");
        for err in validation {
            message.push_str(&format!("  - {err}\n"));
        }
        return Err(message);
    }

    println!("Solving floor plan...");
    let result = solve(
        intent,
        SolveOptions {
            inspect: Some(inspect),
            variants: Some(variants),
            ..Default::default()
        },
    );

    match result {
        SolverResult::Success {
            plan_script,
            state,
            score,
            inspect_trace,
            ..
        } => {
            if inspect {
                if let Some(trace) = inspect_trace {
                    println!("\n{trace}\n");
                }
            }
            println!("Solve successful!");
            println!("  Score: {:.2}", score.total);
            println!("  Rooms placed: {}", state.placed.len());
            println!("  Openings: {}", state.openings.len());

            if let Some(path) = psc_out {
                fs::write(&path, &plan_script).map_err(|e| format!("{path}: error: {e}"))?;
                println!("  PlanScript written to: {path}");
            } else {
                println!("\nGenerated PlanScript:\n---\n{plan_script}---");
            }

            if let Some(path) = svg_out {
                println!("\nCompiling to SVG...");
                let compiled = compile(
                    &plan_script,
                    CompileOptions {
                        emit_svg: Some(true),
                        svg_options: Some(SvgExportOptions {
                            show_labels: Some(true),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                );
                if compiled.success {
                    if let Some(svg) = compiled.svg {
                        fs::write(&path, svg).map_err(|e| format!("{path}: error: {e}"))?;
                        println!("  SVG written to: {path}");
                    }
                } else {
                    return Err(format!(
                        "Failed to compile generated PlanScript: {}",
                        compiled
                            .errors
                            .first()
                            .map(|e| e.message.clone())
                            .unwrap_or_else(|| "unknown error".to_string())
                    ));
                }
            }
            Ok(())
        }
        SolverResult::Failure {
            error,
            violations,
            inspect_trace,
            ..
        } => {
            if inspect {
                if let Some(trace) = inspect_trace {
                    eprintln!("\n{trace}\n");
                }
            }
            eprintln!("\nSolve failed: {error}");
            if let Some(violations) = violations {
                for violation in violations {
                    eprintln!("  - {violation}");
                }
            }
            Err(String::new())
        }
    }
}

fn absolute_path(path: &str) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path))
}
