//! Thin JSON wrapper around `esplugin`, the Bethesda plugin parser that LOOT
//! itself uses.
//!
//! ModCheck does not reimplement record-level plugin parsing. This binary
//! exposes exactly what the compatibility analysis needs -- masters, flags,
//! record counts, override counts and the FormIDs a plugin touches -- as JSON
//! on stdout, and nothing else. It reads files and writes JSON; it never
//! writes to the plugin or anywhere else.
//!
//! esplugin is GPL-3.0. This wrapper is therefore GPL-3.0-or-later and is kept
//! as a separate optional binary, built on demand and not bundled.

use std::env;
use std::path::Path;
use std::process::ExitCode;

use esplugin::{GameId, ParseOptions, Plugin};
use serde_json::{json, Value};

fn game_id(name: &str) -> Option<GameId> {
    match name {
        "skyrimse" => Some(GameId::SkyrimSE),
        "skyrim" => Some(GameId::Skyrim),
        "fallout4" => Some(GameId::Fallout4),
        "falloutnv" => Some(GameId::FalloutNV),
        "fallout3" => Some(GameId::Fallout3),
        "oblivion" => Some(GameId::Oblivion),
        "morrowind" => Some(GameId::Morrowind),
        "starfield" => Some(GameId::Starfield),
        _ => None,
    }
}

fn describe(game: GameId, path: &Path, full: bool) -> Result<Value, String> {
    let mut plugin = Plugin::new(game, path);
    let options = if full {
        ParseOptions::whole_plugin()
    } else {
        ParseOptions::header_only()
    };
    plugin
        .parse_file(options)
        .map_err(|e| format!("{}: {:?}", path.display(), e))?;

    let mut out = json!({
        "path": path.display().to_string(),
        "filename": plugin.filename(),
        "masters": plugin.masters().map_err(|e| format!("{:?}", e))?,
        "is_master": plugin.is_master_file(),
        "is_light": plugin.is_light_plugin(),
        "is_valid_as_light": plugin.is_valid_as_light_plugin().unwrap_or(false),
        "header_version": plugin.header_version(),
        "record_and_group_count": plugin.record_and_group_count(),
        "parsed_fully": full,
    });

    if full {
        out["override_record_count"] = json!(plugin
            .count_override_records()
            .map_err(|e| format!("{:?}", e))?);
    }
    Ok(out)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "usage: modcheck-esplugin <game> [--full] <plugin>...\n\
             games: skyrimse skyrim fallout4 falloutnv fallout3 oblivion morrowind starfield\n\
             --full parses records (slower) and enables override counts and overlap"
        );
        return ExitCode::from(2);
    }

    let game = match game_id(&args[1]) {
        Some(g) => g,
        None => {
            eprintln!("unsupported game id: {}", args[1]);
            return ExitCode::from(2);
        }
    };

    let full = args.iter().any(|a| a == "--full");
    let paths: Vec<&String> = args[2..].iter().filter(|a| !a.starts_with("--")).collect();

    let mut plugins = Vec::new();
    let mut results = Vec::new();
    let mut failed = false;

    for raw in &paths {
        let path = Path::new(raw.as_str());
        match describe(game, path, full) {
            Ok(v) => {
                results.push(v);
                if full {
                    let mut p = Plugin::new(game, path);
                    if p.parse_file(ParseOptions::whole_plugin()).is_ok() {
                        plugins.push(p);
                    }
                }
            }
            Err(e) => {
                failed = true;
                results.push(json!({"path": raw, "error": e}));
            }
        }
    }

    // Record-level overlap between every pair, which is the signal a header
    // read cannot give.
    let mut overlaps = Vec::new();
    if full {
        for i in 0..plugins.len() {
            for j in (i + 1)..plugins.len() {
                if let Ok(true) = plugins[i].overlaps_with(&plugins[j]) {
                    overlaps.push(json!({
                        "a": plugins[i].filename(),
                        "b": plugins[j].filename(),
                    }));
                }
            }
        }
    }

    let output = json!({
        "tool": "esplugin",
        "tool_version": env!("CARGO_PKG_VERSION"),
        "game": args[1],
        "plugins": results,
        "overlaps": overlaps,
        "overlap_checked": full,
    });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
    if failed { ExitCode::from(1) } else { ExitCode::SUCCESS }
}
