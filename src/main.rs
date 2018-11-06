#![windows_subsystem = "windows"]

mod app;
mod board;
mod sdl;
mod ui;
mod graphics;

fn main() {
    let mut sdl = sdl::init();
    app::run(&mut sdl);
}
