use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LightsError {
    #[error("Could not set lights brightness.")]
    BrightnessError,
}

/// Lights controlling aspect.
///
/// This trait exposes methods for controlling lights playback. In various cases numeric values can have different
/// values ranges (e.g. 0-100%, 0-255 etc.). For the moment driver intention is not to unify those differences, but
/// pass them to underlying system.
#[async_trait]
pub trait Lights {
    /// Sets lights brightness to given level.
    ///
    /// # Arguments
    ///
    /// * `level` - Brightness level.
    ///
    /// # Examples
    ///
    /// ```
    /// // 128 is half of the range for BleBox implementation
    /// lights.brightness(128).await?
    /// ```
    async fn brightness(&self, level: i32) -> Result<(), LightsError>;
}
