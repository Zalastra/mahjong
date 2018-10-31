#![windows_subsystem = "windows"]

mod app;
mod board;
mod sdl;
mod ui;

fn main() {
    let mut sdl = sdl::init_sdl();
    app::run_game(&mut sdl);
}
