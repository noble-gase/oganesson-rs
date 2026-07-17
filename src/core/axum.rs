use std::vec;

use tera::Tera;

pub fn global() -> Tera {
    let mut tera = Tera::default();

    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        (
            ".dockerignore",
            include_str!("../../template/dockerignore.tera"),
        ),
        (".gitignore", include_str!("../../template/gitignore.tera")),
        ("Cargo.toml", include_str!("../../template/axum/Cargo.tera")),
        ("README.md", include_str!("../../template/axum/README.tera")),
    ])
    .unwrap();

    tera
}

pub fn app() -> Tera {
    let mut tera = Tera::default();

    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // Cargo.toml
        (
            "Cargo.toml",
            include_str!("../../template/axum/app/Cargo.tera"),
        ),
        // config.toml
        (
            "config.toml",
            include_str!("../../template/axum/app/config.tera"),
        ),
        // main.rs
        (
            "src/main.rs",
            include_str!("../../template/axum/app/src/main.tera"),
        ),
        // cmd
        (
            "src/cmd/mod.rs",
            include_str!("../../template/axum/app/src/cmd/mod.tera"),
        ),
        (
            "src/cmd/hello.rs",
            include_str!("../../template/axum/app/src/cmd/hello.tera"),
        ),
        (
            "src/cmd/serve.rs",
            include_str!("../../template/axum/app/src/cmd/serve.tera"),
        ),
        // handler
        (
            "src/handler/mod.rs",
            include_str!("../../template/axum/app/src/handler/mod.tera"),
        ),
        (
            "src/handler/greeter.rs",
            include_str!("../../template/axum/app/src/handler/greeter.tera"),
        ),
        // router
        (
            "src/router/mod.rs",
            include_str!("../../template/axum/app/src/router/mod.tera"),
        ),
        (
            "src/router/route.rs",
            include_str!("../../template/axum/app/src/router/route.tera"),
        ),
        // service
        (
            "src/service/mod.rs",
            include_str!("../../template/axum/app/src/service/mod.tera"),
        ),
        (
            "src/service/greeter.rs",
            include_str!("../../template/axum/app/src/service/greeter.tera"),
        ),
    ])
    .unwrap();

    tera
}

pub fn infra() -> Tera {
    let mut tera = Tera::default();

    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // Cargo.toml
        (
            "Cargo.toml",
            include_str!("../../template/axum/infra/Cargo.tera"),
        ),
        // lib.rs
        (
            "src/lib.rs",
            include_str!("../../template/axum/infra/src/lib.tera"),
        ),
        // core
        (
            "src/core/mod.rs",
            include_str!("../../template/axum/infra/src/core/mod.tera"),
        ),
        (
            "src/core/cache.rs",
            include_str!("../../template/axum/infra/src/core/cache.tera"),
        ),
        (
            "src/core/config.rs",
            include_str!("../../template/axum/infra/src/core/config.tera"),
        ),
        (
            "src/core/db.rs",
            include_str!("../../template/axum/infra/src/core/db.tera"),
        ),
        (
            "src/core/log.rs",
            include_str!("../../template/axum/infra/src/core/log.tera"),
        ),
        // extract
        (
            "src/extract/mod.rs",
            include_str!("../../template/axum/infra/src/extract/mod.tera"),
        ),
        (
            "src/extract/json.rs",
            include_str!("../../template/axum/infra/src/extract/json.tera"),
        ),
        // middleware
        (
            "src/middleware/mod.rs",
            include_str!("../../template/axum/infra/src/middleware/mod.tera"),
        ),
        (
            "src/middleware/log.rs",
            include_str!("../../template/axum/infra/src/middleware/log.tera"),
        ),
        (
            "src/middleware/metric.rs",
            include_str!("../../template/axum/infra/src/middleware/metric.tera"),
        ),
        (
            "src/middleware/panic.rs",
            include_str!("../../template/axum/infra/src/middleware/panic.tera"),
        ),
        (
            "src/middleware/trace.rs",
            include_str!("../../template/axum/infra/src/middleware/trace.tera"),
        ),
        // status
        (
            "src/status/mod.rs",
            include_str!("../../template/axum/infra/src/status/mod.tera"),
        ),
        (
            "src/status/api_ok.rs",
            include_str!("../../template/axum/infra/src/status/api_ok.tera"),
        ),
        (
            "src/status/api_err.rs",
            include_str!("../../template/axum/infra/src/status/api_err.tera"),
        ),
    ])
    .unwrap();

    tera
}

pub fn repo() -> Tera {
    let mut tera = Tera::default();

    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // Cargo.toml
        (
            "Cargo.toml",
            include_str!("../../template/axum/repo/Cargo.tera"),
        ),
        // lib.rs
        (
            "src/lib.rs",
            include_str!("../../template/axum/repo/src/lib.tera"),
        ),
        // dao
        (
            "src/dao/mod.rs",
            include_str!("../../template/axum/repo/src/dao/mod.tera"),
        ),
        (
            "src/dao/demo.rs",
            include_str!("../../template/axum/repo/src/dao/demo.tera"),
        ),
        // schema
        (
            "src/schema/mod.rs",
            include_str!("../../template/axum/repo/src/schema/mod.tera"),
        ),
        (
            "src/schema/model.rs",
            include_str!("../../template/axum/repo/src/schema/model.tera"),
        ),
        (
            "src/schema/table.rs",
            include_str!("../../template/axum/repo/src/schema/table.tera"),
        ),
    ])
    .unwrap();

    tera
}
