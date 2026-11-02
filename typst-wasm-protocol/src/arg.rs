use crate::error::{PluginError, Result};

use std::borrow::Cow;

fn invalid_arg(msg: impl Into<String>) -> PluginError {
    PluginError::new(msg).context("Invalid argument")
}

pub trait FromArg<'a>: Sized {
    fn from_arg(arg: &'a [u8]) -> Result<Self>;
}

impl<'a> FromArg<'a> for &'a [u8] {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        Ok(arg)
    }
}

impl<'a> FromArg<'a> for &'a str {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        std::str::from_utf8(arg).map_err(|e| invalid_arg(e.to_string()))
    }
}

impl<'a> FromArg<'a> for Vec<u8> {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        Ok(arg.to_vec())
    }
}

impl<'a> FromArg<'a> for String {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        String::from_utf8(arg.to_vec()).map_err(|e| invalid_arg(e.to_string()))
    }
}

impl<'a> FromArg<'a> for Cow<'a, [u8]> {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        Ok(Cow::Borrowed(arg))
    }
}

impl<'a> FromArg<'a> for Cow<'a, str> {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        std::str::from_utf8(arg)
            .map(Cow::Borrowed)
            .map_err(|e| invalid_arg(e.to_string()))
    }
}

impl<'a, T: FromArg<'a>> FromArg<'a> for Result<T> {
    fn from_arg(arg: &'a [u8]) -> Result<Self> {
        Ok(T::from_arg(arg))
    }
}

pub fn read_args_unchecked(lens: &[usize]) -> Vec<u8> {
    let total = lens.iter().sum();
    let mut buffer = vec![0; total];
    crate::write_args_to(&mut buffer);
    buffer
}

pub fn read_args(lens: &[usize]) -> Result<Vec<u8>> {
    let total = lens
        .iter()
        .try_fold(0usize, |total, &len| total.checked_add(len))
        .ok_or_else(|| PluginError::new("argument lengths overflow"))?;

    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(total)
        .map_err(|_| PluginError::new("failed to allocate argument buffer"))?;
    buffer.resize(total, 0);
    crate::write_args_to(&mut buffer);
    Ok(buffer)
}

pub struct ArgsReader<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> ArgsReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, offset: 0 }
    }

    pub fn read_unchecked<T: FromArg<'a>>(&mut self, len: usize) -> Result<T> {
        let end = self.offset + len;
        let slice = &self.buffer[self.offset..end];
        self.offset = end;
        T::from_arg(slice)
    }

    pub fn read<T: FromArg<'a>>(&mut self, len: usize) -> Result<T> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or_else(|| PluginError::new("argument length overflow"))?;
        let slice = self
            .buffer
            .get(self.offset..end)
            .ok_or_else(|| PluginError::new("argument out of bounds"))?;
        self.offset = end;
        T::from_arg(slice)
    }
}
