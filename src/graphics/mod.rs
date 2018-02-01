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
    use cgmath::EuclideanSpace;
    use gl;
    use std;
    use std::ptr;

    use graphics::Display;
    use graphics::shader::ShaderProgram;
    use util::math::{Point3, Vector3, Matrix4};


    const DIMENSIONS: (u32, u32) = (1024, 1024);
    const NEAR_PLANE: f32 = 0.1;
    const FAR_PLANE: f32 = 25.0;
    // 90 degrees
    const FOV: f32 = std::f32::consts::FRAC_PI_2;
    const CUBE_MAP_LAYERS: [u32; 6] = [
        gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
    ];

    /// Shadow map for point-light shadows.
    pub struct ShadowMap {
        fbo: u32,
        tex_id: u32,
        dimensions: (u32, u32),
        is_bound: bool,
    }

    impl ShadowMap {
        pub fn new() -> ShadowMap {
            let mut fbo = 0;
            let mut tex_id = 0;
            let dimensions = (DIMENSIONS.0, DIMENSIONS.1);

            unsafe {
                gl::GenFramebuffers(1, &mut fbo);

                gl::GenTextures(1, &mut tex_id);
                gl::BindTexture(gl::TEXTURE_CUBE_MAP, tex_id);

                for layer in CUBE_MAP_LAYERS.iter() {
                    gl::TexImage2D(*layer, 0, gl::DEPTH_COMPONENT32 as i32,
                                   dimensions.0 as i32,
                                   dimensions.1 as i32,
                                   0, gl::DEPTH_COMPONENT, gl::FLOAT, ptr::null());
                }
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER,
                                  gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER,
                                  gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S,
                                  gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T,
                                  gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R,
                                  gl::CLAMP_TO_EDGE as i32);

                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
                gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT,
                                       tex_id, 0);
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
                tex_id,
                dimensions,
                is_bound: false,
            }
        }

        pub fn bind(&mut self, light_pos: Point3, shader: &mut ShaderProgram) {
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
            uniforms.send_1f("far_plane", FAR_PLANE);
            uniforms.send_3f("light_pos", light_pos.to_vec());
        }

        pub fn unbind(&mut self, display: &Display) {
            if !self.is_bound {
                panic!("Attempt to unbind unbound shadow map");
            }
            self.is_bound = false;

            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                let dims = display.gl_window().get_inner_size().unwrap();
                gl::Viewport(0, 0, dims.0 as i32, dims.1 as i32);
            }
        }

        fn depth_transforms(&self, light_pos: Point3) -> [Matrix4; 6] {
            let aspect_ratio = self.dimensions.0 as f32 / self.dimensions.1 as f32;
            let depth_projection = cgmath::perspective(cgmath::Rad(FOV), aspect_ratio,
                                                       NEAR_PLANE, FAR_PLANE);
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
                    light_pos + Vector3::new(-11.0, 0.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
                ),
                // Positive Y
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(0.0, 1.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
                ),
                // Negative Y
                depth_projection * Matrix4::look_at(
                    light_pos,
                    light_pos + Vector3::new(0.0, -1.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
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
    }
}
