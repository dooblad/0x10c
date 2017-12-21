pub mod cube;

use cgmath::SquareMatrix;
use cgmath::{Point3, Vector3, Matrix4};
use gl;
use gl::types::*;
use std::mem;
use std::ptr;

use graphics::Render;
use graphics::renderer::RenderingContext;
use graphics::texture::Texture;

// TODO: Reference graphics/geom/mesh.cc from C++ code to get Mesh working.

struct VertexArray {
    pub vbo_id: GLuint,
    pub data: Vec<GLfloat>,
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
    normals: Option<VertexArray>,
    tex_coords: Option<VertexArray>,
    model_matrix: Matrix4<GLfloat>,
    diffuse_texture: Option<Texture>,
}

impl Mesh {
    pub fn new(positions: Vec<GLfloat>,
               normals: Option<Vec<GLfloat>>,
               tex_coords: Option<Vec<GLfloat>>) -> Mesh {
        let mut vao_id = 0;

        let p;
        let mut n = None;
        let mut t = None;

        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);

            p = VertexArray {
                vbo_id: Self::gen_vbo(AttribIndices::Positions as GLuint, 3, &positions),
                data: positions
            };
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

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh {
            vao_id,
            positions: p,
            normals: n,
            tex_coords: t,
            model_matrix: Matrix4::identity(),
            diffuse_texture: None,
        }
    }

    pub fn move_to(&mut self, position: Point3<GLfloat>) {
        // The rightmost column of a model matrix is where translation data is stored.
        self.model_matrix[3][0] = position[0];
        self.model_matrix[3][1] = position[1];
        self.model_matrix[3][2] = position[2];
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
}

impl Render for Mesh {
    fn render(&mut self, context: &mut RenderingContext) {
        unsafe {
            // Bind.
            gl::BindVertexArray(self.vao_id);

            // Enable vertex attributes.
            gl::EnableVertexAttribArray(AttribIndices::Positions as GLuint);
            match self.normals {
                Some(_) => gl::EnableVertexAttribArray(AttribIndices::Normals as GLuint),
                None => (),
            }
            match self.tex_coords {
                Some(_) => gl::EnableVertexAttribArray(AttribIndices::TexCoords as GLuint),
                None => (),
            }

            // Set uniforms.
            let mut uniforms = context.program.uniforms();
            uniforms.send_matrix_4fv("model_matrix", self.model_matrix);
            uniforms.send_3fv("diffuse_color", Vector3::new(0.2, 0.2, 1.0));

            // Draw.
            gl::DrawArrays(gl::TRIANGLES, 0, (self.positions.data.len() as GLsizei) / 3);

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

