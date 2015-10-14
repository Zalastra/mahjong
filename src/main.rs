extern crate rand;
extern crate sdl2;
extern crate sdl2_image;

mod tile;
mod board;
mod app;

fn main() {
    let mut app = app::App::new();
    app.run();
}
