use serde::{Deserialize, Serialize};
use typst_wasm_protocol::{wasm_export, Cbor, Json, PluginError};

#[wasm_export]
fn to_uppercase(input: &[u8]) -> Vec<u8> {
    input.to_ascii_uppercase()
}

#[wasm_export]
fn shout(text: &str) -> String {
    text.to_uppercase()
}

#[wasm_export]
fn echo(input: &[u8]) -> &[u8] {
    input
}

#[wasm_export(export_rename = "count_chars")]
fn count_characters(text: String) -> String {
    text.chars().count().to_string()
}

#[wasm_export]
fn divide(input: &str) -> Result<String, PluginError> {
    let (a, b) = input
        .split_once(',')
        .ok_or_else(|| PluginError::new("expected `a,b`").context("divide"))?;

    let a: f64 = a
        .trim()
        .parse()
        .map_err(|_| PluginError::new("invalid dividend").context("divide"))?;
    let b: f64 = b
        .trim()
        .parse()
        .map_err(|_| PluginError::new("invalid divisor").context("divide"))?;

    if b == 0.0 {
        return Err(PluginError::new("division by zero").context("divide"));
    }

    Ok(format!("{:.2}", a / b))
}

#[derive(Deserialize)]
struct TransformInput {
    text: String,
    repeat: u32,
}

#[derive(Serialize)]
struct TransformOutput {
    text: String,
    bytes: usize,
    repeat: u32,
}

fn transform(input: TransformInput) -> TransformOutput {
    let text = input.text.repeat(input.repeat as usize);
    TransformOutput {
        bytes: text.len(),
        text,
        repeat: input.repeat,
    }
}

#[wasm_export]
fn transform_json(Json(input): Json<TransformInput>) -> Result<Json<TransformOutput>, PluginError> {
    if input.repeat > 100 {
        return Err(PluginError::new("repeat must be <= 100").context("transform_json"));
    }
    Ok(Json(transform(input)))
}

#[wasm_export]
fn transform_cbor(Cbor(input): Cbor<TransformInput>) -> Result<Cbor<TransformOutput>, PluginError> {
    if input.repeat > 100 {
        return Err(PluginError::new("repeat must be <= 100").context("transform_cbor"));
    }
    Ok(Cbor(transform(input)))
}

#[wasm_export(check_args = true)]
fn checked_add(a: String, b: String) -> Result<String, PluginError> {
    let a: u64 = a
        .parse()
        .map_err(|_| PluginError::new("invalid first number").context("checked_add"))?;
    let b: u64 = b
        .parse()
        .map_err(|_| PluginError::new("invalid second number").context("checked_add"))?;

    let sum = a
        .checked_add(b)
        .ok_or_else(|| PluginError::new("overflow").context("checked_add"))?;

    Ok(sum.to_string())
}
