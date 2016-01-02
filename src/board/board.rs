use std::cell::{Cell, RefCell};
use std::collections::hash_set::HashSet;
use std::fmt;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::rc::{Rc, Weak};

use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;

use sdl2_image::LoadTexture;

use rand;
use rand::distributions::{IndependentSample, Range};

use board::tile::{Position, Tile, TilePosition, TileType};

pub struct Board {
    tiles: Vec<Tile>,
    played: Vec<usize>,
    blocking_data: Vec<TileBlockingData>,
    reachable_tiles: Vec<usize>,
    selected_tile: Option<usize>,
    side_texture: Texture,
    bottom_texture: Texture,
}

struct TileBlockingData {
    pub tile_index: usize,
    pub blocking_verticaly: Vec<usize>,
    pub blocking_right: Vec<usize>,
    pub blocking_left: Vec<usize>,
    pub blocked_by_verticaly: Cell<u8>,
    pub blocked_by_left: Cell<u8>,
    pub blocked_by_right: Cell<u8>,
}

enum BlockingDirection {
    Verticaly, Right, Left,
}

use self::BlockingDirection::{Left, Right, Verticaly};

impl TileBlockingData {
    fn get_blocking(&self, blocking: BlockingDirection) -> &Vec<usize> {
        match blocking {
            Left => &self.blocking_left,
            Right => &self.blocking_right,
            Verticaly => &self.blocking_verticaly,
        }
    }
}

fn generate_blocking_data(tiles: &Vec<Tile>) -> Vec<TileBlockingData> {
    macro_rules! blocking_tiles_on {
        ( $( { $x:expr, $y:expr, $z:expr } ),+ ) => {
            tiles.iter()
                 .enumerate()
                 .filter(|&(_, other_tile)| {
                     let positions = [ $( Position::new($x, $y, $z), )+ ];
                     positions.iter().any(|position| other_tile.is_on_position(position))
                 })
                 .map(|(index, _)| index)
                 .collect()
        }
    }

    let mut blocking_data = Vec::new();
    for (index, tile) in tiles.iter().enumerate() {
        let position = &tile.position;
        let blocking_verticaly = match position.z() {
            0 => Vec::new(),
            _ => blocking_tiles_on![{position.x(),     position.y(),     position.z() - 1},
                                    {position.x() + 1, position.y(),     position.z() - 1},
                                    {position.x(),     position.y() + 1, position.z() - 1},
                                    {position.x() + 1, position.y() + 1, position.z() - 1}]
        };

        let blocking_left = match position.x() {
            0 => Vec::new(),
            _ => blocking_tiles_on![{position.x() - 1, position.y(),     position.z()},
                                    {position.x() - 1, position.y() + 1, position.z()}]
        };

        let blocking_right =
            blocking_tiles_on![{position.x() + 2, position.y(),     position.z()},
                               {position.x() + 2, position.y() + 1, position.z()}];

        blocking_data.push(TileBlockingData {
            tile_index: index,
            blocking_verticaly: blocking_verticaly,
            blocking_left: blocking_left,
            blocking_right: blocking_right,
            blocked_by_verticaly: Cell::new(0),
            blocked_by_left: Cell::new(0),
            blocked_by_right: Cell::new(0),
        })
    }

    blocking_data
}

impl Board {
    pub fn new(renderer: &Renderer) -> Board {
        let tiles = create_board(renderer);

        let blocking_data = generate_blocking_data(&tiles);

        let side_texture = renderer.load_texture(Path::new("img/TileSide.png")).expect("error loading side texture");
        let bottom_texture = renderer.load_texture(Path::new("img/TileBottom.png")).expect("error loading bottom texture");

        let mut board = Board {
            tiles: tiles,
            played: Vec::new(),
            blocking_data: blocking_data,
            reachable_tiles: Vec::new(),
            selected_tile: None,
            side_texture: side_texture,
            bottom_texture: bottom_texture,
        };

        board.update_meta_data();

        board
    }

    pub fn select_tile(&mut self, mouse_x: i32, mouse_y: i32) {
        let tile_index = self.find_tile_index_by_coord(mouse_x, mouse_y);

        if let Some(tile_index) = tile_index {
            match self.selected_tile {
                Some(tile_index2) => {
                    // deselect tile
                    if tile_index == tile_index2 {
                        self.tiles[tile_index2].texture.set_color_mod(255, 255, 255);
                        self.selected_tile = None;
                        return;
                    }

                    // test tile match
                    {
                        let tile1 = &self.tiles[tile_index];
                        let tile2 = &self.tiles[tile_index2];

                        if !Tile::matches(&tile1, &tile2) {
                            return;
                        }
                    }

                    // valid match
                    self.played.push(tile_index);
                    self.played.push(tile_index2);
                    self.update_meta_data();

                    self.tiles[tile_index2].texture.set_color_mod(255, 255, 255);
                    self.selected_tile = None;
                },
                None => {
                    self.tiles[tile_index].texture.set_color_mod(255, 127, 127);
                    self.selected_tile = Some(tile_index);
                }
            }
        }
    }

    pub fn undo(&mut self) {
        self.played.pop();
        self.played.pop();
        self.update_meta_data();
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        for (index, tile) in self.tiles.iter().enumerate() {
            if self.played.contains(&index) { continue; }

            let x = tile.position.x() as i32 * 23 + tile.position.z() as i32 * 5 + 20;
            let y = tile.position.y() as i32 * 29 - tile.position.z() as i32 * 5 + 15;

            renderer.copy(&self.side_texture, None, Rect::new(x - 5, y, 5, 62).unwrap());
            renderer.copy(&self.bottom_texture, None, Rect::new(x, y + 57, 46, 5).unwrap());

            renderer.copy(&tile.texture, None, Rect::new(x, y, 46, 57).unwrap());
        }
    }

    fn find_tile_index_by_coord(&self, x: i32, y: i32) -> Option<usize> {
        for (index, tile) in self.tiles.iter().enumerate().rev() {
            if self.played.contains(&index) || !self.reachable_tiles.contains(&index) { continue; }

            let tile_x = tile.position.x() as i32 * 23 + tile.position.z() as i32 * 5 + 15;
            let tile_y = tile.position.y() as i32 * 29 - tile.position.z() as i32 * 5 + 15;

            if x >= tile_x && x <= tile_x + 46 && y >= tile_y && y <= tile_y + 57 {
                return Some(index);
            }
        }
        None
    }

    fn update_meta_data(&mut self) {
        self.update_blocked_by_data();
        self.set_reachable_tiles();
    }

    fn update_blocked_by_data(&self) {
        for data in self.blocking_data.iter() {
            let index = data.tile_index;

            macro_rules! amount_blocked_by {
                ( $blocking_direction:expr ) => {
                    self.blocking_data
                        .iter()
                        .filter(|&data| !self.played.contains(&data.tile_index))
                        .fold(0, |count, data| {
                            if data.get_blocking($blocking_direction).contains(&index) {
                                count + 1
                            } else {
                                count
                            }
                        })
                }
            }

            data.blocked_by_verticaly.set(amount_blocked_by!(Verticaly));
            data.blocked_by_left.set(amount_blocked_by!(Left));
            data.blocked_by_right.set(amount_blocked_by!(Right));
        }
    }

    fn set_reachable_tiles(&mut self) {
        self.reachable_tiles = self.blocking_data
            .iter()
            .filter(|&data| {
                !self.played.contains(&data.tile_index) &&
                data.blocked_by_verticaly.get() == 0 &&
                (data.blocked_by_left.get() == 0 ||
                data.blocked_by_right.get() == 0)
            })
            .map(|data| data.tile_index)
            .collect();
    }

    // TODO: remove once we remove console printing
    fn get_top_tile_index_at_position(&self, x: u8, y: u8) -> Option<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|&(index, tile)| {
                tile.position.x() == x &&
                tile.position.y() == y &&
                !self.played.contains(&index)
            })
            .fold(None, |top_tile_index, (index, tile)| {
                if let Some(index) = top_tile_index {
                    let current_top_tile: &Tile = &self.tiles[index];
                    if current_top_tile.position.z() > tile.position.z() {
                        return top_tile_index
                    }
                }
                Some(index)
            })
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.tiles.iter().map(|tile| tile.position.x()).max().unwrap();
        let height = self.tiles.iter().map(|tile| tile.position.y()).max().unwrap();

        try!(write!(f, "   "));
        for column in 0..width {
            try!(write!(f, "{: >3} ", column));
        }
        try!(write!(f, "\n"));
        for _ in 0..width+1 {
            try!(write!(f, "----"));
        }
        try!(write!(f, "\n"));
        for row in 0..height {
            try!(write!(f, "{: >2}|", row));
            for column in 0..width {
                if let Some(index) = self.get_top_tile_index_at_position(column, row) {
                    try!(write!(f, "{:?}", self.tiles[index]));
                } else {
                    try!(write!(f, "    "));
                }
            }
            try!(write!(f, "\n"));
        }
        write!(f, "")
    }
}

// TODO: put positions in a file and read them from disk
//       use human readable format
static POSITIONS: [u32; 144] = [
    4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26,
    72, 74, 76, 78, 80, 82, 84, 86,
    134, 136, 138, 140, 142, 144, 146, 148, 150, 152,
    196, 198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218,
    224, 226, 252,
    260, 262, 264, 266, 268, 270, 272, 274, 276, 278, 280, 282,
    326, 328, 330, 332, 334, 336, 338, 340, 342, 344,
    392, 394, 396, 398, 400, 402, 404, 406,
    452, 454, 456, 458, 460, 462, 464, 466, 468, 470, 472, 474,

    1098, 1100, 1102, 1104, 1106, 1108,
    1162, 1164, 1166, 1168, 1170, 1172,
    1226, 1228, 1230, 1232, 1234, 1236,
    1290, 1292, 1294, 1296, 1298, 1300,
    1354, 1356, 1358, 1360, 1362, 1364,
    1418, 1420, 1422, 1424, 1426, 1428,

    2188, 2190, 2192, 2194,
    2252, 2254, 2256, 2258,
    2316, 2318, 2320, 2322,
    2380, 2382, 2384, 2386,

    3278, 3280,
    3342, 3344,

    4335,
];

pub fn create_board(renderer: &Renderer) -> Vec<Tile> {
    let tile_types = {
        let mut tile_types = Vec::new();
        for tile_type in TileType::iter() {
            for _ in 0..tile_type.max_allowed() {
                tile_types.push(*tile_type);
            }
        }
        tile_types
    };


    let tile_data = create_position_nodes();
    let starting_positions = get_starting_position_nodes(&tile_data);

    let mut tiles = vec![];
    let mut valid = false;
    while !valid {
        valid = true;
        let mut tile_factory = TileFactory {
            remaining_tile_types: tile_types.clone(),
            available_nodes: starting_positions.clone(),
            used_nodes: vec![],
            renderer: renderer,
        };

        while let Some(tiles_result) = tile_factory.get_tile_set() {
            if let Ok((tile1, tile2)) = tiles_result {
                tiles.push(tile1);
                tiles.push(tile2);
            } else {
                valid = false;
                break;
            }
        }

        // if board isn't valid we clear the vec and try again
        if !valid { tiles.clear(); }
    }

    tiles.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        if a.position.z() < b.position.z() { Less }
        else if a.position.z() > b.position.z() { Greater }
        else if a.position.x() > b.position.x() { Less }
        else if a.position.x() < b.position.x() { Greater }
        else if a.position.y() < b.position.y() { Less }
        else { Greater }
    });

    tiles
}

fn create_position_nodes() -> Vec<Rc<PositionNode>> {
    let tiles =
        POSITIONS.iter()
                 .map(|&position| {
                     let x = ((position % 1024) % 32) as u8;
                     let y = ((position % 1024) / 32) as u8;
                     let z = (position / 1024) as u8;
                     Rc::new(PositionNode::new(TilePosition::new(x, y, z)))
                 })
                 .collect::<Vec<Rc<PositionNode>>>();

    for tile in &tiles {
        for other_tile in &tiles {
            if tile.position.is_neighbour_of(&other_tile.position) {
                tile.neighbours.borrow_mut().push(Rc::downgrade(other_tile));
            }
        }
    }
    tiles
}

fn get_starting_position_nodes(positions: &Vec<Rc<PositionNode>>) -> Vec<Rc<PositionNode>> {
    let ground_position_graphs = get_ground_position_graphs(positions);

    let mut rng = rand::thread_rng();
    let mut starting_positions = vec![];
    for graph in &ground_position_graphs {
        let rows: HashSet<u8> = graph
            .iter()
            .map(|node| {
                node.position.y()
            })
            .fold(RefCell::new(HashSet::new()), |rows, y| {
                rows.borrow_mut().insert(y);
                rows
            })
            .into_inner();

        match rows.len() {
            0 => unreachable!(),
            1 => {
                let random_index = Range::new(0, graph.len()).ind_sample(&mut rng);
                starting_positions.push(graph[random_index].clone());
            },
            _ => {
                // TODO: currently assumes row count to be 3 if not 1, make the code not depend on this to support different board setups

                let mut add_random_node_from_row = |row| {
                    let count = graph.iter()
                        .filter(|&node| node.position.y() == row)
                        .count();
                    let random_index = Range::new(0, count).ind_sample(&mut rng);
                    let (_, node) = graph.iter()
                        .filter(|&node| node.position.y() == row)
                        .enumerate()
                        .filter(|&(index, _)| index == random_index)
                        .next()
                        .unwrap();

                    starting_positions.push(node.clone());
                };

                let row = *rows.iter().min().unwrap();
                add_random_node_from_row(row);

                let row = *rows.iter().max().unwrap();
                add_random_node_from_row(row);

            },
        }
    }

    starting_positions
}

fn get_ground_position_graphs(positions: &Vec<Rc<PositionNode>>) -> Vec<Vec<Rc<PositionNode>>> {
    let mut ground_node_graphs: Vec<Vec<Rc<PositionNode>>> = vec![];
    let mut visited_nodes: HashSet<Rc<PositionNode>> = HashSet::new();
    for position_data in positions {
        if !visited_nodes.contains(position_data) {
            ground_node_graphs.push(vec![]);

            fn traverse_nodes(position_data: &Rc<PositionNode>,
                              visited: &mut HashSet<Rc<PositionNode>>,
                              graph: &mut Vec<Rc<PositionNode>>) {
                if visited.contains(position_data) { return; }
                visited.insert(position_data.clone());
                if position_data.position.z() == 0 { graph.push(position_data.clone()); }
                for neighbour in position_data.neighbours.borrow().iter() {
                    traverse_nodes(&neighbour.upgrade().unwrap(), visited, graph);
                }
            }

            traverse_nodes(position_data, &mut visited_nodes, ground_node_graphs.last_mut().unwrap());
        }
    }
    ground_node_graphs
}


#[derive(Debug)]
struct InvalidBoardError;

struct PositionNode {
    pub position: TilePosition,
    pub neighbours: RefCell<Vec<Weak<PositionNode>>>,
    pub played: bool,
}

impl PositionNode {
    fn new(position: TilePosition) -> Self {
        PositionNode {
            position: position,
            neighbours: RefCell::new(vec![]),
            played: false,
        }
    }
}

impl PartialEq for PositionNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for PositionNode { }

impl Hash for PositionNode {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut hash = (self.position.x() as u32) << 16;
        hash |= (self.position.y() as u32) << 8;
        hash |= self.position.z() as u32;
        state.write_u32(hash);
    }
}

struct TileFactory<'a> {
    pub remaining_tile_types: Vec<TileType>,
    pub available_nodes: Vec<Rc<PositionNode>>,
    pub used_nodes: Vec<Rc<PositionNode>>,
    pub renderer: &'a Renderer<'a>,
}

impl<'a> TileFactory<'a> {
    fn get_tile_set(&mut self) -> Option<Result<(Tile, Tile), InvalidBoardError>> {
        if self.available_nodes.is_empty() { return None; }

        let mut rng = rand::thread_rng();

        let random_index = Range::new(0, self.remaining_tile_types.len() / 2).ind_sample(&mut rng) * 2;
        let tile_type1 = self.remaining_tile_types.remove(random_index);
        let tile_type2 = self.remaining_tile_types.remove(random_index);

        let random_index = Range::new(0, self.available_nodes.len()).ind_sample(&mut rng);
        let node1 = self.available_nodes.swap_remove(random_index);
        self.used_nodes.push(node1.clone());

        if self.available_nodes.is_empty() { return Some(Err(InvalidBoardError)); };
        let random_index = Range::new(0, self.available_nodes.len()).ind_sample(&mut rng);
        let node2 = self.available_nodes.swap_remove(random_index);
        self.used_nodes.push(node2.clone());

        for neighbour in node1.neighbours.borrow().iter() {
            let node = &neighbour.upgrade().unwrap();
            if !self.available_nodes.contains(node) && !self.used_nodes.contains(node) {
                self.available_nodes.push(node.clone());
            }
        }
        for neighbour in node2.neighbours.borrow().iter() {
            let node = &neighbour.upgrade().unwrap();
            if !self.available_nodes.contains(node) && !self.used_nodes.contains(node) {
                self.available_nodes.push(node.clone());
            }
        }

        let tile1 = Tile::new(node1.position.clone(), tile_type1, self.renderer);
        let tile2 = Tile::new(node2.position.clone(), tile_type2, self.renderer);
        Some(Ok((tile1, tile2)))
    }
}
