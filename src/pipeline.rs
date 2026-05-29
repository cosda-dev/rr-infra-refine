use crate::error::RefineError;

#[derive(Debug, Default)]
pub struct RefinementPipeline;

impl RefinementPipeline {
    pub fn new() -> Self {
        Self
    }

    pub async fn process(&self, raw: &[u8], _blueprint: &str) -> Result<Vec<u8>, RefineError> {
        Ok(raw.to_vec())
    }
}
