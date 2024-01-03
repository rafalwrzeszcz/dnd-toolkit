use crate::audio::{Audio, AudioError};
use async_trait::async_trait;
use dbus::channel::Sender;
use dbus::message::Message;
use dbus::nonblock::SyncConnection;
use dbus::Error as DbusError;
use dbus_tokio::connection::new_session_sync;
use log::{error, info};
use std::sync::Arc;
use tokio::spawn;

pub struct Spotify {
    dbus: Arc<SyncConnection>,
}

impl Spotify {
    pub fn new() -> Result<Self, DbusError> {
        let (resource, conn) = new_session_sync()?;

        spawn(async {
            // TODO: reconnect?
            let err = resource.await;
            error!("D-Bus connection lost.");
            panic!("Lost connection to D-Bus: {}", err);
        });

        Ok(Self { dbus: conn })
    }
}

#[async_trait]
impl Audio for Spotify {
    async fn play(&self, track: String) -> Result<(), AudioError> {
        info!(target: "audio:spotify", "Playing {}", track);

        let call = Message::call_with_args(
            "org.mpris.MediaPlayer2.spotify",
            "/org/mpris/MediaPlayer2",
            "org.mpris.MediaPlayer2.Player",
            "OpenUri",
            (track,),
        );

        if let Err(()) = &self.dbus.send(call) {
            return Err(AudioError::PlayError);
        }
        Ok(())
    }
}
