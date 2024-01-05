use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Error as TonicError};
use tonic::{include_proto, Request, Response, Status};
use tracing::{error, Instrument, info_span};

use crate::audio::{Audio, AudioError};

include_proto!("rpg");

// client side

pub struct Rpc {
    audio: Mutex<audio_client::AudioClient<Channel>>,
}

impl Rpc {
    pub async fn new(audio_url: String) -> Result<Self, TonicError> {
        Ok(Self {
            audio: Mutex::new(audio_client::AudioClient::connect(audio_url).await?),
        })
    }
}

#[async_trait]
impl Audio for Rpc {
    async fn play(&self, track: String) -> Result<(), AudioError> {
        let call = Request::new(PlayRequest { track: track.clone() });

        if let Err(status) = self.audio.lock()
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

pub struct Listener {
    audio: Arc<dyn Audio + Send + Sync + 'static>,
}

impl Listener {
    pub fn new(audio: Arc<dyn Audio + Send + Sync + 'static>) -> Self {
        Self { audio }
    }
}

#[async_trait]
impl audio_server::Audio for Listener {
    async fn play(&self, request: Request<PlayRequest>) -> Result<Response<PlayResponse>, Status> {
        let request = request.into_inner();

        match self.audio.play(request.track.clone())
            .instrument(info_span!(target: "rpc:audio", "play", "Handling request to play {}", request.track))
            .await
        {
            Ok(()) => Ok(Response::new(PlayResponse {})),
            Err(error) => Err(Status::internal(format!("Play error: {:?}", error))),
        }
    }
}
