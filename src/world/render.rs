use graphics::Render;
use graphics::renderer::RenderingContext;
use entity::player::Player;
use entity::Entity;
use util::collide::Collide;
use util::debug::DebugState;


/// Stores parameters that define what to render each frame and how to render it.
pub struct RenderConfig<'a> {
    pub render_context: &'a mut RenderingContext<'a>,
    pub debug_state: &'a DebugState,
}

impl<'a> RenderConfig<'a> {
    pub fn new(render_context: &'a mut RenderingContext<'a>,
               debug_state: &'a DebugState) -> RenderConfig<'a> {
        RenderConfig {
            render_context,
            debug_state,
        }
    }
}


/// A collection of objects to be rendered for a single frame.
pub struct Renderables<'a> {
    pub collidables: &'a mut Vec<Box<Collide>>,
    pub entities: &'a mut Vec<Box<Entity>>,
    pub player: &'a mut Player,
}

impl<'a> Renderables<'a> {
    pub fn render_all(&mut self, config: &mut RenderConfig) {
        for renderable in self.collidables.iter_mut() {
            renderable.render(config);
        }
        for renderable in self.entities.iter_mut() {
            renderable.render(config);
        }
        self.player.render(config);
    }
}

