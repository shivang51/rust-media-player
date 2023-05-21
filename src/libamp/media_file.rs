use super::decoder::{Decoder, IS_DECODER_INITIALIZED};

pub struct MediaFile {
    name: String,
    path: String,
    is_open: bool,
    pub decoder: Decoder,
}

impl Default for MediaFile {
    fn default() -> Self {
        return MediaFile {
            name: String::from(""),
            path: String::from(""),
            is_open: false,
            decoder: Default::default(),
        };
    }
}

impl MediaFile {
    pub fn open(&mut self, path: String) {
        if !unsafe { IS_DECODER_INITIALIZED } {
            Decoder::avdevice_register();
        }
        self.name = path
            .split("\\")
            .into_iter()
            .last()
            .unwrap_or("")
            .to_string();
        self.decoder.open(path.clone());
        self.path = path;
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.decoder.close();
    }

    pub fn is_open(&mut self) -> bool {
        self.is_open
    }
}

impl Drop for MediaFile {
    fn drop(&mut self) {
        self.close();
    }
}
