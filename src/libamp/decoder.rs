use ffmpeg::{AVCodecContext, AVFrame, AVPacket};
use ffmpeg_sys_next as ffmpeg;
use ffmpeg_sys_next::{AVDictionaryEntry, AVStream};
use std::ffi::CString;
use std::ptr::{null, null_mut};

struct MediaStream {
    stream: *const AVStream,
    decoder: *mut AVCodecContext,
}

impl Default for MediaStream {
    fn default() -> Self {
        return MediaStream {
            stream: null(),
            decoder: null_mut(),
        };
    }
}

pub struct Decoder {
    fmt: Box<*mut ffmpeg::AVFormatContext>,
    name: String,
    video_stream: MediaStream,
    _audio_stream: MediaStream,
}

impl Default for Decoder {
    fn default() -> Self {
        return Decoder {
            fmt: Box::new(std::ptr::null_mut()),
            name: String::from(""),
            video_stream: Default::default(),
            _audio_stream: Default::default(),
        };
    }
}

pub static mut IS_DECODER_INITIALIZED: bool = false;

impl Decoder {
    /// done only once, don't call again
    pub fn avdevice_register() {
        unsafe {
            assert_ne!(IS_DECODER_INITIALIZED, true, "dec");
            ffmpeg::avdevice_register_all();
            IS_DECODER_INITIALIZED = true;
        }
        println!("[!] ffmpeg initialized");
    }

    pub fn open(&mut self, path: String) -> bool {
        let url = std::ffi::CString::new(path.clone()).unwrap();

        let ret = unsafe {
            ffmpeg::avformat_open_input(
                self.fmt.as_mut(),
                url.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        if ret != 0 {
            panic!("Cannot open a file");
        }

        self.name = path;

        self.video_stream.stream = self.get_video_stream();
        unsafe {
            self.video_stream.decoder = self.get_stream_decoder(self.video_stream.stream);
        }

        return true;
    }

    pub fn close(&mut self) {
        if !self.fmt.is_null() {
            unsafe {
                ffmpeg::avformat_close_input(self.fmt.as_mut());
            }
        }
    }

    fn get_video_stream(&mut self) -> *const AVStream {
        unsafe {
            let fmt = self.fmt.as_mut().as_mut().unwrap();
            for i in 0..fmt.nb_streams.to_owned() {
                let stream = *(fmt.streams).offset(i as isize);
                if (*(*stream).codecpar).codec_type == ffmpeg::AVMediaType::AVMEDIA_TYPE_VIDEO {
                    self.dump_stream_info(stream);
                    return stream;
                }
            }
        }

        return null();
    }

    unsafe fn get_stream_decoder(&mut self, stream: *const AVStream) -> *mut AVCodecContext {
        let codec = ffmpeg::avcodec_find_decoder((*(*stream).codecpar).codec_id);

        let decoder = ffmpeg::avcodec_alloc_context3(codec);

        ffmpeg::avcodec_parameters_to_context(decoder, (*stream).codecpar);
        ffmpeg::avcodec_open2(decoder, codec, null_mut());

        return decoder;
    }

    fn seek_to(&mut self, mut time: f64) {
        unsafe {
            time = time / ffmpeg::av_q2d((*self.video_stream.stream).time_base);
            ffmpeg::av_seek_frame(
                self.fmt.as_mut().as_mut().unwrap(),
                (*self.video_stream.stream).index,
                time as i64,
                ffmpeg::AVSEEK_FLAG_ANY,
            );
        }
    }

    pub fn get_video_frame(&mut self, time: f64) -> AVFrame {
        self.seek_to(time);

        let mut got_frame = false;
        unsafe {
            let frame = ffmpeg::av_frame_alloc();
            let packet = ffmpeg::av_packet_alloc();
            while !got_frame {
                ffmpeg::av_read_frame(self.fmt.as_mut().as_mut().unwrap(), packet);
                if (*packet).stream_index == (*self.video_stream.stream).index {
                    if self.decode_video_packet(packet, frame) {
                        got_frame = true;
                    }
                }
            }
            let f = *frame;
            ffmpeg::av_frame_unref(frame);
            ffmpeg::av_packet_unref(packet);
            return f;
        }
    }

    unsafe fn decode_video_packet(&mut self, packet: *mut AVPacket, frame: *mut AVFrame) -> bool {
        ffmpeg::avcodec_send_packet(self.video_stream.decoder, packet);
        let err = ffmpeg::avcodec_receive_frame(self.video_stream.decoder, frame);

        if err == ffmpeg::AVERROR(ffmpeg::EAGAIN) {
            return false;
        }

        return true;
    }

    pub fn dump_stream_info(&self, stream_: *const AVStream) {
        let stream: AVStream;
        unsafe {
            stream = *stream_;
        }
        if stream.codecpar.is_null() {
            panic!("invalid stream");
        }

        let mut dict_entry: *mut AVDictionaryEntry = null_mut();

        let c_str = CString::new("").unwrap();
        unsafe {
            dict_entry = ffmpeg::av_dict_get(
                stream.metadata,
                c_str.as_ptr(),
                dict_entry,
                ffmpeg::AV_DICT_IGNORE_SUFFIX,
            );
        }

        while !dict_entry.is_null() {
            unsafe {
                let key = CString::from_raw((*dict_entry).key);
                let value = CString::from_raw((*dict_entry).value);

                println!(
                    "{} --> {}",
                    key.clone().into_string().unwrap(),
                    value.clone().into_string().unwrap()
                );
                (*dict_entry).key = key.into_raw();
                (*dict_entry).value = value.into_raw();
                dict_entry = ffmpeg::av_dict_get(
                    stream.metadata,
                    c_str.as_ptr(),
                    dict_entry,
                    ffmpeg::AV_DICT_IGNORE_SUFFIX,
                );
            }
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        self.close();
        println!("[!] decoder closed: {}", self.name);
    }
}
