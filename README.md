# Typst WASM Protocol

[![Crates.io](https://img.shields.io/crates/v/typst-wasm-protocol.svg)](https://crates.io/crates/typst-wasm-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A smol toolkit for writing [Typst WebAssembly plugins](https://typst.app/docs/reference/foundations/plugin/) in Rust.

- [`typst-wasm-protocol`](typst-wasm-protocol): `#[wasm_export]`, argument extractors, response conversion, JSON/CBOR helpers
- [`typst-wasm-macros`](typst-wasm-macros): the proc macro (re-exported by the protocol crate)

## Installation

```toml
[dependencies]
typst-wasm-protocol = "0.0.2"
serde = { version = "1", features = ["derive"] } # if you're using Json/Cbor
```

MSRV: 1.82.

## Quick start

```rust
use serde::{Deserialize, Serialize};
use typst_wasm_protocol::{wasm_export, Cbor, PluginError};

#[derive(Deserialize, Serialize)]
struct Point {
    x: i32,
    y: i32,
}

#[wasm_export]
pub fn double(Cbor(point): Cbor<Point>) -> Result<Cbor<Point>, PluginError> {
    Ok(Cbor(Point {
        x: point.x * 2,
        y: point.y * 2,
    }))
}
```

```typ
#let p = plugin("plugin.wasm")
#let input = (x: 2, y: 3)
#cbor(p.double(bytes(cbor.encode(input))))
```

You may also use `&[u8]` or `&str` to handle raw input directly.

## Argument patterns

The macro keeps the original function intact, so extractor-style parameters work:

```rust
#[wasm_export]
pub fn transform(Json(point): Json<Point>) -> Result<Json<Point>, PluginError> {
    // ...
}
```

## Errors

Return `Result<T, PluginError>`; the message is sent to Typst as the plugin error:

```rust
Err(PluginError::new("invalid input").context("parse"))
```

Typst reports:

```text
plugin errored with: parse: invalid input
```

## Length checking

By default the generated wrapper trusts the argument lengths supplied by the host. If the module can be called by other WebAssembly hosts, enable checking:

```rust
#[wasm_export(check_args = true)]
pub fn checked(input: &[u8]) -> Vec<u8> {
    input.to_vec()
}
```

With checking enabled, overflow, allocation failure, and out-of-bounds reads become `PluginError` responses instead of traps.

## Export renaming

```rust
#[wasm_export(export_rename = "greet")]
pub fn say_hello(name: &[u8]) -> Vec<u8> {
    format!("Hello, {}!", String::from_utf8_lossy(name)).into_bytes()
}
```

## Examples

See [`typst-wasm-protocol/examples`](typst-wasm-protocol/examples) for a runnable feature showcase covering raw bytes, `Json<T>`, `Cbor<T>`, error handling, `export_rename`, and `check_args`.
