pub mod collidable;

/// Contains structs related to ticking.
mod tick;
pub use self::tick::*;
/// Contains structs related to rendering.
mod render;
pub use self::render::*;

use glutin::VirtualKeyCode;

use entity::player::Player;
use entity::Entity;
use game::event_handler::EventHandler;
use graphics::Display;
use graphics::renderer::Renderer;
use game::camera::Camera;
use hardware::lem::Lem;
use util::collide::Collide;
use util::math::Point3;

use self::collidable::{cube, obj, rect};


pub struct World {
    player: Player,
    collidables: Vec<Box<Collide>>,
    entities: Vec<Box<Entity>>,
    renderer: Renderer,
    debug: bool,
}

impl World {
    pub fn new(player: Player, display: Display) -> World {
        let mut collidables: Vec<Box<Collide>> = Vec::new();

        collidables.push(Box::new(cube::new(2.0, Point3 {
            x: -2.0,
            y: 0.0,
            z: -10.0,
        })));
        collidables.push(Box::new(cube::new(4.0, Point3 {
            x: 3.0,
            y: 0.0,
            z: -10.0,
        })));

        collidables.push(Box::new(rect::new(5.0, 1.0, 5.0, Point3 {
            x: 12.0,
            y: 3.0,
            z: -10.0,
        })));

        // Floor
        collidables.push(Box::new(rect::new(50.0, 1.0, 50.0, Point3 {
            x: 0.0,
            y: -0.5,
            z: 0.0,
        })));
        // Ceiling
        collidables.push(Box::new(rect::new(50.0, 1.0, 50.0, Point3 {
            x: 0.0,
            y: 25.0,
            z: 0.0,
        })));
        // Walls
        collidables.push(Box::new(rect::new(1.0, 25.0, 50.0, Point3 {
            x: -25.0,
            y: 12.5,
            z: 0.0,
        })));
        collidables.push(Box::new(rect::new(1.0, 25.0, 50.0, Point3 {
            x: 25.0,
            y: 12.5,
            z: 0.0,
        })));
        collidables.push(Box::new(rect::new(50.0, 25.0, 1.0, Point3 {
            x: 0.0,
            y: 12.5,
            z: 25.0,
        })));
        collidables.push(Box::new(rect::new(50.0, 25.0, 1.0, Point3 {
            x: 0.0,
            y: 12.5,
            z: -25.0,
        })));

        collidables.push(Box::new(obj::new("res/globe.obj", Point3 {
            x: 0.0,
            y: 8.0,
            z: 0.0,
        })));

        collidables.push(Box::new(obj::new("res/ramp.obj", Point3 {
            x: 5.0,
            y: 0.0,
            z: 17.0,
        })));

        collidables.push(Box::new(obj::new("res/ramp_steep.obj", Point3 {
            x: -2.0,
            y: 0.0,
            z: 17.0,
        })));

        let mut entities: Vec<Box<Entity>> = Vec::new();
        let monitor = Lem::new(Point3 {
            x: 0.0,
            y: 0.0,
            z: -3.0,
        });
        entities.push(Box::new(monitor));

        World {
            player,
            collidables,
            entities,
            renderer: Renderer::new(display),
            debug: false,
        }
    }

    pub fn tick(&mut self, event_handler: &EventHandler) {
        if event_handler.is_key_pressed(&VirtualKeyCode::Grave) {
            self.debug = !self.debug;
        }


        {
            let (left, right) = self.entities.split_at_mut(0);
            self.player.tick(TickConfig::new(event_handler,
                                             &self.collidables,
                                             EntitySlice::new(left, right),
                                             self.debug));
        }

        for i in 0..self.entities.len() {
            let (lsplit, rest) = self.entities.split_at_mut(i);
            let (mid, rsplit) = rest.split_at_mut(1);
            mid[0].tick(TickConfig::new(event_handler,
                                        &self.collidables,
                                        EntitySlice::new(lsplit, rsplit),
                                        self.debug));
        }
    }

    pub fn render(&mut self, camera: &Camera) {
        self.renderer.render(RenderConfig::new(&mut Renderables {
            collidables: &mut self.collidables,
            entities: &mut self.entities,
            player: &mut self.player,
        }, self.debug), camera);
    }

    pub fn player(&mut self) -> &mut Player {
        &mut self.player
    }
}
