#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginError {
    message: String,
}

pub type Result<T> = std::result::Result<T, PluginError>;

impl PluginError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn into_message(self) -> String {
        self.message
    }

    pub fn context(self, context: impl std::fmt::Display) -> Self {
        Self::new(format!("{context}: {}", self.message))
    }
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for PluginError {}

impl From<String> for PluginError {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
impl From<&str> for PluginError {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}
