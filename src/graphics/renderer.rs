use game::camera::Camera;
use graphics;
use graphics::Render;
use graphics::shader::{ShaderProgram, ShaderSource};
use util::math::Vector3;

use ::read_file;

pub struct RenderingContext<'a> {
    pub program: &'a mut ShaderProgram,
    pub camera: &'a Camera,
}

impl<'a> RenderingContext<'a> {
    pub fn new(program: &'a mut ShaderProgram,
               camera: &'a Camera) -> RenderingContext<'a> {
        RenderingContext { program, camera }
    }
}

pub struct Renderer {
    program: ShaderProgram,
    display: graphics::Display,
}

impl Renderer {
    pub fn new(display: graphics::Display) -> Renderer {
        // Compiling shaders and linking them together.
        let program = ShaderProgram::new(
            ShaderSource {
                // TODO: Use better shaders.
                vertex_shader: read_file("shaders/basic.vert").unwrap().as_str(),
                fragment_shader: read_file("shaders/basic.frag").unwrap().as_str(),
            },
        );

        Renderer {
            program,
            display,
        }
    }

    pub fn start_frame(&mut self, camera: &Camera) {
        graphics::clear_screen();
        self.program.bind();

        let uniforms = self.program.uniforms();

        // TODO: Set up lights.
        let camera_position = camera.position();
        uniforms.send_3f("light_position",
                         Vector3::new(
                             camera_position[0],
                             camera_position[1],
                             camera_position[2],
                         ));

        // Set up matrices.
        uniforms.send_matrix_4fv("view_matrix", camera.view_matrix());
        uniforms.send_matrix_4fv("projection_matrix", camera.projection_matrix());
    }

    pub fn end_frame(&mut self) {
        self.display.finish_frame();
    }

    /*
    pub fn render<R,I>(&mut self, camera: &Camera, renderables: I)
        where R: ?Sized + Render,
              I: Iterator<Item = Box<R>> {
              */
    pub fn render<R>(&mut self, camera: &Camera, renderables: &mut Vec<Box<R>>)
        where R: ?Sized + Render {
        let mut context = RenderingContext::new(&mut self.program, camera);

        for renderable in renderables.iter_mut() {
            renderable.render(&mut context);
        }
    }
}
