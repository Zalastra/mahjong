// TODO: further seperate the sdl2 back-end and figure out a strategy to remove C dependencies
//       and thus not replace sdl2 by another c based dependency if possible.

use {
    std::path::Path,
    sdl2::{
        rect::Rect as SDL_Rect,
        render::{
            Texture,
            TextureCreator as SDL_TextureCreator,
            WindowCanvas,
        },
        video::WindowContext,
    },
};

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    x: i32,
    y: i32,
    width: u16,
    height: u16,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u16, height: u16) -> Rect {
        Rect { x, y, width, height }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.x <= point.x
            && self.y <= point.y
            && self.x + self.width as i32 >= point.x
            && self.y + self.height as i32 >= point.y
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl From<Rect> for SDL_Rect {
    fn from(rect: Rect) -> Self {
        SDL_Rect::new(rect.x, rect.y, rect.width as u32, rect.height as u32)
    }
}

trait ConvertOptionalRect {
    fn into_sdl(self) -> Option<SDL_Rect>;
}

impl ConvertOptionalRect for Option<Rect> {
    fn into_sdl(self) -> Option<SDL_Rect> {
        self.map(|r| r.into())
    }
}

// TODO: figure out how to best deal with textures so that they can be easily reused
pub struct TextureHandle<'a>(Texture<'a>);

impl TextureHandle<'_> {
    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) {
        self.0.set_color_mod(red, green, blue)
    }
}

pub trait RenderTarget2D {
    fn draw_texture(&mut self, texutre: &TextureHandle, source: Option<Rect>, destination: Option<Rect>);
}

impl RenderTarget2D for WindowCanvas {
    fn draw_texture(
        &mut self,
        texture: &TextureHandle,
        source: Option<Rect>,
        destination: Option<Rect>
    ) {
        // TODO: figure out error handling
        let _ = self.copy(&texture.0, source.into_sdl(), destination.into_sdl());
    }
}

pub trait TextureCreator {
    fn load_texture(&self, path: &Path) -> Result<TextureHandle, ()>;
}

impl TextureCreator for SDL_TextureCreator<WindowContext> {
    fn load_texture(&self, path: &Path) -> Result<TextureHandle, ()> {
        match tc::load_tex(self, path) {
            Ok(texture) => Ok(TextureHandle(texture)),
            _ => Err(()),
        }
    }
}

mod tc {
    use std::path::Path;
    use sdl2::image::LoadTexture;
    use sdl2::render::{Texture, TextureCreator};
    use sdl2::video::WindowContext;

    pub fn load_tex<'tc>(tc: &'tc TextureCreator<WindowContext>, path: &Path) -> Result<Texture<'tc>, String> {
        tc.load_texture(path)
    }
}