use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::messagebox::*;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

use crate::board::Board;
use crate::sdl::SdlContext;
use crate::ui::Action::*;
use crate::ui::UiContext;

pub fn run(sdl: &mut SdlContext) {
    let mut board = Board::new(&sdl.texture_creator);
    let mut ui = UiContext::new(&sdl.texture_creator);
    
    let mut running = true;
    let mut game_over = false;

    let mut mouse_x = 0;
    let mut mouse_y = 0;

    while running {
        for event in sdl.event_pump.poll_iter() {
            let mut done = true;
            match ui.handle_event(&event) {
                Some(Start) => board.reset(),
                Some(Undo) => board.undo(),
                Some(Hint) => board.highlight_possible_matches(),
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
                    if board.try_select_tile(mouse_x, mouse_y).is_err() {
                        game_over = true;
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::H), .. } => {
                    board.highlight_possible_matches();
                }
                Event::KeyUp { keycode: Some(Keycode::N), .. } => {
                    board.reset();
                }
                Event::KeyUp { keycode: Some(Keycode::U), .. } => {
                    board.undo();
                }
                _ => {}
            }
        }

        board.update();

        sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl.canvas.clear();
        board.render(&mut sdl.canvas);
        ui.render(&mut sdl.canvas);
        sdl.canvas.present();

        if game_over {
            show_simple_message_box(
                MessageBoxFlag::all(),
                                    "Game Over",
                                    "You have no possible moves left",
                None,
            ).ok();
            game_over = false;
        }

        thread::sleep(Duration::from_millis(10));
    }
}
