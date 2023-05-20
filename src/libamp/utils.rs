use ffmpeg_sys_next as ffmpeg;

pub struct MediaFile{
    pub name: String,
    pub path: String,
    pub is_open: bool,
}

impl Default for MediaFile{
    fn default() -> Self {
        return MediaFile{
            name: String::from(""),
            path: String::from(""),
            is_open: false
        }
    }
}

impl MediaFile {
    pub fn open(&self, path: String) -> bool {


        return true;
    }
}