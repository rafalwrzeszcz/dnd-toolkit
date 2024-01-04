use async_trait::async_trait;
use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Error as TonicError};
use tonic::{include_proto, Request, Response, Status};

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
        info!(target: "audio:rpc", "Playing {}", track);

        let call = Request::new(PlayRequest { track });

        if let Err(status) = self.audio.lock().await.play(call).await {
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

        info!(target: "rpc:audio", "Handling request to play {}", request.track);

        match self.audio.play(request.track).await {
            Ok(()) => Ok(Response::new(PlayResponse {})),
            Err(error) => Err(Status::internal(format!("Play error: {:?}", error))),
        }
    }
}
