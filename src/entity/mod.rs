pub mod player;

use util::collide::Collide;
use world::TickConfig;

pub trait Entity : Collide {
    fn tick(&mut self, config: TickConfig);
    fn interactable(&self) -> bool;
    fn interact(&mut self);
    fn stop_interact(&mut self);
}

