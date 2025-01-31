This repository contains examples of how to create and use wasm-components.

* `guest-rs`: A wasm-component implemented using Rust.
* `guest-py`: A wasm-component implemented using Python.
* `guest-js`: A wasm-component implemented using JavaScript.
* `host`: A host that loads and executes wasm-components using Rust and wasmtime.

# Requirements

```sh
# Rust
rustup target add wasm32-wasip2
cargo install cargo-component

# Python
pip install componentize-py

# Node.js
npm install @bytecodealliance/jco
```

## Running

```sh
cd host; cargo test --release  # Note: This can take some time
```
