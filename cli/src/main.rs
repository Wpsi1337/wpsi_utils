use clap::Parser;
use std::path::PathBuf;

/// Thin command-line interface for the toolbox scaffold.
#[derive(Debug, Parser)]
#[command(name = "toolbox", about = "Non-interactive toolbox control (placeholder)")]
struct Cli {
    /// Path to a toolbox config file.
    #[arg(long)]
    config: Option<PathBuf>,
    /// List available modules defined under /modules.
    #[arg(long)]
    list_modules: bool,
    /// Run a specific module by identifier.
    #[arg(long, value_name = "MODULE")]
    run: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("Not implemented – add your scripts in /modules.");
    if let Some(path) = cli.config {
        println!("Requested config: {}", path.display());
    }
    if cli.list_modules {
        println!("TODO: enumerate modules by reading module metadata.");
    }
    if let Some(module) = cli.run {
        println!("TODO: invoke module `{module}` with your own automation.");
    }
}
