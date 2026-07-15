# oganesson-rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/oganesson-rs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/oganesson-rs)
[<img alt="MIT" src="http://img.shields.io/badge/license-MIT-brightgreen.svg?style=for-the-badge" height="20">](http://opensource.org/licenses/MIT)

[鿫-Oganesson] Rust Web开发脚手架，支持 `actix-web`、 `axum` 和 `salvo` 框架，并支持「单应用」和「多应用」模式

## 安装

```shell
cargo install oganesson-rs
```

## 特点

- DB使用 [sqlx](https://github.com/launchbadge/sqlx)
- Redis使用 [redis-rs](https://github.com/redis-rs/redis-rs)
- 日志使用 [tracing](https://github.com/tokio-rs/tracing)
- 配置使用 [config-rs](https://github.com/mehcode/config-rs)
- 命令行使用 [clap](https://github.com/clap-rs/clap)
- 异步运行时使用 [tokio](https://github.com/tokio-rs/tokio)
- 参数验证器使用 [validator](https://github.com/Keats/validator)
- 支持 Prometheus Metrics 和 Request 中间件
- 包含 TraceId、认证、请求日志、Panic捕获 中间件
- 简单好用的 API Result 统一输出方式

## 创建项目

<details>
<summary>点击展开</summary>

```shell
og init # 在当前目录初始化项目
og new --name demo # 创建demo项目
.
├── Cargo.toml
├── Dockerfile
├── app/
│   ├── Cargo.toml
│   ├── config.toml
│   └── src/
│       ├── cmd/
│       ├── handler/
│       ├── middleware/
│       ├── router/
│       ├── service/
│       └── main.rs
├── infra/
│   ├── Cargo.toml
│   └── src/
│       ├── core/
│       ├── middleware/
│       ├── status/
│       ├── util/
│       └── lib.rs
└── repo/
    ├── Cargo.toml
    └── src/
        ├── dao/
        ├── schema/
        └── lib.rs

```

</details>

## 创建应用

<details>
<summary>点击展开</summary>

> 多应用项目适用，需在项目根目录执行（即：`Cargo.toml` 所在目录）

```shell
og app --name foo --name bar
.
├── Cargo.toml
├── Dockerfile.foo
├── Dockerfile.bar
├── app/
│   ├── foo/
│   │   ├── Cargo.toml
│   │   ├── config.toml
│   │   └── src/
│   │       ├── cmd/
│   │       ├── handler/
│   │       ├── middleware/
│   │       ├── router/
│   │       ├── service/
│   │       └── main.rs
│   └── bar/
│       ├── Cargo.toml
│       ├── config.toml
│       └── src/
│           ├── ...
│           └── main.rs
├── infra/
│   ├── Cargo.toml
│   └── src/
│       ├── ...
│       └── lib.rs
└── repo/
    ├── Cargo.toml
    └── src/
        ├── ...
        └── lib.rs
```

</details>

**Enjoy 😊**
