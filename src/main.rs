mod cmd;
mod core;

use clap::Parser;

use crate::cmd::Kind;

fn main() {
    // 解析command
    let cli = cmd::Cli::parse();
    // 处理command
    match cli.command {
        Some(cmd::Command::New { name, axum, salvo, app }) => cmd::project::run(
            name,
            if axum {
                Kind::Axum
            } else if salvo {
                Kind::Salvo
            } else {
                Kind::Actix
            },
            app,
        ),
        Some(cmd::Command::App { name, axum, salvo }) => cmd::app::run(
            name,
            if axum {
                Kind::Axum
            } else if salvo {
                Kind::Salvo
            } else {
                Kind::Actix
            },
        ),
        _ => {
            println!("🦀 Welcome to use noble-gase[Rust] scaffold");
        }
    }
}
