pub mod actix;
pub mod axum;
pub mod salvo;

use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use tera::{Context, Tera};

pub fn docker() -> Tera {
    let mut tera = Tera::default();

    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        ("Dockerfile", include_str!("../../template/Dockerfile.tera")),
        ("dockerun.sh", include_str!("../../template/dockerun.tera")),
    ])
    .unwrap();

    tera
}

pub fn is_empty_dir(path: &Path) -> bool {
    match path.read_dir() {
        Ok(entries) => entries.count() == 0,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => true,
            _ => panic!("{e}"),
        },
    }
}

// --------------------------- actix ---------------------------

pub fn build_actix_project(root: &Path, name: String, apps: Vec<String>) {
    let libs = (actix::global(), actix::infra(), actix::repo());
    let members = gen_members(&apps, Some(vec!["infra".to_string(), "repo".to_string()]));

    build_project(root, &name, &members, libs);

    // single app
    if apps.is_empty() {
        build_app(root, &name, None, actix::app());
        return;
    }

    // multi apps
    for app in &apps {
        build_app(root, app, Some(app), actix::app());
    }
}

pub fn build_actix_app(root: &Path, apps: Vec<String>) {
    for app in &apps {
        build_app(root, app, Some(app), actix::app());
    }
}

// --------------------------- axum ---------------------------

pub fn build_axum_project(root: &Path, name: String, apps: Vec<String>) {
    let libs = (axum::global(), axum::infra(), axum::repo());
    let members = gen_members(&apps, Some(vec!["infra".to_string(), "repo".to_string()]));

    build_project(root, &name, &members, libs);

    // single app
    if apps.is_empty() {
        build_app(root, &name, None, axum::app());
        return;
    }

    // multi apps
    for app in &apps {
        build_app(root, app, Some(app), axum::app());
    }
}

pub fn build_axum_app(root: &Path, apps: Vec<String>) {
    for app in &apps {
        build_app(root, app, Some(app), axum::app());
    }
}

// --------------------------- salvo ---------------------------

pub fn build_salvo_project(root: &Path, name: String, apps: Vec<String>) {
    let libs = (salvo::global(), salvo::infra(), salvo::repo());
    let members = gen_members(&apps, Some(vec!["infra".to_string(), "repo".to_string()]));

    build_project(root, &name, &members, libs);

    // single app
    if apps.is_empty() {
        build_app(root, &name, None, salvo::app());
        return;
    }

    // multi apps
    for app in &apps {
        build_app(root, app, Some(app), salvo::app());
    }
}

pub fn build_salvo_app(root: &Path, apps: Vec<String>) {
    for app in &apps {
        build_app(root, app, Some(app), salvo::app());
    }
}

// --------------------------- project & app ---------------------------

fn build_project(root: &Path, name: &str, members: &str, libs: (tera::Tera, tera::Tera, tera::Tera)) {
    let mut ctx = Context::new();
    ctx.insert("name", name);
    ctx.insert("members", members);

    // 创建项目
    println!("🦀 Create project: {name}");

    let (tera_global, tera_infra, tera_repo) = libs;

    // global
    gen_files(&ctx, root, vec![], tera_global);
    // infra
    gen_files(&ctx, root, vec!["infra"], tera_infra);
    // repo
    gen_files(&ctx, root, vec!["repo"], tera_repo);
}

fn build_app(root: &Path, name: &str, crate_name: Option<&str>, template: tera::Tera) {
    // 创建app
    let mut ctx = Context::new();
    let mut subset = vec!["app"];

    // 模式
    ctx.insert("app_name", name);
    match crate_name {
        None => {
            ctx.insert("app_crate", "app");
        }
        Some(v) => {
            ctx.insert("app_crate", v);
            subset.push(v);

            println!("🦀 Create application: {v}");
        }
    };

    // app
    gen_files(&ctx, root, subset, template);

    // dockerfile
    let tera_docker = docker();
    for filename in tera_docker.get_template_names() {
        let content = tera_docker.render(filename, &ctx).unwrap();
        let path = match crate_name {
            None => root.join(filename),
            Some(v) => {
                if filename == "Dockerfile" {
                    root.join(format!("Dockerfile.{v}").as_str())
                } else {
                    root.join(format!("{v}_{}", filename.to_lowercase()).as_str())
                }
            }
        };

        // 创建文件
        let mut file = File::create(path).unwrap();
        // 将内容写入文件
        file.write_all(content.as_bytes()).unwrap();

        println!("{filename}")
    }
}

pub fn gen_members(apps: &[String], base: Option<Vec<String>>) -> String {
    let mut members = Vec::new();

    if let Some(list) = base {
        for v in list {
            members.push(v);
        }
    }

    if apps.is_empty() {
        members.push("app".to_string())
    } else {
        members.push("app/*".to_string())
    }

    format!("[{}]", members.iter().map(|m| format!("\"{}\"", m)).collect::<Vec<_>>().join(", "))
}

fn gen_files(ctx: &Context, root: &Path, subset: Vec<&str>, template: tera::Tera) {
    if subset.is_empty() {
        for filename in template.get_template_names() {
            let content = template.render(filename, ctx).unwrap();
            let path = root.join(filename);

            // 创建文件
            let mut file = File::create(path).unwrap();
            // 将内容写入文件
            file.write_all(content.as_bytes()).unwrap();

            println!("{filename}")
        }
        return;
    }

    let prefix = &subset.join("/");
    let dir = subset.into_iter().fold(root.to_path_buf(), |acc, part| acc.join(part));

    if !is_empty_dir(&dir) {
        println!("👿 The directory({dir:?}) is not empty, please confirm!");
        return;
    }

    for filename in template.get_template_names() {
        let content = template.render(filename, ctx).unwrap();
        let path = dir.join(filename);
        if let Some(v) = path.parent() {
            fs::create_dir_all(v).unwrap();
        }

        // 创建文件
        let mut file = File::create(path).unwrap();
        // 将内容写入文件
        file.write_all(content.as_bytes()).unwrap();

        println!("{prefix}/{filename}")
    }
}
