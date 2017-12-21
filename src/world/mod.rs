pub mod collidable;

use entity;
use entity::Entity;
use game;
use graphics;
use graphics::renderer;
use game::camera;
use util::collide::Collide;
use util::math::Point3;
use self::collidable::cube::Cube;

pub struct World {
    player: entity::player::Player,
    collidables: Vec<Box<Collide>>,
    renderer: renderer::Renderer,
}

impl World {
    pub fn new(player: entity::player::Player, display: graphics::Display) -> World {
        let mut collidables: Vec<Box<Collide>> = Vec::new();

        for i in 1..10 {
            let i_f = i as f32;
            let z = (1.1 * i_f * i_f) + 10.0;
            let size = i_f * 2.0;
            collidables.push(Box::new(Cube::new(size, Point3 {
                x: 0.0,
                y: -15.0,
                z,
            })));
        }

        World {
            player,
            collidables,
            renderer: renderer::Renderer::new(display),
        }
    }

    pub fn tick(&mut self, event_handler: &game::event_handler::EventHandler) {
        self.player.tick(event_handler, &self.collidables);
    }

    pub fn render(&mut self, camera: &camera::Camera) {
        self.renderer.render(camera, &mut self.collidables);
    }

    pub fn player(&self) -> &entity::player::Player {
        &self.player
    }
}
