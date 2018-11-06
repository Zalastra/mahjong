use {
    std::path::Path,
    sdl2::{
        //image::LoadTexture,
        rect::Rect as SDL_Rect,
        render::{
            Texture,
            TextureCreator as SDL_TextureCreator,
            WindowCanvas,
        },
        video::WindowContext,
    },
};

pub struct Rect(SDL_Rect);

impl From<Rect> for SDL_Rect {
    fn from(rect: Rect) -> Self {
        rect.0
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

pub struct TextureHandle<'a>(Texture<'a>);

pub trait RenderTarget2D {
    fn draw_texture(&mut self, texutre: TextureHandle, source: Option<Rect>, destination: Option<Rect>);
}

impl RenderTarget2D for WindowCanvas {
    fn draw_texture(
        &mut self,
        texture: TextureHandle,
        source: Option<Rect>,
        destination: Option<Rect>
    ) {
        // TODO: figure out error handling
        let _ = self.copy(&texture.0, source.into_sdl(), destination.into_sdl());
    }
}

pub trait TextureCreator {
    fn load_texture(&mut self, path: &Path) -> Result<TextureHandle, ()>;
}

impl TextureCreator for SDL_TextureCreator<WindowContext> {
    fn load_texture(&mut self, path: &Path) -> Result<TextureHandle, ()> {
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

    pub fn load_tex<'tc>(tc: &'tc mut TextureCreator<WindowContext>, path: &Path) -> Result<Texture<'tc>, String> {
        tc.load_texture(path)
    }
}