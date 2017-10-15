pub mod player;

use game;
use util::collide::Collide;

pub trait Entity : Collide {
    fn tick(&mut self, event_handler: &game::event_handler::EventHandler,
            collidables: &Vec<Box<Collide>>);
}

