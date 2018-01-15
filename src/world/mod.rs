pub mod collidable;

use std;
use std::slice;

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
    collidables: Vec<Box<Collide>>,
    entities: Vec<Box<Entity>>,
    renderer: Renderer,
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
        // TODO: Add entities.

        World {
            player,
            collidables,
            entities,
            renderer: Renderer::new(display),
        }
    }

    pub fn tick(&mut self, event_handler: &EventHandler) {
        {
            let (left, right) = self.entities.split_at_mut(0);
            self.player.tick(event_handler, &self.collidables,
                             EntitySlice::new(left, right));
        }

        for i in 0..self.entities.len() {
            let (lsplit, rest) = self.entities.split_at_mut(i);
            let (mid, rsplit) = rest.split_at_mut(1);
            mid[0].tick(event_handler, &self.collidables,
                        EntitySlice::new(lsplit, rsplit));
        }
    }

    pub fn render(&mut self, camera: &camera::Camera) {
        self.renderer.start_frame(camera);
        self.renderer.render(camera, &mut self.collidables);
        self.renderer.render(camera, &mut self.entities);
        self.renderer.end_frame();
    }

    pub fn player(&mut self) -> &mut Player {
        &mut self.player
    }
}


// TODO: Figure out how to use an iterator instead, if it's possible.
pub struct EntitySlice<'a> {
    left: &'a mut [Box<Entity>],
    right: &'a mut [Box<Entity>],
}

impl<'a> EntitySlice<'a> {
    pub fn new(left: &'a mut [Box<Entity>], right: &'a mut [Box<Entity>])
        -> EntitySlice<'a> {
        EntitySlice {
            left,
            right,
        }
    }
}

impl<'a> IntoIterator for EntitySlice<'a> {
    type Item = &'a mut Box<Entity>;
    type IntoIter = std::iter::Chain<slice::IterMut<'a, Box<Entity>>,
                                     slice::IterMut<'a, Box<Entity>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.left.into_iter().chain(self.right.into_iter())
    }
}
