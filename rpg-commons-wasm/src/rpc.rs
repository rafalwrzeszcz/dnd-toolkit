use async_trait::async_trait;
use futures::executor::block_on;
use rpg_core::audio::{Audio, AudioError};
use tokio::sync::Mutex;
use tonic::{include_proto, Request};
use tonic_web_wasm_client::Client;
use tracing::{error, info_span, Instrument};

include_proto!("rpg");

// TODO: check if HTTP2 is possible?

// client side

/// gRPC client that delegates local actions to remote nodes.
pub struct Rpc {
    audio: Mutex<audio_client::AudioClient<Client>>,
}

impl Rpc {
    /// Configures gRPC client with URL to target nodes.
    ///
    /// # Arguments
    ///
    /// * `url` - RPC URL to node that will handle audio subsystem.
    pub fn new(url: String) -> Self {
        Self {
            audio: Mutex::new(audio_client::AudioClient::new(Client::new(url))),
        }
    }
}

/// Delegates audio calls to remote node through RPC calls.
#[async_trait]
impl Audio for Rpc {
    async fn play(&self, track: String) -> Result<(), AudioError> {
        let call = Request::new(PlayRequest { track: track.clone() });

        block_on(async {
            if let Err(status) = self
                .audio
                .lock()
                .await
                .play(call)
                .instrument(info_span!(target: "audio:rpc", "play", "Playing {}", track))
                .await
            {
                error!(target: "audio:rpc", "Play operation caused {:?}", status);
                return Err(AudioError::PlayError);
            }

            Ok(())
        })
    }
}
