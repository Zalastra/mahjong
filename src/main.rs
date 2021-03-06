#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod board;
mod sdl;
mod ui;

fn main() {
    let mut sdl = sdl::init();
    app::run(&mut sdl);
}
