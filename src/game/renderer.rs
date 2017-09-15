use glium;
use glium::Surface;
use glium::index::PrimitiveType;

use ::read_file;

pub struct Renderer {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
    display: glium::Display,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

impl Renderer {
    pub fn new(display: glium::Display) -> Renderer {
        let vertex_buffer = {
            implement_vertex!(Vertex, position);

            glium::VertexBuffer::new(
                &display,
                &[
                    Vertex { position: [-0.5, -0.5] },
                    Vertex { position: [ 0.0,  0.5] },
                    Vertex { position: [ 0.5, -0.5] },
                ]
            ).unwrap()
        };

        // building the index buffer
        let index_buffer = glium::IndexBuffer::new(
            &display,
            PrimitiveType::Patches { vertices_per_patch: 3 },
            &[0u16, 1, 2]
        ).unwrap();

        // compiling shaders and linking them together
        let program = glium::Program::new(
            &display,
            glium::program::SourceCode {
                vertex_shader: read_file("shaders/tess.vert").unwrap().as_str(),
                fragment_shader: read_file("shaders/tess.frag").unwrap().as_str(),
                geometry_shader: Some(read_file("shaders/tess.geom").unwrap().as_str()),
                tessellation_control_shader: Some(read_file("shaders/tess.tess_control").unwrap().as_str()),
                tessellation_evaluation_shader: Some(read_file("shaders/tess.tess_eval").unwrap().as_str()),
            },
        );

        let program = match program {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        Renderer {
            vertex_buffer,
            index_buffer,
            program,
            display,
        }
    }

    pub fn render(&self) {
        // level of tessellation
        let tess_level: i32 = 5;

        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tess_level: tess_level,
        };

        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &Default::default()
        ).unwrap();
        target.finish().unwrap();
    }
}
