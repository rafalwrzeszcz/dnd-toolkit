#[cxx_qt::bridge]
mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Audio = super::AudioRust;

        #[qinvokable]
        pub fn play_audio(self: &Audio);
    }
}

#[derive(Default)]
pub struct AudioRust {}

impl qobject::Audio {
    pub fn play_audio(&self) {
        audio
            .play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into())
            .await
            .unwrap();
    }
}
