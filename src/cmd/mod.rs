pub mod app;
pub mod project;

use clap::{Parser, Subcommand};

pub enum Kind {
    Axum,
    Actix,
    Salvo,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Init,
    New {
        #[arg(short, long)]
        name: String,
    },
    App {
        #[arg(short, long)]
        name: Vec<String>,
    },
}
