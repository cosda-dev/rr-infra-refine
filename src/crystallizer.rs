use crate::error::RefineError;

#[derive(Debug, Default)]
pub struct ArtifactCrystallizer;

impl ArtifactCrystallizer {
    pub fn new() -> Self {
        Self
    }

    pub async fn crystallize(&self, artifact: &[u8]) -> Result<Vec<u8>, RefineError> {
        Ok(artifact.to_vec())
    }
}
