use crate::error::{PluginError, Result};

pub struct Cbor<T>(pub T);

impl<'a, T: serde::de::DeserializeOwned> crate::arg::FromArg<'a> for Cbor<T> {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        ciborium::de::from_reader(arg)
            .map(Cbor)
            .map_err(|e| PluginError::new(e.to_string()).context("Invalid cbor argument"))
    }
}

impl<'a, T: serde::Serialize> crate::IntoResponse<'a> for Cbor<T> {
    fn into_response(self) -> crate::Response<'a> {
        let mut bytes = Vec::new();
        match ciborium::ser::into_writer(&self.0, &mut bytes) {
            Ok(()) => crate::Response::ok(bytes),
            Err(err) => crate::Response::err(format!("Invalid cbor response: {err}")),
        }
    }
}
