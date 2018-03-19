use graphics::Render;
use graphics::renderer::RenderingContext;
use entity::player::Player;
use entity::Entity;
use util::collide::Collide;


/// Stores parameters that define what to render each frame and how to render it.
pub struct RenderConfig<'a> {
    pub renderables: &'a mut Renderables<'a>,
    pub debug: bool,
}

impl<'a> RenderConfig<'a> {
    pub fn new(renderables: &'a mut Renderables<'a>, debug: bool) -> RenderConfig<'a> {
        RenderConfig {
            renderables,
            debug,
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
    pub fn render_all(&mut self, context: &mut RenderingContext) {
        for renderable in self.collidables.iter_mut() {
            renderable.render(context);
        }
        for renderable in self.entities.iter_mut() {
            renderable.render(context);
        }
        self.player.render(context);
    }
}

