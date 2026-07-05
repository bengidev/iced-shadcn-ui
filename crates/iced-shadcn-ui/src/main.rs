//! # iced-shadcn-ui
//!
//! CLI for adding [shadcn/ui](https://ui.shadcn.com)-inspired components to
//! [iced](https://github.com/iced-rs/iced) 0.14 projects. Components are copied as
//! self-contained Rust source into your project — you own the files.
//!
//! Install with `cargo install iced-shadcn-ui`; the binary is `iced-shadcn`.
//!
//! See the [crate README](https://docs.rs/iced-shadcn-ui) for usage, configuration,
//! and links to the full documentation on GitHub.

mod add;
mod cargo_merge;
mod cli;
mod config;
mod diff;
mod error;
mod fetch;
mod list;
mod patch;
mod registry;
mod registry_load;
mod render;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let result: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Commands::Add { components } => add::run(components).map_err(Into::into),
        Commands::List => list::run().map_err(Into::into),
        Commands::Diff { component } => diff::run(component).map_err(Into::into),
    };
    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
