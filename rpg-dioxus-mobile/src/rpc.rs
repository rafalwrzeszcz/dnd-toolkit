use async_trait::async_trait;
use rpg_core::audio::{Audio, AudioError};
use rpg_core::context::AppContext;
use rpg_core::lights::{Lights, LightsError};
use tokio::sync::Mutex;
use tonic::transport::{Channel, Error as TonicError};
use tonic::{include_proto, Request, Response, Status};
use tracing::{error, info_span, Instrument};

include_proto!("rpg");

// client side

/// gRPC client that delegates local actions to remote nodes.
pub struct Rpc {
    audio: Mutex<audio_client::AudioClient<Channel>>,
    lights: Mutex<lights_client::LightsClient<Channel>>,
}

impl Rpc {
    /// Configures gRPC client with URL to target nodes.
    ///
    /// # Arguments
    ///
    /// * `url` - RPC URL to node that will handle audio subsystem.
    pub async fn new(url: String) -> Result<Self, TonicError> {
        Ok(Self {
            audio: Mutex::new(audio_client::AudioClient::connect(url.clone()).await?),
            lights: Mutex::new(lights_client::LightsClient::connect(url.clone()).await?),
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

/// Delegates lights calls to remote node through RPC calls.
#[async_trait]
impl Lights for Rpc {
    async fn brightness(&self, level: i32) -> Result<(), LightsError> {
        let call = Request::new(BrightnessRequest { level });

        if let Err(status) = self
            .lights
            .lock()
            .await
            .brightness(call)
            .instrument(info_span!(target: "lights:rpc", "brightness", "Setting {}", level))
            .await
        {
            error!(target: "lights:rpc", "Brightness operation caused {:?}", status);
            return Err(LightsError::BrightnessError);
        }
        Ok(())
    }
}

// service side

#[async_trait]
impl audio_server::Audio for AppContext {
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

#[async_trait]
impl lights_server::Lights for AppContext {
    async fn brightness(&self, request: Request<BrightnessRequest>) -> Result<Response<BrightnessResponse>, Status> {
        let request = request.into_inner();

        match self
            .lights
            .brightness(request.level)
            .instrument(info_span!(target: "rpc:lights", "brightness", "Handling brightness request {}", request.level))
            .await
        {
            Ok(()) => Ok(Response::new(BrightnessResponse {})),
            Err(error) => Err(Status::internal(format!("Brightness error: {:?}", error))),
        }
    }
}
