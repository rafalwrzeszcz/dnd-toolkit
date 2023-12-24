use crate::audio::Audio;

pub struct Void {
}

impl Audio for Void {
    fn play(&self, _track: String) {}
}
