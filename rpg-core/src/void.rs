use async_trait::async_trait;

use crate::audio::{Audio, AudioError};
use crate::lights::{Lights, LightsError};

/// No-op implementation.
///
/// This is an empty implementation that can be used to disable particular sub-system. It implements as many traits as
/// possible within toolkit and in every case, where possible, will simply do nothing without failing.
///
/// It may be useful especially in case of multi-node setup where given aspects will be handled by separate nodes.
pub struct Void {}

/// Using this driver for audio will simply result in no audio being ever played without any error.
#[async_trait]
impl Audio for Void {
    async fn play(&self, _track: String) -> Result<(), AudioError> {
        Ok(())
    }
}

/// Using this driver for lights will simply result in no reactions to any lights events without any error.
#[async_trait]
impl Lights for Void {
    async fn brightness(&self, _level: i32) -> Result<(), LightsError> {
        Ok(())
    }
}
