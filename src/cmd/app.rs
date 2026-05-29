use std::{env, fs};

use crate::{cmd::Kind, core};

pub fn run(apps: Vec<String>, kind: Kind) {
    // 检查Cargo.toml是否存在
    if fs::metadata("Cargo.toml").is_err() {
        println!("👿 Cargo.toml does not exist, please confirm!");
        return;
    }

    // 获取当前目录
    let dir = env::current_dir().unwrap().canonicalize().unwrap();

    match kind {
        Kind::Axum => core::build_axum_app(&dir, apps),
        Kind::Salvo => core::build_salvo_app(&dir, apps),
        _ => core::build_actix_app(&dir, apps),
    }

    println!("🦀 The app is now created!");
}
