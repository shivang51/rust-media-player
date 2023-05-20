use std::ffi::CString;
use std::ptr::{null_mut};
use super::utils::{MediaFile};
use ffmpeg_sys_next as ffmpeg;
use ffmpeg_sys_next::{AVDictionaryEntry, AVStream};

pub struct Decoder{
    pub file: MediaFile,
    fmt: Box<*mut ffmpeg::AVFormatContext>,
}

impl Default for Decoder{
    fn default() -> Self {
        return Decoder{
            file: Default::default(),
            fmt: Box::new(std::ptr::null_mut()),
        };
    }
}

impl Decoder{

    pub fn init(&self){
        unsafe{
            ffmpeg::avdevice_register_all();
        }
        println!("[!] ffmpeg initialized");
    }

    pub fn open(&mut self, path: String) -> bool {

        let url = std::ffi::CString::new(path.clone()).unwrap();

        let ret = unsafe{
            ffmpeg::avformat_open_input(
                self.fmt.as_mut(),
                url.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut()
            )
        };

        if ret != 0{
            panic!("Cannot open a file");
        }


        self.file.open(path);

        return true;
    }

    pub fn close(&mut self){
        unsafe{
            ffmpeg::avformat_close_input(self.fmt.as_mut());
        }
    }

    pub fn terminate(&mut self){
        if !self.fmt.is_null() {
            self.close();
        }
    }

    pub unsafe fn get_video_stream(&mut self){
        let fmt = self.fmt.as_mut().as_mut().unwrap();
        for i in 0..fmt.nb_streams.to_owned(){

            let stream = *(*fmt.streams).offset(i as isize);
                // println!("{}", stream.codecpar);
            // self.dump_stream_info(&stream);
        }
    }

    pub unsafe fn dump_stream_info(&self, stream: &AVStream){
        if stream.codecpar.is_null() {
            panic!("invalid stream");
        }

        let mut dict_entry: *mut AVDictionaryEntry = null_mut();

        let c_str = CString::new("").unwrap();
        dict_entry = ffmpeg::av_dict_get(stream.metadata,
                                          c_str.as_ptr(),
                                          dict_entry,
                                          ffmpeg::AV_DICT_IGNORE_SUFFIX
        );
        while !dict_entry.is_null() {
            let key = CString::from_raw((*dict_entry).key);
            let value = CString::from_raw((*dict_entry).value);
            println!("{} --> {}", key.into_string().unwrap(), value.into_string().unwrap());
            dict_entry = ffmpeg::av_dict_get(stream.metadata,
                                             c_str.as_ptr(),
                                             dict_entry,
                                             ffmpeg::AV_DICT_IGNORE_SUFFIX);
        }

    }
}

impl Drop for Decoder{
    fn drop(&mut self) {
        self.terminate();
        println!("[!] ffmpeg terminated");
    }
}


