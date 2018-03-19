use std;
use std::slice;

use entity::Entity;
use game::event_handler::EventHandler;
use util::collide::Collide;


/// Stores parameters that define what to update each frame and how to update it.
pub struct TickConfig<'a> {
    pub event_handler: &'a EventHandler,
    pub collidables: &'a Vec<Box<Collide>>,
    pub entities: EntitySlice<'a>,
    pub debug: bool,
}

impl<'a> TickConfig<'a> {
    pub fn new(event_handler: &'a EventHandler,
               collidables: &'a Vec<Box<Collide>>,
               entities: EntitySlice<'a>,
               debug: bool) -> TickConfig<'a> {
        TickConfig {
            event_handler,
            collidables,
            entities,
            debug,
        }
    }
}


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

