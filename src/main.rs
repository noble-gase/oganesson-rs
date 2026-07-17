mod cmd;
mod core;

use clap::Parser;
use inquire::{Select, Text};

use crate::cmd::Kind;

fn main() {
    // 解析command
    let cli = cmd::Cli::parse();

    // 处理command
    match cli.command {
        Some(cmd::Command::Init) => {
            let (kind, apps) = project_form();
            cmd::project::run(None, kind, apps)
        }
        Some(cmd::Command::New { name }) => {
            let (kind, apps) = project_form();
            cmd::project::run(Some(name), kind, apps)
        }
        Some(cmd::Command::App { name }) => {
            let kind = app_form();
            cmd::app::run(name, kind)
        }
        _ => {
            println!("🦀 Welcome to use noble-gase[Rust] scaffold");
        }
    }
}

fn project_form() -> (Kind, Vec<String>) {
    let framework = Select::new("Framework:", vec!["axum", "actix", "salvo"])
        .prompt()
        .unwrap();
    let kind = match framework {
        "axum" => Kind::Axum,
        "actix" => Kind::Actix,
        "salvo" => Kind::Salvo,
        _ => panic!("Invalid framework"),
    };

    let app_raw = Text::new("Apps (comma separated, optional):")
        .with_placeholder("foo,bar")
        .prompt()
        .unwrap();
    let apps = app_raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();

    (kind, apps)
}

fn app_form() -> Kind {
    let framework = Select::new("Framework:", vec!["axum", "actix", "salvo"])
        .prompt()
        .unwrap();

    match framework {
        "axum" => Kind::Axum,
        "actix" => Kind::Actix,
        "salvo" => Kind::Salvo,
        _ => panic!("Invalid framework"),
    }
}
