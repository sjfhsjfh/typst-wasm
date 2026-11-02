mod arg;
mod error;
mod formats;
mod response;

pub use arg::{read_args, read_args_unchecked, ArgsReader, FromArg};
pub use error::{PluginError, Result};
pub use response::{send_response, IntoResponse, Response};

#[cfg(feature = "json")]
pub use formats::Json;

pub use typst_wasm_macros::wasm_export;

#[link(wasm_import_module = "typst_env")]
unsafe extern "C" {
    #[link_name = "wasm_minimal_protocol_send_result_to_host"]
    unsafe fn __send_result_to_host(ptr: *const u8, len: usize);
    #[link_name = "wasm_minimal_protocol_write_args_to_buffer"]
    unsafe fn __write_args_to_buffer(ptr: *mut u8);
}

pub fn send_result_to_host(val: &[u8]) {
    unsafe {
        __send_result_to_host(val.as_ptr(), val.len());
    }
}

pub fn write_args_to(buffer: &mut [u8]) {
    unsafe {
        __write_args_to_buffer(buffer.as_mut_ptr());
    }
}
