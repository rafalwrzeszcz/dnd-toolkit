use async_trait::async_trait;
use dbus::channel::Sender;
use dbus::message::Message;
use dbus::nonblock::SyncConnection;
use dbus_tokio::connection::new_session_sync;
use std::sync::Arc;
use tokio::spawn;
use crate::audio::Audio;

pub struct Spotify {
    dbus: Arc<SyncConnection>,
}

impl Spotify {
    pub fn new() -> Self {
        let (resource, conn) = new_session_sync().unwrap();

        // TODO
        spawn(async {
            let err = resource.await;
            panic!("Lost connection to D-Bus: {}", err);
        });

        Self {
            dbus: conn, // TODO
        }
    }
}

#[async_trait]
impl Audio for Spotify {
    async fn play(&self, track: String) {
        let call = Message::call_with_args(
            "org.mpris.MediaPlayer2.spotify",
            "/org/mpris/MediaPlayer2",
            "org.mpris.MediaPlayer2.Player",
            "OpenUri",
            (track,),
        );

        &self.dbus.send(call); // TODO
    }
}
