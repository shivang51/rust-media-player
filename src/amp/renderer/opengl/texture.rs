use std::ffi::c_void;

use imgui::TextureId;

pub struct Texture {
    id: TextureId,
}

impl Default for Texture {
    fn default() -> Self {
        Texture {
            id: TextureId::new(0),
        }
    }
}

impl Texture {
    pub fn new() -> Texture {
        let mut id: u32 = 0;
        let p_id: *mut u32 = &mut id;
        unsafe {
            gl::GenTextures(1, p_id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
        Texture {
            id: TextureId::new(id as usize),
        }
    }

    pub fn get_id(&mut self) -> TextureId {
        self.id
    }

    pub fn bind(&mut self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id.id() as u32) };
    }

    pub fn send_data(&mut self, width: i32, height: i32, data: Vec<u8>) {
        self.bind();

        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                width,
                height,
                gl::RGB32UI,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl::GetError();
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            let ptr: *mut u32 = &mut (self.id.id() as u32);
            gl::DeleteTextures(1, ptr);
        }
    }
}
