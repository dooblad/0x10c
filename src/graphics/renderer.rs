use cgmath::EuclideanSpace;
use gl;
use std::collections::HashMap;

use game::camera::Camera;
use graphics;
use graphics::shadow_map::ShadowMap;
use graphics::shader::{ShaderConfig, ShaderProgram, ShaderSource, ShaderType};
use util::math::Point3;
use world::{Renderables, RenderConfig};


const LIGHT_POSITION: Point3 = Point3 { x: 3.0, y: 5.0, z: 2.0 };
const DEBUG: bool = false;


#[derive(Debug, PartialEq)]
pub enum ContextState {
    /// When this state is entered, the depth shader is bound and no other shaders are allowed to be
    /// bound.
    Depth,
    /// The default state for the context.  In this state, the context acts normally (i.e., without
    /// restrictions).
    Main,
}

pub struct RenderingContext<'a> {
    shaders: &'a mut HashMap<String, ShaderProgram>,
    curr_shader_name: Option<String>,
    shader_stack: Vec<String>,
    state: ContextState
}

impl<'a> RenderingContext<'a> {
    pub fn new(shaders: &'a mut HashMap<String, ShaderProgram>) -> RenderingContext<'a> {
        RenderingContext {
            shaders,
            curr_shader_name: None,
            shader_stack: Vec::new(),
            state: ContextState::Main,
        }
    }

    pub fn bind_shader(&mut self, shader_name: String) {
        if let ContextState::Depth = self.state {
            return;
        }
        let shader = match self.shaders.get(&shader_name) {
            Some(s) => s,
            None => panic!("No shader named \"{}\" exists", shader_name),
        };
        self.curr_shader_name = Some(shader_name);
        shader.bind();
    }

    pub fn push_shader_state(&mut self) {
        if let ContextState::Depth = self.state {
            return;
        }
        match self.curr_shader_name {
            Some(ref pn) => self.shader_stack.push(pn.clone()),
            None => panic!("No (currently-bound) shader to push"),
        }
    }

    pub fn pop_shader_state(&mut self) {
        if let ContextState::Depth = self.state {
            return;
        }
        match self.shader_stack.pop() {
            Some(p) => self.bind_shader(p),
            None => panic!("No shader to pop"),
        }
    }

    pub fn set_state(&mut self, state: ContextState) {
        if state == self.state {
            panic!("Already in state \"{:?}\"", self.state);
        }
        if let ContextState::Depth = state {
            self.bind_shader(String::from("depth"));
        }
        self.state = state;
    }

    pub fn curr_shader(&mut self) -> &mut ShaderProgram {
        match self.curr_shader_name {
            Some(ref mut pn) => self.shaders.get_mut(pn).unwrap(),
            None => panic!("No currently-bound shader"),
        }
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
        shaders.insert(String::from("debug"),
                       ShaderProgram::new(ShaderSource::from(ShaderConfig {
                           name: String::from("debug"),
                           shader_type: VertFrag,
                       })));

        let shadow_map = ShadowMap::new();

        // Setup uniforms that won't vary across frames.
        for (shader_name, shader) in shaders.iter_mut() {
            shader.bind();
            let uniforms = shader.uniforms();

            // The depth shader uses the far plane for packing z values and other shaders use it
            // for unpacking those z values.
            uniforms.send_1f("far_plane", shadow_map.z_range().1);

            if shader_name != "depth" {
                // Only send the depth map to shaders that aren't the depth shader.
                shadow_map.bind_and_send("depth_map", uniforms);
            }
        }


        Renderer {
            shaders,
            shadow_map,
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
        unsafe {
            // Render the depth map with only the insides of meshes showing to fix shadow acne.
            gl::CullFace(gl::FRONT);
        }

        self.shadow_map.begin_pass(self.light_pos, self.shaders.get_mut("depth").unwrap());
        {
            let mut context = RenderingContext::new(&mut self.shaders);
            context.set_state(ContextState::Depth);
            renderables.render_all(&mut context);
        }
        self.shadow_map.end_pass(&self.display);

        unsafe {
            // Reset culling mode for normal rendering.
            gl::CullFace(gl::BACK);
        }
    }

    fn main_pass(&mut self, renderables: &mut Renderables, camera: &Camera) {
        use graphics::mesh::cube;
        use graphics::Render;

        let mut context = RenderingContext::new(&mut self.shaders);

        if DEBUG {
            // Visualize depth cube map.
            unsafe {
                context.bind_shader(String::from("debug"));

                let mut camera = camera.clone();
                camera.set_position(&Point3 { x: 0.0, y: 0.0, z: 0.0 });

                {
                    let uniforms = context.curr_shader().uniforms();
                    uniforms.send_matrix_4fv("view_matrix", camera.view_matrix());
                }

                gl::Disable(gl::CULL_FACE);
                let mut depth_cube = cube::new(5.0);
                depth_cube.render(&mut context);
                gl::Enable(gl::CULL_FACE);
            }
        } else {
            context.bind_shader(String::from("basic"));
            renderables.render_all(&mut context);
        }
    }

    pub fn render(&mut self, mut config: RenderConfig, camera: &Camera) {
        self.start_frame(camera);

        // Populate shadow map.
        self.depth_pass(&mut config.renderables);
        // Actually render to screen.
        self.main_pass(&mut config.renderables, camera);

        self.end_frame();
    }
}
