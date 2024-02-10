use async_trait::async_trait;

#[derive(Debug)]
pub enum AudioError {
    PlayError,
}

/// Audio controlling aspect.
///
/// This trait exposes methods for controlling audio playback. Media reference is assumed to be a string refering to
/// particular media - handling of that reference will depend on particular implementation (please see README for
/// general concepts).
#[async_trait]
pub trait Audio {
    /// Plays given media.
    ///
    /// # Arguments
    ///
    /// * `track` - Media identifier/reference.
    ///
    /// # Examples
    ///
    /// ```
    /// audio.play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into()).await?
    /// ```
    async fn play(&self, track: String) -> Result<(), AudioError>;
}
