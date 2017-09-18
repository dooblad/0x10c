use cgmath;
use glium;
use glium::Surface;

use game::camera;
use graphics::cube_mesh;
use graphics::cube_mesh::Drawable;

use ::read_file;

pub struct Renderer {
    program: glium::Program,
    display: glium::Display,
    cube: cube_mesh::CubeMesh,
}

pub struct RenderingContext<'a> {
    pub program: &'a glium::Program,
    pub target: &'a mut glium::Frame,
    pub view_matrix: &'a cgmath::Matrix4<f32>,
    pub projection_matrix: &'a cgmath::Matrix4<f32>,
}

impl Renderer {
    pub fn new(display: glium::Display) -> Renderer {
        // compiling shaders and linking them together
        let program = glium::Program::new(
            &display,
            glium::program::SourceCode {
                vertex_shader: read_file("shaders/normal_mapping.vert").unwrap().as_str(),
                fragment_shader: read_file("shaders/normal_mapping.frag").unwrap().as_str(),
                geometry_shader: None,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
            },
        );

        let program = match program {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let cube = cube_mesh::CubeMesh::new(&display, 1.0);

        Renderer {
            program,
            display,
            cube,
        }
    }

    pub fn render(&mut self, camera: &camera::Camera) {
        let mut target = self.display.draw();
        {
            let mut context = RenderingContext {
                program: &self.program,
                target: &mut target,
                view_matrix: &camera.view_matrix,
                projection_matrix: &camera.projection_matrix,
            };
            context.target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

            self.cube.draw(&mut context);
        }

        target.finish().unwrap();
    }
}
