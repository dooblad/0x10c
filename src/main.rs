#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use std::fs::File;
use std::io::Read;

mod game;

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
