use glium;
use glium::Surface;

use game::camera;
use graphics::Render;

use ::read_file;

pub struct Renderer {
    program: glium::Program,
    display: glium::Display,
}

pub struct RenderingContext<'a> {
    pub program: &'a glium::Program,
    pub target: &'a mut glium::Frame,
    pub camera: &'a camera::Camera,
}

impl Renderer {
    pub fn new(display: glium::Display) -> Renderer {
        // Compiling shaders and linking them together.
        let program = glium::Program::new(
            &display,
            glium::program::SourceCode {
                vertex_shader: read_file("shaders/basic_textured.vert").unwrap().as_str(),
                fragment_shader: read_file("shaders/basic_textured.frag").unwrap().as_str(),
                geometry_shader: None,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
            },
        ).unwrap();

        Renderer {
            program,
            display,
        }
    }

    pub fn render<R: ?Sized>(&mut self, camera: &camera::Camera, renderables: &mut Vec<Box<R>>)
        where R: Render {
        let mut target = self.display.draw();
        {
            let mut context = RenderingContext {
                program: &self.program,
                target: &mut target,
                camera,
            };
            context.target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

             for renderable in renderables.iter_mut() {
                 renderable.render(&mut context);
             }
        }

        target.finish().unwrap();
    }
}
