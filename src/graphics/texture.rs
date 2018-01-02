use gl;
use gl::types::*;
use image;
use std::fs::File;
use std::io::BufReader;
use std::mem;

use graphics::shader::ProgramUniforms;

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

    pub fn update(&mut self, image: image::RgbaImage) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexSubImage2D(gl::TEXTURE_2D, Texture::LEVEL_OF_DETAIL, 0, 0,
                              image.width() as i32, image.height() as i32, gl::RGBA,
                              gl::UNSIGNED_BYTE, mem::transmute(&image.into_raw()[0]));
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind_and_send(&self, uniform_name: &str, uniforms: &mut ProgramUniforms) {
        unsafe {
            // TODO: Make this class not just for diffuse textures.
            // Use 0th texture unit for diffuse textures by convention.
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            uniforms.send_1i("use_texture", 1);
            uniforms.send_1i(uniform_name, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}

impl<'a> From<&'a str> for Texture {
    fn from(file_name: &str) -> Self {
        let image = image::load(
            BufReader::new(File::open(file_name).unwrap()),
            image::JPEG,
        ).unwrap().to_rgba();
        Texture::new(image)
    }
}
