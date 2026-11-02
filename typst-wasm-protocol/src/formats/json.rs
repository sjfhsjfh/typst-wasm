use crate::error::{PluginError, Result};

#[cfg(feature = "json")]
pub struct Json<T>(pub T);

impl<'a, T: serde::Deserialize<'a>> crate::arg::FromArg<'a> for Json<T> {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        let mut de = serde_json::Deserializer::from_slice(arg);
        serde_path_to_error::deserialize(&mut de)
            .map(Json)
            .map_err(|e| PluginError::new(e.to_string()).context("Invalid json argument"))
    }
}

impl<'a, T: serde::Serialize> crate::IntoResponse<'a> for Json<T> {
    fn into_response(self) -> crate::Response<'a> {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => crate::Response::ok(bytes),
            Err(err) => crate::Response::err(format!("Invalid json response: {err}")),
        }
    }
}
