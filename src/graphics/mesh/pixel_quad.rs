use image::RgbaImage;

use graphics::mesh::Mesh;
use graphics::mesh::util::*;
use graphics::Render;
use graphics::renderer::RenderingContext;
use graphics::texture::Texture;
use util::math::Point3;

/// Stores an array of pixels that, when updated, generates a texture that is then drawn
/// as a quad with the same aspect ratio as the texture.
pub struct PixelQuad {
    mesh: Mesh,
    dimensions: (u32, u32),
    pixels: Vec<u8>,
}

impl PixelQuad {
    /// Returns a new `PixelQuad`.
    ///
    /// # Arguments
    ///
    /// * `size` - A size of 1.0 constrains the width to be 1.0 and the height to be
    ///            constrained by the width and the aspect ratio.
    pub fn new(dimensions: (u32, u32), size: f32, position: Point3) -> PixelQuad {
        assert!(dimensions.0 > 0);
        assert!(dimensions.1 > 0);
        assert!(size > 0.0);
        let mut pixels = vec![0; (4 * dimensions.0 * dimensions.1) as usize];
        for i in 0..pixels.len() {
            if i % 4 == 0 {
                pixels[i] = 0x42;
            } else if i % 4 == 1 {
                pixels[i] = 0x86;
            } else if i % 4 == 2 {
                pixels[i] = 0xf4;
            } else if i % 4 == 3 {
                pixels[i] = 0xff;
            }
        }
        let diffuse_texture = Self::gen_texture(dimensions, &pixels);
        let mut mesh = Self::gen_mesh(dimensions, size, diffuse_texture);
        mesh.move_to(position);
        PixelQuad {
            mesh,
            dimensions,
            pixels,
        }
    }

    pub fn update(&mut self) {
        let image = RgbaImage::from_raw(self.dimensions.0, self.dimensions.1,
                                        self.pixels.clone()).unwrap();
        match self.mesh.diffuse_texture {
            Some(ref mut dt) => dt.update(image),
            None => panic!("No diffuse texture found on mesh"),
        };
    }

    fn gen_texture(dimensions: (u32, u32), pixels: &Vec<u8>) -> Texture {
        Texture::new(RgbaImage::from_raw(dimensions.0, dimensions.1,
                                         pixels.clone()).unwrap())
    }

    fn gen_mesh(dimensions: (u32, u32), size: f32, diffuse_texture: Texture) -> Mesh {
        let aspect_ratio = dimensions.0 as f32 / dimensions.1 as f32;
        let w = size / 2.0;
        let h = (size / aspect_ratio) / 2.0;
        let base_positions = vec![
            -w, -h, 0.0,
            w, -h, 0.0,
            w, h, 0.0,
            -w, h, 0.0,
        ];

        let indices = vec![
            0, 1, 2,
            0, 2, 3,
        ];

        let positions = expand_indices(&base_positions, &indices);
        let normals = generate_normals(&positions);

        let tex_coords = vec![
            // Lower right triangle
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            // Upper left triangle
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ];

        Mesh::new(positions, Some(normals), Some(tex_coords), Some(diffuse_texture))
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn pixels(&mut self) -> &mut Vec<u8> {
        &mut self.pixels
    }
}

impl Render for PixelQuad {
    fn render(&mut self, context: &mut RenderingContext) {
        self.mesh.render(context);
    }
}
