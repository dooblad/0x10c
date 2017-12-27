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
use self::collidable::rect::Rect;

pub struct World {
    player: entity::player::Player,
    collidables: Vec<Box<Collide>>,
    renderer: renderer::Renderer,
}

impl World {
    pub fn new(player: entity::player::Player, display: graphics::Display) -> World {
        let mut collidables: Vec<Box<Collide>> = Vec::new();

        collidables.push(Box::new(Cube::new(2.0, Point3 {
            x: -2.0,
            y: 0.0,
            z: 10.0,
        })));
        collidables.push(Box::new(Cube::new(4.0, Point3 {
            x: 3.0,
            y: 0.0,
            z: 10.0,
        })));

        collidables.push(Box::new(Rect::new(5.0, 1.0, 5.0, Point3 {
            x: 12.0,
            y: 3.0,
            z: 10.0,
        })));

        // Floor
        collidables.push(Box::new(Rect::new(50.0, 1.0, 50.0, Point3 {
            x: 0.0,
            y: -0.5,
            z: 0.0,
        })));
        // Ceiling
        collidables.push(Box::new(Rect::new(50.0, 1.0, 50.0, Point3 {
            x: 0.0,
            y: 25.0,
            z: 0.0,
        })));
        // Walls
        collidables.push(Box::new(Rect::new(1.0, 25.0, 50.0, Point3 {
            x: -25.0,
            y: 12.5,
            z: 0.0,
        })));
        collidables.push(Box::new(Rect::new(1.0, 25.0, 50.0, Point3 {
            x: 25.0,
            y: 12.5,
            z: 0.0,
        })));
        collidables.push(Box::new(Rect::new(50.0, 25.0, 1.0, Point3 {
            x: 0.0,
            y: 12.5,
            z: 25.0,
        })));
        collidables.push(Box::new(Rect::new(50.0, 25.0, 1.0, Point3 {
            x: 0.0,
            y: 12.5,
            z: -25.0,
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
