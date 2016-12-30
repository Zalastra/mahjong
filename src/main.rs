#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

extern crate rand;
extern crate sdl2;

mod app;
mod board;
mod ui;

fn main() {
    let mut app = app::App::new();
    app.run();
}
