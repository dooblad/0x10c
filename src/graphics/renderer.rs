use game::camera;
use graphics;
use graphics::Mesh;
use graphics::Render;

use ::read_file;

pub struct RenderingContext<'a, 'b: 'a> {
    pub program: &'a mut graphics::ShaderProgram,
    pub target: &'a mut graphics::Frame<'b>,
    pub camera: &'a camera::Camera,
}

impl<'a, 'b: 'a> RenderingContext<'a, 'b> {
    pub fn new(program: &'a mut graphics::ShaderProgram,
               target: &'a mut graphics::Frame<'b>,
               camera: &'a camera::Camera) -> RenderingContext<'a, 'b> {
        RenderingContext { program, target, camera }
    }

    pub fn draw(&mut self, mesh: &Mesh) {
        self.target.draw(mesh);
    }
}

pub struct Renderer<'a, 'b: 'a> {
    program: graphics::ShaderProgram,
    display: graphics::Display<'a>,
}

impl<'a, 'b: 'a> Renderer<'a, 'b> {
    pub fn new(display: graphics::Display) -> Renderer<'a, 'b> {
        // Compiling shaders and linking them together.
        let program = graphics::ShaderProgram::new(
            graphics::ShaderSource {
                vertex_shader: read_file("shaders/basic_textured.vert").unwrap().as_str(),
                fragment_shader: read_file("shaders/basic_textured.frag").unwrap().as_str(),
            },
        );

        Renderer {
            program,
            display,
        }
    }

    pub fn render<R>(&mut self, camera: &camera::Camera, renderables: &mut Vec<Box<R>>)
        where R: ?Sized + Render {
        self.display.use_program(&mut self.program);
        let mut target = self.display.draw().unwrap();
        {
            let mut context = RenderingContext::new(&mut self.program, &mut target, camera);
            context.target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0));

             for renderable in renderables.iter_mut() {
                 renderable.render(&mut context);
             }
        }

        target.finish();
    }
}
