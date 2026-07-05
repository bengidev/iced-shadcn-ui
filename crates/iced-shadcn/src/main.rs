mod cli;
mod config;
mod registry;
mod render;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let result: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Commands::Add { components } => {
            eprintln!("add not yet implemented: {components:?}");
            Ok(())
        }
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
