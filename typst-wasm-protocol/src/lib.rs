pub use typst_wasm_macros::wasm_export;

#[link(wasm_import_module = "typst_env")]
unsafe extern "C" {
    #[link_name = "wasm_minimal_protocol_send_result_to_host"]
    unsafe fn __send_result_to_host(ptr: *const u8, len: usize);
    #[link_name = "wasm_minimal_protocol_write_args_to_buffer"]
    unsafe fn __write_args_to_buffer(ptr: *mut u8);
}

pub fn send_result_to_host(val: Vec<u8>) {
    unsafe {
        __send_result_to_host(val.as_ptr(), val.len());
    }
}

pub fn write_args_to_buffer(ptr: *mut u8) {
    unsafe {
        __write_args_to_buffer(ptr);
    }
}

pub trait PluginResult {
    fn send_result(self) -> i32;
}

impl<T, E> PluginResult for Result<T, E>
where
    T: Into<Vec<u8>>,
    E: ToString,
{
    fn send_result(self) -> i32 {
        let (value, code) = match self {
            Ok(value) => (value.into(), 0),
            Err(err) => (err.to_string().into_bytes(), 1),
        };
        send_result_to_host(value);
        code
    }
}

impl PluginResult for &[u8] {
    fn send_result(self) -> i32 {
        send_result_to_host(self.to_vec());
        0
    }
}

impl PluginResult for Vec<u8> {
    fn send_result(self) -> i32 {
        send_result_to_host(self);
        0
    }
}
