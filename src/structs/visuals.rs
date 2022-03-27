
use crate::actions::movement::Position;


pub const PIXEL_SIZE: i32 = 128;
pub const BOARD_PIXELS_COUNT: i32 = 8;
pub const WINDOW_SIZE:i32 = PIXEL_SIZE * BOARD_PIXELS_COUNT;

pub const COL_WHITE :[f32; 4] = [0.85,0.85,0.85,1.0];
pub const COL_BLACK :[f32; 4] = [0.15,0.15,0.15,1.0];
pub const COL_RED   :[f32; 4] = [1.0, 0.0, 0.0, 0.2];
pub const COL_RED_HALF   :[f32; 4] = [1.0, 0.0, 0.0, 0.6];
pub const COL_RED_SOLID   :[f32; 4] = [1.0, 0.0, 0.0, 0.9];

pub type Squares<'a> = Vec<Vec<Square>>;

#[derive(Clone, Copy,std::default::Default)]
pub struct Tile {
    pub col: [f32; 4],
    pub pos: [f64; 4]
}

impl Tile {
    pub fn white(pos: [f64;4]) -> Self {
        Tile { col: COL_WHITE, pos }
    }

    pub fn black(pos: [f64;4]) -> Self {
        Tile {col: COL_BLACK, pos}
    }

    pub fn generate(pos: [f64;4], col:[f32; 4] ) -> Self {
        Tile {col, pos}
    }
}

#[derive(Clone, Copy,std::default::Default)]
pub struct VisualTile {
    pub inner_tile: Tile,
    pub outer_tile: Tile,
}

impl VisualTile {
    pub fn new((x,y):Position, border_width:i32) -> Self 
    {
        VisualTile {
            inner_tile: Tile::white([
                (((PIXEL_SIZE) * x as i32) + border_width/2) as f64,
                (((PIXEL_SIZE) * y as i32) + border_width/2) as f64,
                (PIXEL_SIZE - border_width)   as f64,
                (PIXEL_SIZE - border_width)   as f64
            ]),
            outer_tile: Tile::black([
                (PIXEL_SIZE * x as i32) as f64,
                (PIXEL_SIZE * y as i32) as f64,
                PIXEL_SIZE as f64,
                PIXEL_SIZE as f64
            ]),
        }
    }

    pub fn generate_square((x,y): &Position, col: [f32; 4]) -> VisualTile
    {
        VisualTile { 
            outer_tile: Tile::generate([
                (PIXEL_SIZE * *x as i32) as f64,
                (PIXEL_SIZE * *y as i32) as f64,
                PIXEL_SIZE as f64,
                PIXEL_SIZE as f64
                ],
                col
            ),
            inner_tile: Tile::white([0.0;4])
        }
    }

    pub fn empty() -> Self 
    {
        VisualTile { 
            outer_tile: Tile::black([0.0;4]), 
            inner_tile: Tile::white([0.0;4])
        }
    }
}

#[derive(Clone, std::default::Default)]
pub struct Square{
    pub current_piece: Option<String>,
    pub tile: VisualTile
}

impl Square{
    pub fn new(vt: VisualTile) -> Self
    {
        Square{
            current_piece: None,
            tile: vt
        }
    }

    pub fn empty() -> Self
    {
        Square {
            current_piece: None,
            tile: VisualTile::empty()
        }
    }
}