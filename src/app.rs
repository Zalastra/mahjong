use std::thread;
use std::time::Duration;

use sdl2::{self, EventPump};
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::event::Event;
use sdl2::mouse::Mouse;
use sdl2::messagebox::*;

use sdl2_image::{self, INIT_PNG};

use board::Board;
use ui::UiContext;
use ui::Action::*;

pub struct App<'a> {
    board: Board,
    ui: UiContext,
    renderer: Renderer<'a>,
    event_pump: EventPump,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let sdl_context = sdl2::init().expect("error creating sdl context");
        let video_subsystem = sdl_context.video().expect("error creating video subsystem");
        sdl2_image::init(INIT_PNG).expect("error initializing sdl2 image");
        let mut window = video_subsystem.window("Mahjong", 1080, 750)
            .maximized()
            .resizable()
            .build()
            .expect("error creating window");

        window.set_minimum_size(730, 500).unwrap();

        let mut renderer = window.renderer().build().expect("error creating renderer");
        let event_pump = sdl_context.event_pump().expect("error creating event pump");

        renderer.set_logical_size(730, 500).unwrap();

        let ui = UiContext::new(&renderer);

        App {
            board: Board::new(&renderer),
            ui: ui,
            renderer: renderer,
            event_pump: event_pump,
        }
    }

    pub fn run(&mut self) {
        let mut running = true;
        let mut game_over = false;

        let mut mouse_x = 0;
        let mut mouse_y = 0;

        while running {
            for event in self.event_pump.poll_iter() {
                match self.ui.handle_event(&event) {
                    Some(Start) => self.board.reset(),
                    Some(Undo) => self.board.undo(),
                    Some(Hint) => self.board.highlight_possible_matches(),
                    _ => {}
                }
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false;
                    }
                    Event::MouseButtonDown { mouse_btn: Mouse::Left, x, y, .. } => {
                        mouse_x = x;
                        mouse_y = y;
                    }
                    Event::MouseButtonUp { mouse_btn: Mouse::Left, .. } => {
                        if self.board.try_select_tile(mouse_x, mouse_y).is_err() {
                            game_over = true;
                        }
                    }
                    Event::KeyUp { keycode: Some(Keycode::H), .. } => {
                        self.board.highlight_possible_matches();
                    }
                    Event::KeyUp { keycode: Some(Keycode::N), .. } => {
                        self.board.reset();
                    }
                    Event::KeyUp { keycode: Some(Keycode::U), .. } => {
                        self.board.undo();
                    }
                    _ => {}
                }
            }

            self.board.update();

            self.renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.renderer.clear();
            self.board.render(&mut self.renderer);
            self.ui.render(&mut self.renderer);
            self.renderer.present();

            if game_over {
                show_simple_message_box(MessageBoxFlag::all(),
                                        "Game Over",
                                        "You have no possible moves left",
                                        None)
                    .ok();
                game_over = false;
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
