use cgmath::EuclideanSpace;
use std::collections::HashMap;

use game::camera::Camera;
use graphics;
use graphics::shadow_map::ShadowMap;
use graphics::shader::{ShaderConfig, ShaderProgram, ShaderSource, ShaderType};
use util::math::Point3;
use world::Renderables;


const LIGHT_POSITION: Point3 = Point3 {x: 0.0, y: 5.0, z: 0.0};


enum ContextState {
    Depth,
    Main,
}

pub struct RenderingContext<'a> {
    shaders: &'a mut HashMap<String, ShaderProgram>,
    curr_shader_name: Option<String>,
    shader_stack: Vec<String>,
    camera: &'a Camera,
    // TODO: Have rendering context trait?
    // Only problem seems to be that we can't pass trait objects to trait objects.
    state: ContextState
}

impl<'a> RenderingContext<'a> {
    pub fn new(shaders: &'a mut HashMap<String, ShaderProgram>, camera: &'a Camera)
        -> RenderingContext<'a> {
        RenderingContext {
            shaders,
            curr_shader_name: None,
            shader_stack: Vec::new(),
            camera,
            // TODO: Match on state in methods and do different things depending on
            // state.  And add methods for changing the state.
            state: ContextState::Main,
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
    shadow_map: ShadowMap,
    light_pos: Point3,
    display: graphics::Display,
}

impl Renderer {
    pub fn new(display: graphics::Display) -> Renderer {
        use self::ShaderType::*;

        let mut shaders = HashMap::new();
        // TODO: Load all shaders in the shader directory.
        shaders.insert(String::from("basic"),
                        ShaderProgram::new(ShaderSource::from(ShaderConfig {
                            name: String::from("basic"),
                            shader_type: VertFrag,
                        })));
        shaders.insert(String::from("unlit"),
                       ShaderProgram::new(ShaderSource::from(ShaderConfig {
                           name: String::from("unlit"),
                           shader_type: VertFrag,
                       })));
        shaders.insert(String::from("depth"),
                       ShaderProgram::new(ShaderSource::from(ShaderConfig {
                           name: String::from("depth"),
                           shader_type: VertGeomFrag,
                       })));

        Renderer {
            shaders,
            shadow_map: ShadowMap::new(),
            light_pos: LIGHT_POSITION,
            display,
        }
    }

    /// Should be called before any render calls are made for the current frame.
    fn start_frame(&mut self, camera: &Camera) {
        graphics::clear_screen();

        // Bind essential uniforms for all shaders.
        for shader in self.shaders.values_mut() {
            shader.bind();
            let uniforms = shader.uniforms();

            uniforms.send_3f("light_position", self.light_pos.to_vec());

            // Set up matrices.
            uniforms.send_matrix_4fv("view_matrix", camera.view_matrix());
            uniforms.send_matrix_4fv("projection_matrix", camera.projection_matrix());
        }
    }

    /// Should be called after all render calls are made for the current frame.
    fn end_frame(&mut self) {
        self.display.finish_frame();
    }

    fn depth_pass(&mut self, renderables: &mut Renderables) {
        self.shadow_map.bind(self.light_pos, self.shaders.get_mut("depth").unwrap());
        // TODO: Render things without allowing things to change which shader is bound.
        self.shadow_map.unbind(&self.display);
    }

    fn main_pass(&mut self, renderables: &mut Renderables, camera: &Camera) {
        let mut context = RenderingContext::new(&mut self.shaders, camera);
        context.bind_shader(String::from("basic"));
        renderables.render_all(&mut context);
    }

    pub fn render(&mut self, mut renderables: Renderables, camera: &Camera) {
        self.start_frame(camera);

        // Populate shadow map.
        self.depth_pass(&mut renderables);
        // Actually render to screen.
        self.main_pass(&mut renderables, camera);

        self.end_frame();
    }
}
