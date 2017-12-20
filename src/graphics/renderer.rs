use game::camera;
use graphics;
use graphics::Render;
use graphics::shader::{ShaderProgram, ShaderSource};

use ::read_file;

pub struct RenderingContext<'a> {
    pub program: &'a mut ShaderProgram,
    pub camera: &'a camera::Camera,
}

impl<'a> RenderingContext<'a> {
    pub fn new(program: &'a mut ShaderProgram,
               camera: &'a camera::Camera) -> RenderingContext<'a> {
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

    pub fn render<R>(&mut self, camera: &camera::Camera, renderables: &mut Vec<Box<R>>)
        where R: ?Sized + Render {
        graphics::clear_screen();
        self.program.bind();

        {
            let mut context = RenderingContext::new(&mut self.program, camera);

            for renderable in renderables.iter_mut() {
                renderable.render(&mut context);
            }
        }

        self.display.finish_frame();
    }
}
