use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{include_proto, Request, Response, Status};
use tonic::transport::Channel;

use crate::audio::Audio;

include_proto!("rpg");

pub struct Rpc {
    audio: Arc<Mutex<audio_client::AudioClient<Channel>>>,
}

impl Rpc {
    pub async fn new() -> Self {
        Self {
            audio: Arc::new(Mutex::new(audio_client::AudioClient::connect("http://127.0.0.1:50051").await.unwrap())), // TODO
        }
    }
}

#[async_trait]
impl Audio for Rpc {
    async fn play(&self, track: String) {
        let call = Request::new(PlayRequest {
            track,
        });

        self.audio.clone().lock().await.play(call).await; // TODO
    }
}

// TODO: listener to separate file

pub struct Listener<T: Audio> {
    audio: Arc<Mutex<T>>,
}

impl<T: Audio> Listener<T> {
    pub fn new(audio: T) -> Self {
        Self {
            audio: Arc::new(Mutex::new(audio)),
        }
    }
}

#[async_trait]
impl<T: Audio + std::marker::Send + 'static> audio_server::Audio for Listener<T> {
    async fn play(&self, request: Request<PlayRequest>) -> Result<Response<PlayResponse>, Status> {
        self.audio.clone().lock().await.play(request.into_inner().track).await;
        Ok(Response::new(PlayResponse {}))
        // TODO
    }
}

/* TODO: rpc -> Remote */
