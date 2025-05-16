use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod hash;
mod verify;
mod report;
mod benchmark;
mod serve;
mod audit;
mod exclude;

#[derive(Parser)]
#[command(name = "fips-hasher")]
#[command(about = "A FIPS-like file hashing tool written in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hash {
        #[arg(short, long)]
        path: PathBuf,
        #[arg(short, long)]
        key: Option<String>,
        #[arg(short, long, default_value_t = false)]
        incremental: bool,
        #[arg(short, long)]
        exclude: Vec<String>,
    },
    Verify {
        #[arg(short, long)]
        path: PathBuf,
        #[arg(short, long)]
        report: PathBuf,
    },
    Report {
        #[arg(short, long)]
        path: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    Benchmark,
    Serve {
        #[arg(short, long, default_value = "127.0.0.1:3000")]
        addr: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    audit::log_invocation();
    let cli = Cli::parse();

    match cli.command {
        Commands::Hash { path, key, exclude, incremental } => {
            let hashes = hash::hash_path(path, key.as_deref(), &exclude, incremental)?;
            for (file, hash) in hashes {
                println!("{file}: {hash}");
            }
        }
        Commands::Verify { path, report } => {
            let result = verify::verify_path(path, &report)?;
            if result.all_matched {
                println!("All files match the report.");
            } else {
                println!("Mismatch found!");
            }
        }
        Commands::Report { path, out } => {
            let hashes = hash::hash_path(path, None, &[], false)?;
            report::save_report(out, &hashes)?;
            println!("Report saved.");
        }
        Commands::Benchmark => benchmark::run(),
        Commands::Serve { addr } => serve::run_server(addr).await?,
    }

    Ok(())
}