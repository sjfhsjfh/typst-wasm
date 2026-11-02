# Examples

- `basic/` — raw bytes, `&str`, borrowed returns, `Json<T>`, `Cbor<T>`,
  `PluginError`, `export_rename`, and `check_args`.

## Build

```sh
cargo build --workspace --target wasm32-unknown-unknown --release
```

The wasm files land at:

```text
target/wasm32-unknown-unknown/release/*.wasm
```

Crates share the size-oriented release profile in `examples/Cargo.toml`.
