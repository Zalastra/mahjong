#![windows_subsystem = "windows"]

extern crate rand;
extern crate sdl2;

mod app;
mod board;
mod ui;
mod sdl;

fn main() {
    let mut app = app::App::new();
    app.run();
}
