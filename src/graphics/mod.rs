pub mod mesh;
pub mod renderer;
pub mod shader;
pub mod texture;

use gl;
use glutin;
use glutin::GlContext;

use self::renderer::RenderingContext;

////////////////////////////////////////////////////////////////////////////////

pub trait Render {
    fn render(&mut self, context: &mut RenderingContext);
}

////////////////////////////////////////////////////////////////////////////////

fn clear_screen() {
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Display {
    gl_window: glutin::GlWindow,
}

impl Display {
    pub fn new(window: glutin::WindowBuilder, context: glutin::ContextBuilder,
               events_loop: &glutin::EventsLoop) -> Option<Display> {
        use glutin::GlContext;

        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe { gl_window.make_current() }.unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        unsafe{
            gl::Enable(gl::DEPTH_TEST);
            gl::CullFace(gl::BACK);
        }

        Some(Display {
            gl_window,
        })
    }

    pub fn finish_frame(&self) {
        self.gl_window.swap_buffers().unwrap();
    }

    pub fn gl_window(&self) -> &glutin::GlWindow {
        &self.gl_window
    }
}

////////////////////////////////////////////////////////////////////////////////

pub mod shadow_map {
    use cgmath;
    use gl;
    use std;

    use graphics::Display;
    use graphics::shader::{ProgramUniforms, ShaderProgram};
    use util::math::{Point3, Vector3, Matrix4};

    use super::texture::{Texture, TextureType};


    const DIMENSIONS: (u32, u32) = (1024, 1024);
    const Z_RANGE: (f32, f32) = (0.1, 50.0);
    // 90 degrees
    const FOV: f32 = std::f32::consts::FRAC_PI_2;

    /// Shadow map for point-light shadows.
    pub struct ShadowMap {
        fbo: u32,
        texture: Texture,
        dimensions: (u32, u32),
        z_range: (f32, f32),
        is_bound: bool,
    }

    impl ShadowMap {
        pub fn new() -> ShadowMap {
            let mut fbo = 0;
            let texture = Texture::new(TextureType::CubeMap, Some(DIMENSIONS));
            let dimensions = (DIMENSIONS.0, DIMENSIONS.1);

            unsafe {
                gl::GenFramebuffers(1, &mut fbo);

                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
                gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT,
                                       texture.id(), 0);
                // No color output in the bound framebuffer, only depth.
                gl::DrawBuffer(gl::NONE);
                gl::ReadBuffer(gl::NONE);

                if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                    panic!("Framebuffer incomplete.");
                }

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
            ShadowMap {
                fbo,
                texture,
                dimensions,
                z_range: Z_RANGE,
                is_bound: false,
            }
        }

        pub fn begin_pass(&mut self, light_pos: Point3, shader: &mut ShaderProgram) {
            if self.is_bound {
                panic!("Attempt to bind bound shadow map");
            }
            self.is_bound = true;

            unsafe {
                // Adjust viewport and bind framebuffer.
                gl::Viewport(0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32);
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
                gl::Clear(gl::DEPTH_BUFFER_BIT);
            }

            shader.bind();
            let uniforms = shader.uniforms();
            let depth_transforms = self.depth_transforms(light_pos);
            for i in 0..depth_transforms.len() {
                uniforms.send_matrix_4fv(&format!("depth_transforms[{}]", i),
                                         depth_transforms[i]);
            }
        }

        pub fn end_pass(&mut self, display: &Display) {
            if !self.is_bound {
                panic!("Attempt to unbind unbound shadow map");
            }
            self.is_bound = false;

            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                let dims = display.gl_window().get_inner_size().unwrap();
                if cfg!(target_os = "macos") {
                    // TODO: Why is the viewport 2x too small on Mac if you use inner size?
                    gl::Viewport(0, 0, dims.0 as i32 * 2, dims.1 as i32 * 2);
                } else {
                    gl::Viewport(0, 0, dims.0 as i32, dims.1 as i32);
                }
            }
        }

        pub fn bind_and_send(&self, uniform_name: &str, uniforms: &mut ProgramUniforms) {
            self.texture.bind_and_send(uniform_name, uniforms);
        }

        fn depth_transforms(&self, light_pos: Point3) -> [Matrix4; 6] {
            let aspect_ratio = self.dimensions.0 as f32 / self.dimensions.1 as f32;
            let depth_projection = cgmath::perspective(cgmath::Rad(FOV), aspect_ratio,
                                                       self.z_range.0, self.z_range.1);
            [
                // Positive X
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(1.0, 0.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
                ),
                // Negative X
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(-1.0, 0.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
                ),
                // Positive Y
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(0.0, 1.0, 0.0),
                    Vector3::new(0.0, 0.0, 1.0),
                ),
                // Negative Y
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(0.0, -1.0, 0.0),
                    Vector3::new(0.0, 0.0, -1.0),
                ),
                // Positive Z
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, -1.0, 0.0),
                ),
                // Negative Z
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(0.0, 0.0, -1.0),
                    Vector3::new(0.0, -1.0, 0.0),
                ),
            ]
        }

        pub fn z_range(&self) -> (f32, f32) {
            self.z_range
        }
    }
}
