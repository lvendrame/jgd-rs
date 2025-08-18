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

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let generated = jgd_rs::generate_jgd_from_file(&cli.input);

    if let Err(error) = generated {
        eprintln!("{}", error);
        return Ok(());
    }

    let generated = generated.unwrap();
    let serialized = if cli.pretty {
        serde_json::to_string_pretty(&generated).unwrap()
    } else {
        serde_json::to_string(&generated).unwrap()
    };

    if let Some(path) = cli.out {
        let io_result = fs::write(path, serialized);
        if let Err(error) = io_result {
            println!("Error to record the file. Details: {}", error);
        }
    } else {
        println!("{}", serialized);
    }

    Ok(())
}
