use crate::audio::Audio;
use crate::lights::Lights;
use std::sync::Arc;

/// Application running context.
pub struct AppContext {
    /// Audio handler.
    pub audio: Arc<dyn Audio + Send + Sync + 'static>,
    /// Lights handler.
    pub lights: Arc<dyn Lights + Send + Sync + 'static>,
}
