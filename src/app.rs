#![allow(unused_imports, unused_mut)]
use std::path::PathBuf;
use std::io;
use std::num::ParseIntError;

use sdl2::{self};
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::render::Texture;
use sdl2::rect::Rect;

use sdl2_image::{self, INIT_PNG, LoadTexture};

use board::{Board, BoardPosition};

pub struct App<'a> {
    board: Board,
    renderer: Renderer<'a>,
    event_pump: EventPump,
}

struct Match {
    position1: BoardPosition,
    position2: BoardPosition,
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
        use std::thread;
        use std::sync::mpsc::channel;

        let mut running = true;

        println!("{}", self.board);

        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        self.board.render(&mut self.renderer);
        self.renderer.present();

        let (match_sender, match_receiver) = channel();

        thread::spawn(move || {
            loop {
                println!("Give the positions of the two tiles:");

                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).expect("unrecoverable error trying to read from stdin");
                let input: Vec<&str> = buffer.trim().split(' ').collect();
                if input.len() != 4 {
                    println!("wrong input: incorrect <{}> amount of inputs", input.len());
                    continue;
                }
                let parsed_input: Vec<Result<u8, ParseIntError>> =
                    input.iter().map(|&input_part| input_part.parse::<u8>()).collect();
                if parsed_input.iter().any(|parsed_input_part| parsed_input_part.is_err()) {
                    println!("wrong input: non-number input detected");
                    continue;
                }
                let parsed_input: Vec<u8> = parsed_input.iter().cloned().map(|input| input.unwrap()).collect();
                let position1 = BoardPosition { x: parsed_input[0], y: parsed_input[1] };
                let position2 = BoardPosition { x: parsed_input[2], y: parsed_input[3] };

                match_sender.send(Match {position1: position1, position2: position2}).unwrap();
            }
        });

        let mut mouse_x = 0;
        let mut mouse_y = 0;

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
                    },
                    _ => {}
                }
            }

            self.renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.renderer.clear();
            self.board.render(&mut self.renderer);
            self.renderer.present();

            if let Ok(tile_match) = match_receiver.try_recv() {
                self.board.make_match(tile_match.position1, tile_match.position2);

                println!("{}", self.board);
            }

            thread::sleep_ms(10);
        }
    }
}
