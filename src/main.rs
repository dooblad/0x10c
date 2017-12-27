extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image;
extern crate rand;

use std::fs::File;
use std::io::Read;

pub mod dcpu;
pub mod entity;
pub mod game;
pub mod graphics;
pub mod util;
pub mod world;

fn main() {
    let mut game = game::Game::new();
    game.run();
}

pub fn read_file(filename: &str) -> Option<String> {
    match File::open(filename) {
        Ok(mut f) => {
            let mut contents = String::new();
            match f.read_to_string(&mut contents) {
                Ok(_) => {
                    Some(contents)
                },
                Err(_) => None,
            }
        },
        Err(_) => None,
    }
}
