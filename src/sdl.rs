use sdl2::{
    EventPump,
    image::INIT_PNG,
    render::{
        TextureCreator,
        WindowCanvas,
    },
    video::WindowContext,
};

pub struct SdlContext {
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
    pub event_pump: EventPump,
}

pub fn init() -> SdlContext {
    let sdl_context = sdl2::init().expect("error creating sdl context");
    sdl2::image::init(INIT_PNG).expect("error initializing sdl2 image");

    let video_subsystem = sdl_context.video().expect("error creating video subsystem");
    let mut window = video_subsystem
        .window("Mahjong", 1080, 750)
        .maximized()
        .resizable()
        .build()
        .expect("error creating window");

    window.set_minimum_size(730, 500).unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("error creating window canvas");
    canvas.set_logical_size(730, 500).unwrap();

    let texture_creator = canvas.texture_creator();

    let event_pump = sdl_context.event_pump().expect("error creating event pump");

    SdlContext {
        canvas,
        texture_creator,
        event_pump,
    }
}
