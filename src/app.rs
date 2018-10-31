use {
    std::{
        thread,
        time::Duration,
    },
    sdl2::{
        event::Event,
        keyboard::Keycode,
        messagebox::*,
        mouse::MouseButton,
        pixels::Color,
    },
    crate::{
        board::Board,
        sdl::SdlContext,
        ui::{
            Action,
            UiContext
        },
    },
};

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
                Some(Action::Start) => board.reset(),
                Some(Action::Undo) => board.undo(),
                Some(Action::Hint) => board.highlight_possible_matches(),
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
