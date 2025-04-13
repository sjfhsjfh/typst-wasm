# Typst WASM Protocol

[![Crates.io](https://img.shields.io/crates/v/typst-wasm-protocol.svg)](https://crates.io/crates/typst-wasm-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/typst-wasm-protocol.svg)](https://crates.io/crates/typst-wasm-protocol)

A toolkit for [Typst Plugins](https://typst.app/docs/reference/foundations/plugin/) that provides a macro and protocol for exporting functions to WebAssembly.

## Installation

Add the following dependency to your Rust project:

```toml
[dependencies]
typst-wasm-protocol = "0.0.2"
```

## Usage

### Exporting Functions to WASM

Use the `wasm_export` macro to mark functions for export:

```rust
use typst_wasm_protocol::wasm_export;

#[wasm_export]
pub fn hello_world(name: &[u8]) -> Vec<u8> {
    format!("Hello, {}!", String::from_utf8_lossy(name)).into_bytes()
}

// Custom export name
#[wasm_export(export_rename = "greet")]
pub fn say_hello(name: &[u8]) -> Vec<u8> {
    format!("Hello, {}!", String::from_utf8_lossy(name)).into_bytes()
}
```

### Handling Results and Errors

The protocol provides a `PluginResult` trait that automatically handles data conversion for various return types:

```rust
use typst_wasm_protocol::PluginResult;

#[wasm_export]
pub fn process_data(input: &[u8]) -> Result<Vec<u8>, String> {
    // Process data, return Result
    // The PluginResult trait automatically handles the conversion
    // No manual data structure transformation needed
    Ok(input.to_vec())
}

// Works with different Result types without manual conversions
#[wasm_export]
pub fn validate_text(text: &[u8]) -> Result<String, String> {
    let text_str = std::str::from_utf8(text).map_err(|e| e.to_string())?;
    if text_str.len() > 10 {
        Ok("Text is valid".to_string())
    } else {
        Err("Text is too short".to_string())
    }
}
```

## Examples

See [typst-wasm-protocol/examples](typst-wasm-protocol/examples) for a basic example of using the `wasm_export` macro and handling results. Also [typst-relescope](https://github.com/sjfhsjfh/typst-relescope) as a real-world example of a Typst plugin using this protocol.

## Building WASM Modules

Compile to WebAssembly using:

```bash
cargo build --target wasm32-unknown-unknown --release
```
