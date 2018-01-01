pub mod collidable;

use entity::player::Player;
use entity::Entity;
use game::event_handler::EventHandler;
use graphics::Display;
use graphics::renderer::Renderer;
use game::camera;
use hardware::lem::Lem;
use util::collide::Collide;
use util::math::Point3;
use self::collidable::cube::Cube;
use self::collidable::rect::Rect;

pub struct World {
    player: Player,
    interactables: Interactables,
    renderer: Renderer,
}

pub struct Interactables {
    pub collidables: Vec<Box<Collide>>,
    pub entities: Vec<Box<Entity>>,
}

impl World {
    pub fn new(player: Player, display: Display) -> World {
        let mut collidables: Vec<Box<Collide>> = Vec::new();

        collidables.push(Box::new(Cube::new(2.0, Point3 {
            x: -2.0,
            y: 0.0,
            z: -10.0,
        })));
        collidables.push(Box::new(Cube::new(4.0, Point3 {
            x: 3.0,
            y: 0.0,
            z: -10.0,
        })));

        collidables.push(Box::new(Rect::new(5.0, 1.0, 5.0, Point3 {
            x: 12.0,
            y: 3.0,
            z: -10.0,
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

        let mut entities: Vec<Box<Entity>> = Vec::new();
        let monitor = Lem::new(Point3 {
            x: 0.0,
            y: 3.0,
            z: -2.0,
        });
        entities.push(Box::new(monitor));

        World {
            player,
            interactables: Interactables {
                collidables,
                entities,
            },
            renderer: Renderer::new(display),
        }
    }

    pub fn tick(&mut self, event_handler: &EventHandler) {
        self.player.tick(event_handler, &mut self.interactables);
    }

    pub fn render(&mut self, camera: &camera::Camera) {
        self.renderer.start_frame(camera);
        self.renderer.render(camera, &mut self.interactables.collidables);
        self.renderer.render(camera, &mut self.interactables.entities);
        self.renderer.end_frame();
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn interactables(&self) -> &Interactables {
        &self.interactables
    }
}
