use {
    std::ops::{
        Deref,
        DerefMut,
    },
    sdl2::rect::Rect,
};

static TILE_WIDTH: u32 = 46;
static TILE_HEIGHT: u32 = 57; // TODO: change texture height to even number
static TILE_SIDE_WIDTH: u32 = 5;
static TILE_BOTTOM_HEIGHT: u32 = 5;
static TILE_SIDE_HEIGHT: u32 = TILE_HEIGHT + TILE_BOTTOM_HEIGHT;

pub struct Models(Vec<TileModel>);

impl Models {
    pub fn new(positions: &[(u8, u8, u8); 144]) -> Models {
        let models = positions
            .iter()
            .map(|&(x, y, z)| {
                let model_x = i32::from(x) * 23 + i32::from(z) * TILE_SIDE_WIDTH as i32 + 20;
                let model_y = i32::from(y) * 29 - i32::from(z) * TILE_BOTTOM_HEIGHT as i32 + 15;
                TileModel::new(model_x, model_y)
            }).collect::<Vec<_>>();

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
    fn new(x: i32, y: i32) -> Self {
        TileModel {
            face_rect: Rect::new(x, y, TILE_WIDTH, TILE_HEIGHT),
            side_rect: Rect::new(
                x - TILE_SIDE_WIDTH as i32,
                y,
                TILE_SIDE_WIDTH,
                TILE_SIDE_HEIGHT,
            ),
            bottom_rect: Rect::new(x, y + TILE_HEIGHT as i32, TILE_WIDTH, TILE_BOTTOM_HEIGHT),
            highlighted: false,
        }
    }

    pub fn face(&self) -> Rect {
        self.face_rect
    }

    pub fn side(&self) -> Rect {
        self.side_rect
    }

    pub fn bottom(&self) -> Rect {
        self.bottom_rect
    }

    pub fn is_highlighted(&self) -> bool {
        self.highlighted
    }

    pub fn highlight(&mut self) {
        self.highlighted = true;
    }

    pub fn dehighlight(&mut self) {
        self.highlighted = false;
    }

    // TODO: better name needed?
    pub fn hit_test(&self, x: i32, y: i32) -> bool {
        x >= self.x()
            && x <= self.x() + TILE_WIDTH as i32
            && y >= self.y()
            && y <= self.y() + TILE_HEIGHT as i32
    }

    fn x(&self) -> i32 {
        self.face_rect.x()
    }

    fn y(&self) -> i32 {
        self.face_rect.y()
    }
}
