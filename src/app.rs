use std::thread;
use std::time::Duration;

use sdl2;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::messagebox::*;
use sdl2::mouse::MouseButton;
use sdl2::image::INIT_PNG;

use board::Board;
use ui::UiContext;
use ui::Action::*;
use sdl::get_systems;

pub struct App {
    board: Board,
    ui: UiContext,
}

impl App {
    pub fn new() -> App {
        let _ = get_systems(); // Force SDL init
        sdl2::image::init(INIT_PNG).expect("error initializing sdl2 image");

        let board = Board::new();
        let ui = UiContext::new();

        App {
            board: board,
            ui: ui,
        }
    }

    pub fn run(&mut self) {
        let sdl_systems = get_systems();
        let mut renderer = sdl_systems.0.borrow_mut();
        let mut event_pump = sdl_systems.1.borrow_mut();

        let mut running = true;
        let mut game_over = false;

        let mut mouse_x = 0;
        let mut mouse_y = 0;

        while running {
            for event in event_pump.poll_iter() {
                let mut done = true;
                match self.ui.handle_event(&event) {
                    Some(Start) => self.board.reset(),
                    Some(Undo) => self.board.undo(),
                    Some(Hint) => self.board.highlight_possible_matches(),
                    _ => done = false,
                }
                if done {
                    continue;
                }
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false;
                    }
                    Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                        mouse_x = x;
                        mouse_y = y;
                    }
                    Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
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

            renderer.set_draw_color(Color::RGB(0, 0, 0));
            renderer.clear();
            self.board.render(&mut renderer);
            self.ui.render(&mut renderer);
            renderer.present();

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
