use std::{env, fs};

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let lib_name = if target_arch == "x86_64" {
        "VigemClient_x64"
    }
    else if target_arch == "x86" {
        "VigemClient_x86"
    }
    else {
        panic!("Target arch {} is not supported!", target_arch)
    };

    let project_dir = env::var("OUT_DIR").unwrap();
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let first_path = format!("{}\\libs\\{}.lib", root_dir, lib_name);
    let second_path = format!("{}\\{}.lib", project_dir, lib_name);

    let result = fs::copy(&first_path, &second_path);
    if result.is_err(){
        println!("cargo:warning=Failed to copy file. Hi, docs.rs!");
    }

    println!("cargo:rustc-link-search=static={}", format!("{}\\", project_dir));
    println!("cargo:rustc-link-lib=setupapi");

    println!("cargo:rustc-link-lib={}", lib_name);
}
