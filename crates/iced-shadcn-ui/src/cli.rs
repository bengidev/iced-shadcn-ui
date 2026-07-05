use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "iced-shadcn",
    about = "Add shadcn/ui-inspired components to your iced project",
    long_about = "Copy shadcn/ui-inspired iced 0.14 components into your project as Rust source.\n\n\
                  Install: cargo install iced-shadcn-ui\n\
                  Docs: https://docs.rs/iced-shadcn-ui"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add one or more components to the current project
    Add {
        /// Component names (e.g. button card)
        components: Vec<String>,
    },
    /// List available components from the remote registry
    List,
    /// Show diff for a component without writing files
    Diff {
        /// Component name
        component: String,
    },
}
