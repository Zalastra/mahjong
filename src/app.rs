use std::thread;

use sdl2::{self, EventPump};
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;

use sdl2_image::{self, INIT_PNG};

use board::Board;

pub struct App<'a> {
    board: Board,
    renderer: Renderer<'a>,
    event_pump: EventPump,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let sdl_context = sdl2::init().expect("error creating sdl context");
        let video_subsystem = sdl_context.video().expect("error creating video subsystem");
        sdl2_image::init(INIT_PNG);
        let window = video_subsystem.window("Mahjong", 800, 600)
                                    .position_centered()
                                    .build()
                                    .expect("error creating window");

        let renderer = window.renderer().build().expect("error creating renderer");
        let event_pump = sdl_context.event_pump().expect("error creating event pump");

        App {
            board: Board::new(&renderer),
            renderer: renderer,
            event_pump: event_pump,
        }
    }

    pub fn run(&mut self) {
        let mut running = true;

        let mut mouse_x = 0;
        let mut mouse_y = 0;

        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        self.board.render(&mut self.renderer);
        self.renderer.present();

        println!("{}", self.board);

        while running {
            for event in self.event_pump.poll_iter() {
                use sdl2::event::Event;
                use sdl2::mouse::Mouse;

                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false
                    },
                    Event::MouseButtonDown { mouse_btn: Mouse::Left, x, y, .. } => {
                        mouse_x = x;
                        mouse_y = y;
                    },
                    Event::MouseButtonUp { mouse_btn: Mouse::Left, .. } => {
                        self.board.select_tile(mouse_x, mouse_y);

                        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
                        self.renderer.clear();
                        self.board.render(&mut self.renderer);
                        self.renderer.present();

                        println!("{}", self.board);
                    },
                    _ => {}
                }
            }

            thread::sleep_ms(10);
        }
    }
}