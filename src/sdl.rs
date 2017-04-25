use std::cell::RefCell;
use std::rc::Rc;

use sdl2::{self, EventPump};
use sdl2::render::Renderer;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

pub fn get_systems<'a>() -> Rc<(RefCell<Renderer<'static>>, RefCell<EventPump>)> {
    thread_local!(static SDL_SYSTEMS: Rc<(RefCell<Renderer<'static>>, RefCell<EventPump>)> = {
        let sdl_context = sdl2::init().expect("error creating sdl context");

        let video_subsystem = sdl_context.video().expect("error creating video subsystem");
        let mut window = video_subsystem.window("Mahjong", 1080, 750)
            .maximized()
            .resizable()
            .build()
            .expect("error creating window");

        window.set_minimum_size(730, 500).unwrap();

        let mut renderer = window.renderer().build().expect("error creating renderer");
        renderer.set_logical_size(730, 500).unwrap();

        let event_pump = sdl_context.event_pump().expect("error creating event pump");

        Rc::new((RefCell::new(renderer), RefCell::new(event_pump)))
    });

    SDL_SYSTEMS.with(|sdl_systems| sdl_systems.clone())
}

#[allow(dead_code)]
pub fn wait_for_click() {
    let sdl_systems = get_systems();
    if let Ok(mut event_pump) = sdl_systems.1.try_borrow_mut() {
        wait_for_click_ep(&mut *event_pump);
    }; // semicolon required in last if let of scope to not break the borrow checker
}

#[allow(dead_code)]
pub fn wait_for_click_ep(event_pump: &mut EventPump) {
    'a: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => break 'a,
                _ => (),
            }
        }
    }
}