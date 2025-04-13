use std::str;
use typst_wasm_protocol::wasm_export;

// Simple string operation function
#[wasm_export]
fn to_uppercase(input: &[u8]) -> Vec<u8> {
    let input_str = match str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return b"Invalid UTF-8 input".to_vec(),
    };

    input_str.to_uppercase().into_bytes()
}

// Using custom export name
#[wasm_export(export_rename = "count_chars")]
fn count_characters(input: &[u8]) -> Vec<u8> {
    let input_str = match str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return b"Invalid UTF-8 input".to_vec(),
    };

    format!("Character count: {}", input_str.chars().count()).into_bytes()
}

// Function returning Result type
#[wasm_export]
fn divide_numbers(input: &[u8]) -> Result<String, String> {
    let input_str = str::from_utf8(input).map_err(|e| format!("UTF-8 error: {}", e))?;

    let numbers: Vec<&str> = input_str.split(',').collect();
    if numbers.len() != 2 {
        return Err("Expected two comma-separated numbers".to_string());
    }

    let a: f64 = numbers[0]
        .trim()
        .parse()
        .map_err(|_| "First value is not a valid number".to_string())?;
    let b: f64 = numbers[1]
        .trim()
        .parse()
        .map_err(|_| "Second value is not a valid number".to_string())?;

    if b == 0.0 {
        return Err("Cannot divide by zero".to_string());
    }

    let result = a / b;
    Ok(format!("Result: {:.2}", result))
}

// Function returning Result<String, String> type
#[wasm_export]
fn validate_email(input: &[u8]) -> Result<String, String> {
    let email = str::from_utf8(input)
        .map_err(|e| format!("UTF-8 error: {}", e))?
        .trim();

    // Simple email validation
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }

    Ok("Email is valid".to_string())
}
