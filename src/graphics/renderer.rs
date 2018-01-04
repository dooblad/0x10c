use std::collections::HashMap;

use game::camera::Camera;
use graphics;
use graphics::Render;
use graphics::shader::{ShaderProgram, ShaderSource};
use util::math::Vector3;


pub struct RenderingContext<'a> {
    shaders: &'a mut HashMap<String, ShaderProgram>,
    curr_shader_name: Option<String>,
    shader_stack: Vec<String>,
    camera: &'a Camera,
}

impl<'a> RenderingContext<'a> {
    pub fn new(shaders: &'a mut HashMap<String, ShaderProgram>, camera: &'a Camera)
        -> RenderingContext<'a> {
        RenderingContext {
            shaders,
            curr_shader_name: None,
            shader_stack: Vec::new(),
            camera,
        }
    }

    pub fn bind_shader(&mut self, shader_name: String) {
        let shader = match self.shaders.get(&shader_name) {
            Some(s) => s,
            None => panic!("No shader named \"{}\" exists", shader_name),
        };
        self.curr_shader_name = Some(shader_name);
        shader.bind();
    }

    pub fn push_shader_state(&mut self) {
        match self.curr_shader_name {
            Some(ref pn) => self.shader_stack.push(pn.clone()),
            None => panic!("No (currently-bound) shader to push"),
        }
    }

    pub fn pop_shader_state(&mut self) {
        match self.shader_stack.pop() {
            Some(p) => self.shaders.get(&p).unwrap().bind(),
            None => panic!("No shader to pop"),
        }
    }

    pub fn curr_shader(&mut self) -> &mut ShaderProgram {
        match self.curr_shader_name {
            Some(ref mut pn) => self.shaders.get_mut(pn).unwrap(),
            None => panic!("No currently-bound shader"),
        }
    }

    pub fn camera(&self) -> &'a Camera {
        self.camera
    }
}


pub struct Renderer {
    shaders: HashMap<String, ShaderProgram>,
    display: graphics::Display,
}

impl Renderer {
    pub fn new(display: graphics::Display) -> Renderer {
        let mut shaders = HashMap::new();
        // TODO: Load all shaders in the shader directory.
        shaders.insert(String::from("basic"),
                        ShaderProgram::new(ShaderSource::from(String::from("basic"))));
        shaders.insert(String::from("unlit"),
                        ShaderProgram::new(ShaderSource::from(String::from("unlit"))));

        Renderer {
            shaders,
            display,
        }
    }

    /// Should be called before any render calls are made for the current frame.
    pub fn start_frame(&mut self, camera: &Camera) {
        graphics::clear_screen();

        // Bind essential uniforms for all shaders.
        for shader in self.shaders.values_mut() {
            shader.bind();
            let uniforms = shader.uniforms();

            uniforms.send_3f("light_position", Vector3::new(0.0, 5.0, 0.0));

            // Set up matrices.
            uniforms.send_matrix_4fv("view_matrix", camera.view_matrix());
            uniforms.send_matrix_4fv("projection_matrix", camera.projection_matrix());
        }
    }

    /// Should be called after all render calls are made for the current frame.
    pub fn end_frame(&mut self) {
        self.display.finish_frame();
    }

    pub fn render<R>(&mut self, camera: &Camera, renderables: &mut Vec<Box<R>>)
        where R: ?Sized + Render {
        let mut context = RenderingContext::new(&mut self.shaders, camera);
        context.bind_shader(String::from("basic"));

        for renderable in renderables.iter_mut() {
            renderable.render(&mut context);
        }
    }

    /*
    pub fn render<'a, I, R: 'a>(&mut self, camera: &Camera, renderables: I)
        where I: Iterator<Item = &'a mut Box<R>>,
              R: ?Sized + Render {
        let mut context = RenderingContext::new(&mut self.shaders, camera);

        for renderable in renderables {
            renderable.render(&mut context);
        }
    }
    */
}
