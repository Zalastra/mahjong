#![windows_subsystem = "windows"]

extern crate rand;
extern crate sdl2;

mod app;
mod board;
mod sdl;
mod ui;

use app::run_game;
use sdl::init_sdl;

fn main() {
    let mut sdl = init_sdl();
    run_game(&mut sdl);
}
