use gl;
use gl::types::*;
use image;
use std::mem;

pub struct Texture {
    id: u32,
}

impl Texture {
    const LEVEL_OF_DETAIL: GLint = 0;

    pub fn new(image: image::RgbaImage) -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(gl::TEXTURE_2D, Texture::LEVEL_OF_DETAIL, gl::RGBA as i32,
                           image.width() as i32, image.height() as i32, 0, gl::RGBA,
                           gl::UNSIGNED_BYTE, mem::transmute(&image.into_raw()[0]));
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture {
            id,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id); }
    }
}
