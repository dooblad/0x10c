pub mod renderer;

use cgmath::{Array, Matrix, SquareMatrix};
use cgmath::{Vector1, Vector2, Vector3, Vector4};
use cgmath::{Matrix2, Matrix3, Matrix4};
use gl;
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use glutin;
use image;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

pub trait Render {
    fn render(&mut self, context: &mut renderer::RenderingContext);
}

////////////////////////////////////////////////////////////////////////////////

pub struct Texture {
    id: u32,
}

impl Texture {
    const LEVEL_OF_DETAIL: GLint = 0;

    pub fn new(image: image::RgbaImage) -> Texture {
        // TODO: Actually generate OpenGL texture.
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

////////////////////////////////////////////////////////////////////////////////

// TODO: Reference graphics/geom/mesh.cc from C++ code to get Mesh working.

/*
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
*/

struct VertexArray {
    pub vbo_id: u32,
    pub data: Vec<GLfloat>,
}

pub struct Mesh {
    vao_id: u32,
    positions: VertexArray,
    normals: Option<VertexArray>,
    tex_coords: Option<VertexArray>,
    model_matrix: Matrix4<GLfloat>,
}

enum AttribIndex {
    Positions = 0,
    Normals = 1,
    TexCoords = 2,
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
                vbo_id: Self::gen_vbo(AttribIndex::Positions as GLuint, 3, &positions),
                data: positions
            };
            if let Some(nn) = normals {
                n = Some(VertexArray {
                    vbo_id: Self::gen_vbo(AttribIndex::Normals as GLuint, 3, &nn),
                    data: nn
                });
            }
            if let Some(tt) = tex_coords {
                t = Some(VertexArray {
                    vbo_id: Self::gen_vbo(AttribIndex::TexCoords as GLuint, 2, &tt),
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
        }
    }

    pub fn draw(&self, uniforms: &mut ProgramUniforms) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::EnableVertexAttribArray(AttribIndex::Positions as GLuint);
            match self.normals {
                Some(_) => gl::EnableVertexAttribArray(AttribIndex::Normals as GLuint),
                None => (),
            }
            match self.tex_coords {
                Some(_) => gl::EnableVertexAttribArray(AttribIndex::TexCoords as GLuint),
                None => (),
            }
            uniforms.send_matrix_4fv("model", self.model_matrix);
            gl::DrawArrays(gl::TRIANGLES, 0, self.positions.data.len() as GLsizei);
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
}


/*
pub struct VertexBuffer {
    vao_id: u32,
    vbo_id: u32,
    data: Vec<GLfloat>,
}

impl VertexBuffer {
    pub fn new(display: &graphics::Display, vertices: Vec<GLfloat>) -> Option<VertexBuffer> {
        // TODO: What we doin' wit' `display`?
        let mut vao_id = 0;
        let mut vbo_id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);

            gl::GenBuffers(1, &mut vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&vertices[0]),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BindVertexArray(0);
        }

        Some(VertexBuffer {
            vao_id,
            vbo_id,
            data: vertices,
        })
    }

    pub fn data(&self) -> &Vec<GLfloat> {
        &self.data
    }
}
*/

////////////////////////////////////////////////////////////////////////////////

pub struct Display<'a> {
    curr_program: Option<&'a mut ShaderProgram>,
    gl_window: glutin::GlWindow,
}

impl<'a> Display<'a> {
    pub fn new(window: glutin::WindowBuilder, context: glutin::ContextBuilder,
               events_loop: &glutin::EventsLoop) -> Option<Display<'a>> {
        use glutin::GlContext;

        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe { gl_window.make_current() }.unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        Some(Display {
            curr_program: None,
            gl_window,
        })
    }

    pub fn draw(&mut self) -> Option<Frame> {
        match self.curr_program {
            Some(ref mut program) =>
                Some(Frame::new(self.gl_window.get_inner_size().unwrap(),
                                program.uniforms())),
            None => None,
        }
    }

    pub fn use_program<'b: 'a>(&mut self, program: &'b mut ShaderProgram) {
        unsafe { gl::UseProgram(program.id) }
        self.curr_program = Some(program);
    }

    pub fn gl_window(&self) -> &glutin::GlWindow {
        &self.gl_window
    }
}

////////////////////////////////////////////////////////////////////////////////

// TODO: Do some shit with macros like glium has with `uniform!`.

pub struct ProgramUniforms {
    program_id: u32,
    // TODO: Cache get_location calls?
}

impl ProgramUniforms {
    pub fn new(program_id: u32) -> ProgramUniforms {
        ProgramUniforms {
            program_id,
        }
    }

    // TODO: Implement methods so that anything indexable can be used.

    pub fn get_location(&self, name: &str) -> GLint {
        unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8) }
    }

    pub fn send_1f(&mut self, name: &str, data: GLfloat) {
        unsafe { gl::Uniform1f(self.get_location(name), data); }
    }

    pub fn send_2f(&mut self, name: &str, data: Vector2<GLfloat>) {
        unsafe { gl::Uniform2f(self.get_location(name), data[0], data[1]); }
    }

    pub fn send_3f(&mut self, name: &str, data: Vector3<GLfloat>) {
        unsafe { gl::Uniform3f(self.get_location(name), data[0], data[1], data[2]); }
    }

    pub fn send_4f(&mut self, name: &str, data: Vector4<GLfloat>) {
        unsafe { gl::Uniform4f(self.get_location(name), data[0], data[1], data[2], data[3]); }
    }

    pub fn send_1i(&mut self, name: &str, data: GLint) {
        unsafe { gl::Uniform1i(self.get_location(name), data); }
    }

    pub fn send_2i(&mut self, name: &str, data: Vector2<GLint>) {
        unsafe { gl::Uniform2i(self.get_location(name), data[0], data[1]); }
    }

    pub fn send_3i(&mut self, name: &str, data: Vector3<GLint>) {
        unsafe { gl::Uniform3i(self.get_location(name), data[0], data[1], data[2]); }
    }

    pub fn send_4i(&mut self, name: &str, data: Vector4<GLint>) {
        unsafe { gl::Uniform4i(self.get_location(name), data[0], data[1], data[2], data[3]); }
    }

    pub fn send_1ui(&mut self, name: &str, data: GLuint) {
        unsafe { gl::Uniform1ui(self.get_location(name), data); }
    }

    pub fn send_2ui(&mut self, name: &str, data: Vector2<GLuint>) {
        unsafe { gl::Uniform2ui(self.get_location(name), data[0], data[1]); }
    }

    pub fn send_3ui(&mut self, name: &str, data: Vector3<GLuint>) {
        unsafe { gl::Uniform3ui(self.get_location(name), data[0], data[1], data[2]); }
    }

    pub fn send_4ui(&mut self, name: &str, data: Vector4<GLuint>) {
        unsafe { gl::Uniform4ui(self.get_location(name), data[0], data[1], data[2], data[3]); }
    }

    pub fn send_1fv(&mut self, name: &str, data: Vector1<GLfloat>) {
        unsafe { gl::Uniform1fv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_2fv(&mut self, name: &str, data: Vector2<GLfloat>) {
        unsafe { gl::Uniform2fv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_3fv(&mut self, name: &str, data: Vector3<GLfloat>) {
        unsafe { gl::Uniform3fv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_4fv(&mut self, name: &str, data: Vector4<GLfloat>) {
        unsafe { gl::Uniform4fv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_1iv(&mut self, name: &str, data: Vector1<GLint>) {
        unsafe { gl::Uniform1iv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_2iv(&mut self, name: &str, data: Vector2<GLint>) {
        unsafe { gl::Uniform2iv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_3iv(&mut self, name: &str, data: Vector3<GLint>) {
        unsafe { gl::Uniform3iv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_4iv(&mut self, name: &str, data: Vector4<GLint>) {
        unsafe { gl::Uniform4iv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_1uiv(&mut self, name: &str, data: Vector1<GLuint>) {
        unsafe { gl::Uniform1uiv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_2uiv(&mut self, name: &str, data: Vector2<GLuint>) {
        unsafe { gl::Uniform2uiv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_3uiv(&mut self, name: &str, data: Vector3<GLuint>) {
        unsafe { gl::Uniform3uiv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_4uiv(&mut self, name: &str, data: Vector4<GLuint>) {
        unsafe { gl::Uniform4uiv(self.get_location(name), 1, data.as_ptr()); }
    }

    pub fn send_matrix_2fv(&mut self, name: &str, data: Matrix2<GLfloat>) {
        unsafe { gl::UniformMatrix2fv(self.get_location(name), 1, gl::FALSE, data.as_ptr()); }
    }

    pub fn send_matrix_3fv(&mut self, name: &str, data: Matrix3<GLfloat>) {
        unsafe { gl::UniformMatrix3fv(self.get_location(name), 1, gl::FALSE, data.as_ptr()); }
    }

    pub fn send_matrix_4fv(&mut self, name: &str, data: Matrix4<GLfloat>) {
        unsafe { gl::UniformMatrix4fv(self.get_location(name), 1, gl::FALSE, data.as_ptr()); }
    }

    /*
    TODO: These?
    pub fn send_matrix_2x3fv(&mut self, name: &str, data: Matrix4<GLfloat>);
    pub fn send_matrix_3x2fv(&mut self, name: &str, data: Matrix4<GLfloat>);
    pub fn send_matrix_2x4fv(&mut self, name: &str, data: Matrix4<GLfloat>);
    pub fn send_matrix_4x2fv(&mut self, name: &str, data: Matrix4<GLfloat>);
    pub fn send_matrix_3x4fv(&mut self, name: &str, data: Matrix4<GLfloat>);
    pub fn send_matrix_4x3fv(&mut self, name: &str, data: Matrix4<GLfloat>);
    */
}

////////////////////////////////////////////////////////////////////////////////

pub struct ShaderSource<'a> {
    vertex_shader: &'a str,
    fragment_shader: &'a str,
}

pub struct ShaderProgram {
    id: GLuint,
    uniforms: ProgramUniforms,
}

impl ShaderProgram {
    pub fn new(source: ShaderSource) -> ShaderProgram {
        let vs_id = Self::compile_shader(source.vertex_shader, gl::VERTEX_SHADER);
        let fs_id = Self::compile_shader(source.fragment_shader, gl::FRAGMENT_SHADER);
        let program_id = Self::link_program(vs_id, fs_id);

        ShaderProgram {
            id: program_id,
            uniforms: ProgramUniforms::new(program_id),
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn uniforms(&mut self) -> &mut ProgramUniforms {
        &mut self.uniforms
    }

    fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            // Attempt to compile the shader.
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status.
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error.
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                // Subtract 1 to skip the trailing null character.
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8")
                );
            }
            shader
        }
    }

    fn link_program(vs_id: GLuint, fs_id: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs_id);
            gl::AttachShader(program, fs_id);
            gl::LinkProgram(program);
            // Get the link status.
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error.
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                // Subtract 1 to skip the trailing null character.
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8")
                );
            }
            program
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Frame<'a> {
    dimensions: (u32, u32),
    uniforms: &'a mut ProgramUniforms,
}

impl<'a> Frame<'a> {
    pub fn new(dimensions: (u32, u32), uniforms: &'a mut ProgramUniforms) -> Frame<'a> {
        Frame {
            dimensions,
            uniforms,
        }
    }

    pub fn finish(self) { }

    pub fn draw(&mut self, mesh: &Mesh) {
        mesh.draw(self.uniforms);
    }

    pub fn clear_color_and_depth(&mut self, color: (f32, f32, f32, f32)) {
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}
