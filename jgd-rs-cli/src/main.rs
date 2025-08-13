use anyhow::Result;
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about = "Generate JSON from .jgd definitions")]
struct Cli {
    /// Path to .jgd file
    input: PathBuf,
    /// Output file (JSON). If omitted, prints to stdout.
    #[arg(short, long)]
    out: Option<PathBuf>,
    /// Seed override
    #[arg(long)]
    seed: Option<u64>,
    /// Pretty print
    #[arg(short, long)]
    pretty: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let generated = jgd_rs::generate_jgd_from_file(&cli.input);

    let serialized = if cli.pretty {
        serde_json::to_string_pretty(&generated).unwrap()
    } else {
        serde_json::to_string(&generated).unwrap()
    };

    if let Some(path) = cli.out {
        fs::write(path, serialized)?;
    } else {
        println!("{}", serialized);
    }

    Ok(())
}

// fn fingerprint(obj: &BTreeMap<String, Value>, keys: &[String]) -> String {
//     let mut parts = Vec::new();
//     for k in keys { if let Some(v) = obj.get(k) { parts.push(v.to_string()); } }
//     parts.join("|")
// }

