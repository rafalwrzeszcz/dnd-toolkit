use dbus::blocking::Connection;
use dbus::channel::Sender;
use dbus::message::Message;

use crate::audio::Audio;

pub struct Spotify {
    dbus: Connection,
}

impl Spotify {
    pub fn new() -> Self {
        Self {
            dbus: Connection::new_session().unwrap(), // TODO
        }
    }
}

impl Audio for Spotify {
    fn play(&self, track: String) {
        let call = Message::call_with_args(
            "org.mpris.MediaPlayer2.spotify",
            "/org/mpris/MediaPlayer2",
            "org.mpris.MediaPlayer2.Player",
            "OpenUri",
            (track,),
        );

        self.dbus.send(call); // TODO
    }
}
