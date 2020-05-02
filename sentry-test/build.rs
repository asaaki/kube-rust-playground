// build.rs
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
// https://doc.rust-lang.org/cargo/reference/environment-variables.html

use std::{env, fs, path::Path};

fn main() {
    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");
    let app_release = format!("{}@{}", app_name, app_version);
    let app_profile = std::env::var("PROFILE").unwrap();
    let app_target = std::env::var("TARGET").unwrap();

    let release_rs = format!(
        r#"
    #[allow(dead_code)]
    const APP_NAME: &str = "{}";
    #[allow(dead_code)]
    const APP_VERSION: &str = "{}";
    #[allow(dead_code)]
    const APP_RELEASE: &str = "{}";
    #[allow(dead_code)]
    const APP_PROFILE: &str = "{}";
    #[allow(dead_code)]
    const APP_TARGET: &str = "{}";
    "#,
        app_name, app_version, app_release, app_profile, app_target
    );

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("app_data.rs");
    fs::write(&dest_path, release_rs).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
