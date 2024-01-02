use async_trait::async_trait;

#[derive(Debug)]
pub enum AudioError {
    PlayError,
}

#[async_trait]
pub trait Audio {
    async fn play(&self, track: String) -> Result<(), AudioError>;
}
