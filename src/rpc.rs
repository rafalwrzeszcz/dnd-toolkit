use std::cell::RefCell;
use tokio::runtime::Runtime;
use tonic::{include_proto, Request};
use tonic::transport::Channel;

use crate::audio::Audio;

include_proto!("rpg");

pub struct Rpc {
    audio: RefCell<audio_client::AudioClient<Channel>>,
    runtime: Runtime,
}

impl Rpc {
    pub fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        
        Self {
            audio: RefCell::new(runtime.block_on(audio_client::AudioClient::connect("http://127.0.0.1:50051")).unwrap()), // TODO
            runtime,
        }
    }
}

impl Audio for Rpc {
    fn play(&self, track: String) {
        let call = Request::new(PlayRequest {
            track,
        });

        self.runtime.block_on(self.audio.borrow_mut().play(call)); // TODO
    }
}

// TODO: listener to separate file

pub struct Listener<T: Audio> {
    audio: std::sync::Arc<std::sync::Mutex<T>>,
}

impl<T: Audio> Listener<T> {
    pub fn new(audio: T) -> Self {
        Self {
            audio: std::sync::Arc::new(std::sync::Mutex::new(audio)),
        }
    }
}

#[tonic::async_trait]
impl<T: Audio + std::marker::Send + 'static> audio_server::Audio for Listener<T> {
    async fn play(&self, request: Request<PlayRequest>) -> Result<tonic::Response<PlayResponse>, tonic::Status> {
        println!("{:?}", request);
        self.audio.clone().lock().unwrap().play(request.into_inner().track);
        Ok(tonic::Response::new(PlayResponse {}))
        // TODO
    }
}

/* TODO: rest? *rpc?
pub struct Remote {
    // TODO
}

impl Audio for Remote {
    // TODO
}
*/
