pub mod collidable_cube;

use glium;

use entity;
use entity::Entity;
use game;
use graphics::renderer;
use game::camera;
use util::collide::Collide;
use util::math::{Point3, Vector3};
use self::collidable_cube::CollidableCube;

pub struct World {
    player: entity::player::Player,
    collidables: Vec<Box<Collide>>,
    renderer: renderer::Renderer,
}

impl World {
    pub fn new(player: entity::player::Player, display: glium::Display) -> World {
        let mut collidables: Vec<Box<Collide>> = Vec::new();

        collidables.push(Box::new(CollidableCube::new(&display, 5.0, Point3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        }, Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })));
        // TODO: Fix the y coordinate system.
        collidables.push(Box::new(CollidableCube::new(&display, 30.0, Point3 {
            x: 0.0,
            y: -30.0,
            z: 0.0,
        }, Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })));

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
