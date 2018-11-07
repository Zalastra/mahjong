use {
    std::path::Path,
    sdl2::{
        event::Event::{
            self,
            MouseButtonDown,
            MouseButtonUp,
        },
        mouse::MouseButton,
    },
    crate::graphics::{
        Point,
        Rect,
        RenderTarget2D,
        TextureCreator,
        TextureHandle,
    },
};

pub struct UiContext<'tc> {
    buttons: [Button<'tc>; 3],
}

impl<'tc> UiContext<'tc> {
    pub fn new<T: TextureCreator>(texture_creator: &'tc T) -> Self {
        use self::Action::*;
        
        let start_button_texture = texture_creator
            .load_texture(Path::new("img/start.png"))
            .unwrap();
        let undo_button_texture = texture_creator
            .load_texture(Path::new("img/undo.png"))
            .unwrap();
        let hint_button_texture = texture_creator
            .load_texture(Path::new("img/hint.png"))
            .unwrap();

        let start_button = Button::new(10, 10, 120, 50, Start, start_button_texture);
        let undo_button = Button::new(10, 70, 120, 50, Undo, undo_button_texture);
        let hint_button = Button::new(10, 130, 120, 50, Hint, hint_button_texture);

        UiContext {
            buttons: [start_button, undo_button, hint_button],
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<Action> {
        match *event {
            MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                for button in &mut self.buttons {
                    button.mouse_down(x, y);
                }
            }
            MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
                for button in &mut self.buttons {
                    if let Some(action) = button.mouse_up(x, y) {
                        return Some(action);
                    }
                }
            }
            _ => {}
        }
        None
    }
    
    pub fn render<R: RenderTarget2D>(&self, canvas: &mut R) {
        for button in &self.buttons {
            button.render(canvas);
        }
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Start,
    Undo,
    Hint,
}

struct Button<'tc> {
    placement: Rect,
    texture: TextureHandle<'tc>,
    action: Action,
    pressed: bool,
}

impl Button<'_> {
    fn new(x: i32, y: i32, width: u16, height: u16, action: Action, texture: TextureHandle) -> Button {
        Button {
            placement: Rect::new(x / 2, y / 2, width / 2, height / 2),
            texture,
            action,
            pressed: false,
        }
    }

    fn render<R: RenderTarget2D>(&self, canvas: &mut R) {
        let _ = canvas.draw_texture(&self.texture, None, Some(self.placement.into()));
    }

    fn mouse_down(&mut self, x: i32, y: i32) {
        if self.placement.contains_point(Point::new(x, y)) {
            self.pressed = true;
        }
    }

    fn mouse_up(&mut self, x: i32, y: i32) -> Option<Action> {
        if self.pressed {
            self.pressed = false;
            if self.placement.contains_point(Point::new(x, y)) {
                return Some(self.action);
            }
        }
        None
    }
}
