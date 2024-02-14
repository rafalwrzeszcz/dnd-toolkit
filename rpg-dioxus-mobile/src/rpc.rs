use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Error as TonicError};
use tonic::{include_proto, Request, Response, Status};
use tracing::{error, info_span, Instrument};

use crate::audio::{Audio, AudioError};

include_proto!("rpg");

// client side

/// gRPC client that delegates local actions to remote nodes.
pub struct Rpc {
    audio: Mutex<audio_client::AudioClient<Channel>>,
}

impl Rpc {
    /// Configures gRPC client with URL to target nodes.
    ///
    /// # Arguments
    ///
    /// * `url` - RPC URL to node that will handle audio subsystem.
    pub async fn new(url: String) -> Result<Self, TonicError> {
        Ok(Self {
            audio: Mutex::new(audio_client::AudioClient::connect(url).await?),
        })
    }
}

/// Delegates audio calls to remote node through RPC calls.
#[async_trait]
impl Audio for Rpc {
    async fn play(&self, track: String) -> Result<(), AudioError> {
        let call = Request::new(PlayRequest { track: track.clone() });

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
    }
}

// service side

/// gRPC listener that forwards accepted calls to underlying subystems locally.
pub struct Listener {
    audio: Arc<dyn Audio + Send + Sync + 'static>,
}

impl Listener {
    /// Constructs RPC handler by specifying underlying local systems.
    ///
    /// # Arguments
    ///
    /// * `audio` - Local audio subsystem handler.
    pub fn new(audio: Arc<dyn Audio + Send + Sync + 'static>) -> Self {
        Self { audio }
    }
}

#[async_trait]
impl audio_server::Audio for Listener {
    async fn play(&self, request: Request<PlayRequest>) -> Result<Response<PlayResponse>, Status> {
        let request = request.into_inner();

        match self
            .audio
            .play(request.track.clone())
            .instrument(info_span!(target: "rpc:audio", "play", "Handling request to play {}", request.track))
            .await
        {
            Ok(()) => Ok(Response::new(PlayResponse {})),
            Err(error) => Err(Status::internal(format!("Play error: {:?}", error))),
        }
    }
}
