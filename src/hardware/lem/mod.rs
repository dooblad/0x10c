use entity::Entity;
use game::event_handler::EventHandler;
use graphics::Render;
use graphics::renderer::RenderingContext;
use graphics::mesh::pixel_quad::PixelQuad;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::Point3;
use world::Interactables;

const SCREEN_DIMENSIONS: (u32, u32) = (128, 96);
const SCREEN_SIZE: f32 = 2.0;

pub struct Lem {
    aabb: AABB,
    screen: PixelQuad,
}

impl Lem {
    pub fn new(position: Point3) -> Lem {
        let s = SCREEN_SIZE / 2.0;
        let bounds = [
            Range { min: -s, max: s },
            Range { min: -s, max: s },
            Range { min: -0.1, max: 0.1 },
        ];
        Lem {
            aabb: AABB::new(bounds, position),
            screen: PixelQuad::new(SCREEN_DIMENSIONS, SCREEN_SIZE, position),
        }
    }
}

impl Render for Lem {
    fn render(&mut self, context: &mut RenderingContext) {
        self.screen.render(context);
    }
}

impl Collide for Lem {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl Entity for Lem {
    fn tick(&mut self, event_handler: &EventHandler, interactables: &mut Interactables) {
        // TODO: Update some SHIT.
    }
}
