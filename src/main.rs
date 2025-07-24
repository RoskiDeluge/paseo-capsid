use clap::Parser;
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// CLI args
#[derive(Parser, Debug)]
#[command(author, version, about = "Paseo Capsid — pod manifest inspector")]
struct Args {
    /// Path to pod directory (where paseo.json lives)
    pod_dir: PathBuf,
}

/// Minimal manifest type (matches JSON spec in the canvas)
#[derive(Deserialize, JsonSchema, Debug)]
struct Manifest {
    entity: String,
    memory: String,
    schema: String,
    // ignore the rest for now
    #[serde(flatten)]
    _extra: serde_json::Value,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let manifest_path = args.pod_dir.join("paseo.json");

    let raw = fs::read_to_string(&manifest_path)
        .map_err(|e| anyhow::anyhow!("failed to read {:?}: {}", manifest_path, e))?;

    let manifest: Manifest = serde_json::from_str(&raw)
        .map_err(|e| anyhow::anyhow!("manifest is not valid JSON: {}", e))?;

    println!("✅ loaded pod '{}'", manifest.entity);
    println!("   • memory → {}", manifest.memory);
    println!("   • schema → {}", manifest.schema);

    Ok(())
}
