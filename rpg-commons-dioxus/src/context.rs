use rpg_core::audio::Audio;
use std::sync::Arc;

/// Application state exposed to UI.
pub struct AppContext {
    pub audio: Arc<dyn Audio + Send + Sync + 'static>,
}
