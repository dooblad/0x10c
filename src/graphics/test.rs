use cgmath::Matrix;
use cgmath::SquareMatrix;
use gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::ffi::CString;

use graphics::shader::ShaderProgram;
use game::camera::Camera;
use util::math::Matrix4;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

pub fn test(program: &ShaderProgram, camera: &Camera) {
    unsafe {
        let mut vao = 0;
        let mut vbo = 0;

        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        program.bind();

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program.id(), CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            ptr::null(),
        );

        gl::Uniform3f(
            gl::GetUniformLocation(program.id(),CString::new("color").unwrap().as_ptr()),
            1.0, 0.0, 0.0);

        gl::UniformMatrix4fv(
            gl::GetUniformLocation(program.id(), CString::new("model").unwrap().as_ptr()),
            1, gl::FALSE, Matrix4::identity().as_ptr());
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(program.id(), CString::new("view").unwrap().as_ptr()),
            1, gl::FALSE, camera.view_matrix().as_ptr());
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(program.id(), CString::new("projection").unwrap().as_ptr()),
            1, gl::FALSE, camera.projection_matrix().as_ptr());

        // Draw a triangle from the 3 vertices
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        // Cleanup
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
