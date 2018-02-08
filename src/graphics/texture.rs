
use gl;
use gl::types::*;
use image;
use image::RgbaImage;
use std::fs::File;
use std::io::BufReader;
use std::{mem, ptr};

use graphics::shader::ProgramUniforms;

/// Describes how a texture will be used.  Note that for each texture type, only one texture of that
/// type can be bound at a time.
#[derive(Clone, Debug, PartialEq)]
pub enum TextureType {
    Diffuse,
    CubeMap,
}

impl TextureType {
    pub fn texture_index(&self) -> i32 {
        use self::TextureType::*;
        match *self {
            Diffuse => 0,
            CubeMap => 1,
        }
    }

    pub fn texture_unit(&self) -> GLenum {
        use self::TextureType::*;
        match *self {
            Diffuse => gl::TEXTURE0,
            CubeMap => gl::TEXTURE1,
        }
    }

    pub fn bind_target(&self) -> GLenum {
        use self::TextureType::*;
        match *self {
            Diffuse => gl::TEXTURE_2D,
            CubeMap => gl::TEXTURE_CUBE_MAP,
        }
    }
}

pub struct Texture {
    id: u32,
    texture_type: TextureType,
}

impl Texture {
    const LEVEL_OF_DETAIL: GLint = 0;
    const CUBE_MAP_LAYERS: [u32; 6] = [
        gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
    ];

    pub fn new(texture_type: TextureType, dimensions: Option<(u32, u32)>) -> Texture {
        use self::TextureType::*;

        let mut id = 0;
        let bind_target = texture_type.bind_target();
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(bind_target, id);

            // Bind and perform any setup specific to the texture type.
            match texture_type {
                Diffuse => (),
                CubeMap => {
                    let dimensions = dimensions.expect("Must supply dimensions with cube maps");

                    for layer in Self::CUBE_MAP_LAYERS.iter() {
                        gl::TexImage2D(*layer, 0, gl::DEPTH_COMPONENT32 as i32,
                                       dimensions.0 as i32,
                                       dimensions.1 as i32,
                                       0, gl::DEPTH_COMPONENT, gl::FLOAT, ptr::null());
                    }

                    gl::TexParameteri(bind_target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(bind_target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(bind_target, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
                },
            }

            // We always want nearest neighbor interpolation.
            gl::TexParameteri(bind_target, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(bind_target, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::BindTexture(bind_target, 0);
        }
        Texture {
            id,
            texture_type,
        }
    }

    /// Generates a 1x1 white texture.
    pub fn empty() -> Texture {
        const WHITE_PIXELS: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
        Texture::from_image(RgbaImage::from_raw(1, 1, WHITE_PIXELS.to_vec()).unwrap())
    }

    /// Generates a diffuse texture from `image`.
    pub fn from_image(image: RgbaImage) -> Texture {
        let texture = Texture::new(TextureType::Diffuse, None);
        let bind_target = texture.texture_type().bind_target();
        unsafe {
            gl::BindTexture(bind_target, texture.id());
            gl::TexImage2D(bind_target, Texture::LEVEL_OF_DETAIL, gl::RGBA as i32,
                           image.width() as i32, image.height() as i32, 0, gl::RGBA,
                           gl::UNSIGNED_BYTE, mem::transmute(&image.into_raw()[0]));
            gl::BindTexture(bind_target, 0);
        }
        texture
    }

    pub fn update(&mut self, image: RgbaImage) {
        if self.texture_type != TextureType::Diffuse {
            panic!("Can't use this method for \"{:?}\"", self.texture_type);
        }
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
            gl::ActiveTexture(self.texture_type.texture_unit());
            gl::BindTexture(self.texture_type.bind_target(), self.id);
            uniforms.send_1i(uniform_name, self.texture_type.texture_index());
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn texture_type(&self) -> TextureType {
        self.texture_type.clone()
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
        Texture::from_image(image)
    }
}
