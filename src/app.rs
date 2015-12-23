use std::thread;
use std::time::Duration;

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
        let mut window = video_subsystem.window("Mahjong", 1080, 750)
                                    .maximized()
                                    .resizable()
                                    .build()
                                    .expect("error creating window");

        window.set_minimum_size(730, 500);

        let mut renderer = window.renderer().build().expect("error creating renderer");
        let event_pump = sdl_context.event_pump().expect("error creating event pump");

        renderer.set_logical_size(730, 500).unwrap();

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

        if cfg!(debug_assertions) { println!("{:?}", self.board); }

        while running {
            for event in self.event_pump.poll_iter() {
                use sdl2::event::Event;
                use sdl2::event::Event::*;
                use sdl2::event::WindowEventId::*;
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
                        if cfg!(debug_assertions) { println!("{:?}", self.board); }
                    },
                    Event::KeyUp { keycode: Some(Keycode::U), .. } => {
                        self.board.undo();
                        if cfg!(debug_assertions) { println!("{:?}", self.board); }
                    },
                    Event::Window { win_event_id, .. } => {
                        // NOTE: ugly hack to fix SDL2 bug occurs when maximizing a previously
                        //       maximized window that was minimized to the taskbar.
                        if win_event_id == FocusGained {
                            let mut window = self.renderer.window_mut().unwrap();
                            if window.window_flags() & 0x80 == 0x80 {
                                window.restore();
                                window.maximize();
                            }
                        }
                    },
                    _ => {}
                }
            }

            self.renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.renderer.clear();
            self.board.render(&mut self.renderer);
            self.renderer.present();

            thread::sleep(Duration::from_millis(10));
        }
    }
}
