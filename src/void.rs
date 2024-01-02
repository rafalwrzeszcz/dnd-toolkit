use async_trait::async_trait;

use crate::audio::{Audio, AudioError};

pub struct Void {
}

#[async_trait]
impl Audio for Void {
    async fn play(&self, _track: String) -> Result<(), AudioError> {
        Ok(())
    }
}
