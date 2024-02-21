use std::env;
use std::process::Command;

fn main() {
    let guest_rs = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("guest-rs");

    let guest_py = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("guest-py");

    println!("cargo:rerun-if-changed={}", guest_rs.display());
    println!("cargo:rerun-if-changed={}", guest_py.display());

    let status = Command::new("cargo")
        .current_dir(&guest_rs)
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build the crate in the parent directory");
    }

    let status = Command::new("cargo")
        .current_dir(&guest_rs)
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-wasi")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build the crate in the parent directory");
    }

    let status = Command::new("componentize-py")
        .current_dir(&guest_py)
        .arg("-d")
        .arg("hello.wit")
        .arg("-w")
        .arg("hello")
        .arg("componentize")
        .arg("app")
        .arg("-o")
        .arg("app.wasm")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build the crate in the parent directory");
    }
}
