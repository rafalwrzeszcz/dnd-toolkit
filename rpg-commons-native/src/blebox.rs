use async_trait::async_trait;
use reqwest::Client;
use rpg_core::lights::{Lights, LightsError};
use serde_json::json;
use tracing::info;

pub struct BleBox {
    client: Client,
    host: String,
}

/// BleBox DimmerBox handler over REST API.
///
/// Effectively it's an HTTP REST client.
impl BleBox {
    pub fn new(host: String) -> Self {
        Self {
            client: Client::new(),
            host,
        }
    }
}

/// Handles BleBox dimmer box.
///
/// Based on https://technical.blebox.eu/archives/dimmerBoxAPI/.
#[async_trait]
impl Lights for BleBox {
    /// Sets light brightness.
    ///
    /// # Examples
    ///
    /// ```
    /// // 128 is half of the range for BleBox implementation
    /// lights.brightness(128).await?
    /// ```
    async fn brightness(&self, level: i32) -> Result<(), LightsError> {
        info!(target: "lights:blebox", "Setting brightness to {}", level);

        self.client
            .post(format!("http://{}/api/dimmer/set", self.host))
            .json(&json!({
                "dimmer": {
                    "desiredBrightness": level,
                }
            }))
            .send()
            .await
            .map(|_| ())
            .map_err(|_| LightsError::BrightnessError)
    }
}
