use {
    std::path::Path,
    sdl2::{
        event::Event::{
            self,
            MouseButtonDown,
            MouseButtonUp,
        },
        image::LoadTexture,
        mouse::MouseButton,
        rect::Rect,
        render::{
            Texture,
            TextureCreator,
            WindowCanvas,
        },
        video::WindowContext,
    },
};

pub struct UiContext<'tc> {
    buttons: [Button<'tc>; 3],
}

impl<'tc> UiContext<'tc> {
    pub fn new(texture_creator: &'tc TextureCreator<WindowContext>) -> Self {
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
    
    pub fn render(&self, canvas: &mut WindowCanvas) {
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
    texture: Texture<'tc>,
    action: Action,
    pressed: bool,
}

impl Button<'_> {
    fn new(x: i32, y: i32, width: u32, height: u32, action: Action, texture: Texture) -> Button {
        Button {
            placement: Rect::new(x / 2, y / 2, width / 2, height / 2),
            texture,
            action,
            pressed: false,
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) {
        let _ = canvas.copy(&self.texture, None, Some(self.placement));
    }

    fn mouse_down(&mut self, x: i32, y: i32) {
        let point_rect = Rect::new(x, y, 1, 1);
        if self.placement.has_intersection(point_rect) {
            self.pressed = true;
        }
    }

    fn mouse_up(&mut self, x: i32, y: i32) -> Option<Action> {
        if self.pressed {
            self.pressed = false;
            let point_rect = Rect::new(x, y, 1, 1);
            if self.placement.has_intersection(point_rect) {
                return Some(self.action);
            }
        }
        None
    }
}
