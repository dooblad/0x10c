pub mod player;

use game::event_handler::EventHandler;
use world::Interactables;
use util::collide::Collide;

pub trait Entity : Collide {
    fn tick(&mut self, event_handler: &EventHandler, interactables: &mut Interactables);
}

