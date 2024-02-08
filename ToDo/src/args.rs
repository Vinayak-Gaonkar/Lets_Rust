use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct TODO {
    #[command(subcommand)]
    pub action: Commands,

    /// Item text
    pub item: Option<String>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds item to todo
    Add { name: Option<String> }
}