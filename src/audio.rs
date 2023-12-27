use async_trait::async_trait;

#[async_trait]
pub trait Audio {
    async fn play(&self, track: String);
}
