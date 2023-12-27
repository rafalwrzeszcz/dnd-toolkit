use async_trait::async_trait;

use crate::audio::Audio;

pub struct Void {
}

#[async_trait]
impl Audio for Void {
    async fn play(&self, _track: String) {}
}
