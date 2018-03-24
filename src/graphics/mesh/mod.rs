pub mod cube;
pub mod obj;
pub mod rect;
pub mod tetrahedron;
pub mod pixel_quad;

use gl;
use gl::types::*;
use std::mem;
use std::ptr;

use graphics::Render;
use graphics::texture::Texture;
use util::math::{Vector3, Transformation};
use world::RenderConfig;


struct VertexArray {
    pub vbo_id: GLuint,
    pub data: Vec<GLfloat>,
}

struct IndexArray {
    pub vbo_id: GLuint,
    pub data: Vec<u32>,
}

/// Used to establish and utilize attribute location conventions.
///
/// `ShaderProgram::setup_attrib_locs` should be updated whenever this enum is.
pub enum AttribIndices {
    Positions = 0,
    Normals = 1,
    TexCoords = 2,
}


pub struct Mesh {
    vao_id: GLuint,
    positions: VertexArray,
    indices: Option<IndexArray>,
    normals: Option<VertexArray>,
    tex_coords: Option<VertexArray>,
    transformation: Transformation,
    diffuse_texture: Option<Texture>,
    // Need this for when we bind an empty texture.
    textured: bool,
}

impl Mesh {
    pub fn new(positions: Vec<GLfloat>,
               indices: Option<Vec<u32>>,
               normals: Option<Vec<GLfloat>>,
               tex_coords: Option<Vec<GLfloat>>,
               diffuse_texture: Option<Texture>) -> Mesh {
        assert_eq!(tex_coords.is_some(), diffuse_texture.is_some());

        let mut vao_id = 0;

        let p;
        let mut i = None;
        let mut n = None;
        let mut t = None;

        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);

            p = VertexArray {
                vbo_id: Self::gen_vbo(AttribIndices::Positions as GLuint, 3, &positions),
                data: positions
            };
            if let Some(ii) = indices {
                i = Some(IndexArray {
                    vbo_id: Self::gen_index_vbo(&ii),
                    data: ii,
                });
            }
            if let Some(nn) = normals {
                n = Some(VertexArray {
                    vbo_id: Self::gen_vbo(AttribIndices::Normals as GLuint, 3, &nn),
                    data: nn
                });
            }
            if let Some(tt) = tex_coords {
                t = Some(VertexArray {
                    vbo_id: Self::gen_vbo(AttribIndices::TexCoords as GLuint, 2, &tt),
                    data: tt
                });
            }

            // Unbind everything.
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        // TODO: There has to be a better way to do this.
        // Use a white texture if no texture is given.  This way we can use the same
        // shader for textured and untextured objects.
        let textured;
        let diffuse_texture = match diffuse_texture {
            Some(dt) => {
                textured = true;
                Some(dt)
            },
            None => {
                textured = false;
                Some(Texture::empty())
            },
        };

        Mesh {
            vao_id,
            positions: p,
            indices: i,
            normals: n,
            tex_coords: t,
            transformation: Transformation::new(),
            diffuse_texture,
            textured,
        }
    }

    unsafe fn gen_vbo(attrib_index: GLuint, num_components: GLint,
                      data: &Vec<GLfloat>) -> u32 {
        let mut vbo_id = 0;
        gl::GenBuffers(1, &mut vbo_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&data[0]),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(attrib_index, num_components, gl::FLOAT, gl::FALSE, 0,
                                ptr::null());
        vbo_id
    }

    unsafe fn gen_index_vbo(indices: &Vec<u32>) -> u32 {
        let mut vbo_id = 0;
        gl::GenBuffers(1, &mut vbo_id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<u32>()) as GLsizeiptr,
            mem::transmute(&indices[0]),
            gl::STATIC_DRAW,
        );
        vbo_id
    }

    pub fn positions(&self) -> &Vec<GLfloat> {
        &self.positions.data
    }

    pub fn diffuse_texture(&mut self) -> &mut Option<Texture> {
        &mut self.diffuse_texture
    }

    pub fn set_diffuse_texture(&mut self, diffuse_texture: Texture) {
        self.diffuse_texture = Some(diffuse_texture);
    }

    /// Use this to manipulate the mesh's translation, rotation, and scaling.
    pub fn transformation(&mut self) -> &mut Transformation {
        &mut self.transformation
    }
}

impl Render for Mesh {
    fn render(&mut self, config: &mut RenderConfig) {
        let uniforms = config.render_context.curr_shader().uniforms();

        unsafe {
            // Bind.
            gl::BindVertexArray(self.vao_id);

            // Enable vertex attributes.
            gl::EnableVertexAttribArray(AttribIndices::Positions as GLuint);
            if let Some(_) = self.normals {
                gl::EnableVertexAttribArray(AttribIndices::Normals as GLuint);
            }
            if let Some(_) = self.tex_coords {
                gl::EnableVertexAttribArray(AttribIndices::TexCoords as GLuint);
            }

            if let Some(ref dt) = self.diffuse_texture {
                dt.bind_and_send("diffuse_texture", uniforms);
            }
            // If no texture, just use a flat color.
            uniforms.send_3fv("diffuse_color", Vector3::new(0.5, 0.5, 0.5));
            if self.textured {
                uniforms.send_1i("use_texture", 1);
            } else {
                uniforms.send_1i("use_texture", 0);
            }

            // Set model matrix.
            uniforms.send_matrix_4fv("model_matrix",
                                     self.transformation.to_model_matrix());

            match self.indices {
                Some(ref i) => {
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, i.vbo_id);
                    gl::DrawElements(gl::TRIANGLES, i.data.len() as i32, gl::UNSIGNED_INT,
                                     ptr::null());
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
                },
                None => {
                    // Draw without indexing.
                    gl::DrawArrays(gl::TRIANGLES, 0, (self.positions.data.len() as GLsizei) / 3);
                }
            }

            // Disable vertex attributes.
            gl::DisableVertexAttribArray(AttribIndices::Positions as GLuint);
            match self.normals {
                Some(_) => gl::DisableVertexAttribArray(AttribIndices::Normals as GLuint),
                None => (),
            }
            match self.tex_coords {
                Some(_) => gl::DisableVertexAttribArray(AttribIndices::TexCoords as GLuint),
                None => (),
            }

            // Unbind.
            gl::BindVertexArray(0);
        }
    }

}

