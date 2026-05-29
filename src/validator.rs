use crate::error::RefineError;

#[derive(Debug, Default)]
pub struct ArtifactValidator;

impl ArtifactValidator {
    pub fn new() -> Self {
        Self
    }

    pub async fn validate(&self, artifact: &[u8], _blueprint: &str) -> Result<Vec<u8>, RefineError> {
        Ok(artifact.to_vec())
    }
}
