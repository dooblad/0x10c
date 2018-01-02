pub mod player;

use game::event_handler::EventHandler;
use util::collide::Collide;
use world::EntitySlice;

pub trait Entity : Collide {
    fn tick(&mut self, event_handler: &EventHandler,
            collidables: &Vec<Box<Collide>>,
            entities: EntitySlice);
}

