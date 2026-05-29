#[derive(Debug, Clone)]
pub enum RefineError {
    Pipeline(String),
    Validation(String),
    Crystallization(String),
}

impl core::fmt::Display for RefineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Pipeline(msg) => write!(f, "pipeline error: {msg}"),
            Self::Validation(msg) => write!(f, "validation error: {msg}"),
            Self::Crystallization(msg) => write!(f, "crystallization error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RefineError {}
