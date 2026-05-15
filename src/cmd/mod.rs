pub mod app;
pub mod project;

use clap::{Parser, Subcommand};

pub enum Kind {
    Axum,
    Salvo,
    Actix,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    New {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        axum: bool,

        #[arg(short, long)]
        salvo: bool,

        #[arg(short, long)]
        app: Vec<String>,
    },
    App {
        #[arg(short, long)]
        name: Vec<String>,

        #[arg(short, long)]
        axum: bool,

        #[arg(short, long)]
        salvo: bool,
    },
}
