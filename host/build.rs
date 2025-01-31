use std::env;
use std::process::Command;

fn main() {
    build_rs();
    build_py();
    build_js();
}

fn build_rs() {
    let guest_rs = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("guest-rs");

    let status = Command::new("cargo")
        .current_dir(&guest_rs)
        .arg("component")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-wasip2")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build {}", guest_rs.display());
    }

    println!("cargo:rerun-if-changed={}", guest_rs.display());
}

fn build_py() {
    let guest_py = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("guest-py");

    let status = Command::new("componentize-py")
        .current_dir(&guest_py)
        .arg("-d")
        .arg("component.wit")
        .arg("-w")
        .arg("component")
        .arg("bindings")
        .arg("bindings")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build {}", guest_py.display());
    }

    let status = Command::new("componentize-py")
        .current_dir(&guest_py)
        .arg("-d")
        .arg("component.wit")
        .arg("-w")
        .arg("component")
        .arg("componentize")
        .arg("lib")
        .arg("-o")
        .arg("component.wasm")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build {}", guest_py.display());
    }

    println!("cargo:rerun-if-changed={}", guest_py.display());
}

fn build_js() {
    let guest_js = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("guest-js");

    println!("cargo:rerun-if-changed={}", guest_js.display());

    let status = Command::new("npx")
        .current_dir(&guest_js)
        .arg("jco")
        .arg("componentize")
        .arg("lib.js")
        .arg("--wit")
        .arg("component.wit")
        .arg("-n")
        .arg("component")
        .arg("-o")
        .arg("component.wasm")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build {}", guest_js.display());
    }
}
