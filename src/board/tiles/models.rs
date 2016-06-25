use std::ops::{Deref, DerefMut};

use std::rc::Rc;

use sdl2::rect::Rect;

use super::positions::BoardPosition;

static TILE_WIDTH: u32 = 46;
static TILE_HEIGHT: u32 = 57; // TODO: change texture height to even number
static TILE_SIDE_WIDTH: u32 = 5;
static TILE_BOTTOM_HEIGHT: u32 = 5;

pub struct Models(Vec<TileModel>);

impl Models {
    pub fn new(board_positions: &[Rc<BoardPosition>]) -> Models {
        let mut models = Vec::new();
        
        for pos in board_positions.iter() {
            let x = pos.x() as i32 * 23 + pos.z() as i32 * TILE_SIDE_WIDTH as i32 + 20;
            let y = pos.y() as i32 * 29 - pos.z() as i32 * TILE_BOTTOM_HEIGHT as i32 + 15;
            models.push(TileModel::new(x, y));
        }
        models.shrink_to_fit();

        Models(models)
    }
}

impl Deref for Models {
    type Target = [TileModel];

    fn deref(&self) -> &[TileModel] {
        self.0.as_slice()
    }
}

impl DerefMut for Models {
    fn deref_mut(&mut self) -> &mut [TileModel] {
        self.0.as_mut_slice()
    }
}

#[derive(Debug)]
pub struct TileModel {
    pub face_rect: Rect,
    pub side_rect: Rect,
    pub bottom_rect: Rect,
    pub highlighted: bool,
}

impl TileModel {
    #[inline]
    fn new(x: i32, y: i32) -> Self {
        TileModel {
            face_rect: Rect::new(x, y, TILE_WIDTH, TILE_HEIGHT),
            side_rect: Rect::new(x - TILE_SIDE_WIDTH as i32, y, 
                TILE_SIDE_WIDTH, TILE_HEIGHT + TILE_BOTTOM_HEIGHT),
            bottom_rect: Rect::new(x, y + TILE_HEIGHT as i32, TILE_WIDTH, TILE_BOTTOM_HEIGHT),
            highlighted: false,
        }
    }

    #[inline]
    pub fn face(&self) -> Rect {
        self.face_rect
    }

    #[inline]
    pub fn side(&self) -> Rect {
        self.side_rect
    }

    #[inline]
    pub fn bottom(&self) -> Rect {
        self.bottom_rect
    }

    #[inline]
    pub fn is_highlighted(&self) -> bool {
        self.highlighted
    }

    #[inline]
    pub fn highlight(&mut self) {
        self.highlighted = true;
    }

    #[inline]
    pub fn dehighlight(&mut self) {
        self.highlighted = false;
    }

    // TODO: better name needed?
    #[inline]
    pub fn hit_test(&self, x: i32, y: i32) -> bool {
        if x >= self.x() && x <= self.x() + TILE_WIDTH as i32 &&
                y >= self.y() && y <= self.y() + TILE_HEIGHT as i32 {
            true
        } else {
            false
        }
    }

    #[inline]
    fn x(&self) -> i32 {
        self.face_rect.x()
    }

    #[inline]
    fn y(&self) -> i32 {
        self.face_rect.y()
    }
}