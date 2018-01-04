use rand;
use rand::distributions;
use rand::distributions::IndependentSample;

use entity::Entity;
use game::event_handler::EventHandler;
use graphics::Render;
use graphics::renderer::RenderingContext;
use graphics::mesh::pixel_quad::PixelQuad;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::Point3;
use world::EntitySlice;

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
            Range { min: -0.05, max: 0.05 },
        ];
        Lem {
            aabb: AABB::new(bounds, position),
            screen: PixelQuad::new(SCREEN_DIMENSIONS, SCREEN_SIZE, position),
        }
    }
}

impl Render for Lem {
    fn render(&mut self, context: &mut RenderingContext) {
        // TODO: Change shader to not have lighting here.
        context.push_shader_state();
        context.bind_shader(String::from("unlit"));
        self.screen.render(context);
        context.pop_shader_state();
    }
}

impl Collide for Lem {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

const PIXELS_TO_DIDDLE: u32 = 60;
impl Entity for Lem {
    fn tick(&mut self, _: &EventHandler, _: &Vec<Box<Collide>>,
            _: EntitySlice) {
        let idx_range = distributions::Range::new(0, self.screen.pixels().len());
        let col_range = distributions::Range::new(0, 255);
        let mut rng = rand::thread_rng();
        {
            let pixels = self.screen.pixels();
            for _ in 0..PIXELS_TO_DIDDLE {
                let idx = idx_range.ind_sample(&mut rng);
                let col = col_range.ind_sample(&mut rng);
                pixels[idx] = col;
            }
        }
        self.screen.update();
    }
}
