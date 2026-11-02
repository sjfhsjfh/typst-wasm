use std::borrow::Cow;

pub enum Response<'a> {
    Ok(Cow<'a, [u8]>),
    Err(Cow<'a, str>),
}

impl<'a> Response<'a> {
    pub fn ok(body: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::Ok(body.into())
    }
    pub fn err(message: impl Into<Cow<'a, str>>) -> Self {
        Self::Err(message.into())
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            Self::Ok(value) => value.as_ref(),
            Self::Err(err) => err.as_bytes(),
        }
    }

    pub fn send(self) -> i32 {
        super::send_result_to_host(self.bytes());
        match self {
            Self::Ok(_) => 0,
            Self::Err(_) => 1,
        }
    }
}

pub trait IntoResponse<'a> {
    fn into_response(self) -> Response<'a>;

    fn send(self) -> i32
    where
        Self: Sized,
    {
        self.into_response().send()
    }
}

/// Convert a plugin function's return value and send it to the host.
pub fn send_response<'a, T: IntoResponse<'a>>(value: T) -> i32 {
    value.into_response().send()
}

impl<'a> IntoResponse<'a> for Response<'a> {
    fn into_response(self) -> Response<'a> {
        self
    }
}

impl<'a> IntoResponse<'a> for () {
    fn into_response(self) -> Response<'a> {
        Response::ok(&[])
    }
}

impl<'a> IntoResponse<'a> for Vec<u8> {
    fn into_response(self) -> Response<'a> {
        Response::ok(self)
    }
}

impl<'a> IntoResponse<'a> for &'a [u8] {
    fn into_response(self) -> Response<'a> {
        Response::ok(self)
    }
}

impl<'a> IntoResponse<'a> for String {
    fn into_response(self) -> Response<'a> {
        Response::ok(self.into_bytes())
    }
}

impl<'a> IntoResponse<'a> for &'a str {
    fn into_response(self) -> Response<'a> {
        Response::ok(self.as_bytes())
    }
}

impl<'a> IntoResponse<'a> for crate::PluginError {
    fn into_response(self) -> Response<'a> {
        Response::err(self.to_string())
    }
}

impl<'a, T: IntoResponse<'a>> IntoResponse<'a> for crate::Result<T> {
    fn into_response(self) -> Response<'a> {
        match self {
            Ok(value) => value.into_response(),
            Err(err) => Response::err(err.to_string()),
        }
    }
}
