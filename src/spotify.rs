use crate::audio::{Audio, AudioError};
use async_trait::async_trait;
use dbus::channel::Sender;
use dbus::message::Message;
use dbus::nonblock::SyncConnection;
use dbus::Error as DbusError;
use dbus_tokio::connection::new_session_sync;
use std::sync::Arc;
use tokio::spawn;
use tracing::{error, info};

pub struct Spotify {
    dbus: Arc<SyncConnection>,
}

/// Spotify handler over D-Bus.
///
/// Effectively it's a D-Bus client, that sends media control commands to Spotify client app.
///
/// **Note:** This implementation simply sends D-Bus commands, there is no way it can detect if messages are being
/// handled successfully. If there is no Spotify app running on the same D-Bus bus to consume messages it will just
/// continue to work with no effect.
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

/// Handles Spotify audio media.
#[async_trait]
impl Audio for Spotify {
    /// Plays Spotify media reference.
    ///
    /// # Examples
    ///
    /// ```
    /// audio.play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into()).await?
    /// ```
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
