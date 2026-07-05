mod add;
mod cargo_merge;
mod cli;
mod config;
mod error;
mod fetch;
mod patch;
mod registry;
mod render;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let result: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Commands::Add { components } => add::run(components).map_err(Into::into),
        Commands::List => {
            eprintln!("list not yet implemented");
            Ok(())
        }
        Commands::Diff { component } => {
            eprintln!("diff not yet implemented: {component}");
            Ok(())
        }
    };
    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
