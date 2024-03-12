use std::sync::Arc;
use crate::audio::Audio;

/// Application running context.
pub struct AppContext {
    /// Audio handler.
    pub audio: Arc<dyn Audio + Send + Sync + 'static>,
}
