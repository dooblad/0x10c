use cgmath::{Array, Matrix};
use cgmath::{Vector1, Vector2, Vector3, Vector4};
use cgmath::{Matrix2, Matrix3, Matrix4};
use gl;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

use ::read_file;
use graphics::mesh::AttribIndices;

// TODO: Do some shit with macros like glium has with `uniform!`.

pub struct ProgramUniforms {
    program_id: GLuint,
    // TODO: Cache get_location calls?
}

impl ProgramUniforms {
    pub fn new(program_id: GLuint) -> ProgramUniforms {
        ProgramUniforms {
            program_id,
        }
    }

    // TODO: Implement methods so that anything indexable can be used.

    pub fn get_location(&self, name: &str) -> GLint {
        unsafe {
            gl::GetUniformLocation(self.program_id, CString::new(name).unwrap().as_ptr())
        }
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

    pub fn send_1fv(&mut self, name: &str, data: GLfloat) {
        unsafe { gl::Uniform1fv(self.get_location(name), 1, Vector1::new(data).as_ptr()); }
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
}

////////////////////////////////////////////////////////////////////////////////

pub const SHADER_DIR: &str = "res/shaders";

/// Describes the structure of the shader pipeline (e.g., whether there is a geometry
/// shader).
pub enum ShaderType {
    /// vertex shader -> fragment shader.
    VertFrag,
    /// vertex shader -> geometry shader -> fragment shader.
    VertGeomFrag,
}

pub struct ShaderConfig {
    pub name: String,
    pub shader_type: ShaderType,
}

pub struct ShaderSource {
    pub vertex_shader: String,
    pub geometry_shader: Option<String>,
    pub fragment_shader: String,
}

impl From<ShaderConfig> for ShaderSource {
    fn from(ShaderConfig {name, shader_type}: ShaderConfig) -> Self {
        use self::ShaderType::*;

        let vertex_shader = read_file(
            &format!("{}/{}.vert", SHADER_DIR, name)).unwrap();
        let fragment_shader = read_file(
            &format!("{}/{}.frag", SHADER_DIR, name)).unwrap();
        match shader_type {
            VertFrag => {
                ShaderSource {
                    vertex_shader,
                    fragment_shader,
                    geometry_shader: None,
                }
            },
            VertGeomFrag => {
                ShaderSource {
                    vertex_shader,
                    fragment_shader,
                    geometry_shader: Some(read_file(
                        &format!("{}/{}.geom", SHADER_DIR, name)).unwrap()),
                }
            },
        }
    }
}


pub struct ShaderProgram {
    id: GLuint,
    uniforms: ProgramUniforms,
}

impl ShaderProgram {
    pub fn new(source: ShaderSource) -> ShaderProgram {
        let vs_id = Self::compile_shader(source.vertex_shader.as_str(),
                                         gl::VERTEX_SHADER);
        let gs_id = match source.geometry_shader {
            Some(gs) => Some(Self::compile_shader(gs.as_str(), gl::GEOMETRY_SHADER)),
            None => None,
        };
        let fs_id = Self::compile_shader(source.fragment_shader.as_str(),
                                         gl::FRAGMENT_SHADER);
        let program_id = Self::link_program(vs_id, gs_id, fs_id);

        ShaderProgram {
            id: program_id,
            uniforms: ProgramUniforms::new(program_id),
        }
    }

    // TODO: Write a drop function that deletes the shaders.

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
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

    fn link_program(vs_id: GLuint, gs_id: Option<GLuint>, fs_id: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();

            Self::setup_attrib_locs(program);

            gl::AttachShader(program, vs_id);
            match gs_id {
                Some(id) => gl::AttachShader(program, id),
                None => (),
            };
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

    fn setup_attrib_locs(program: GLuint) {
        unsafe {
            gl::BindAttribLocation(program, AttribIndices::Positions as u32,
                                   CString::new("position").unwrap().as_ptr());
            gl::BindAttribLocation(program, AttribIndices::Normals as u32,
                                   CString::new("normal").unwrap().as_ptr());
            gl::BindAttribLocation(program, AttribIndices::TexCoords as u32,
                                   CString::new("tex_coord").unwrap().as_ptr());
        }
    }
}
