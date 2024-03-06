use rpg_core::audio::Audio;
use std::sync::Arc;

pub struct AppContext {
    pub audio: Arc<dyn Audio + Send + Sync + 'static>,
}
