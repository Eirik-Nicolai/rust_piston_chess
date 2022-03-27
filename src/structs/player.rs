use std::collections::HashMap;
use opengl_graphics::Texture;

use crate::actions::movement as Movement;
use crate::actions::movement::Position;


pub type Pieces<'a> = HashMap<String, PieceData<'a>>;


#[derive(Clone)]
pub struct MoveSet {
    pub dir: Vec<Movement::Direction>,
    pub amount: u8
}

#[derive(Clone)]
pub struct PieceData<'a> {
    pub pos: Position,
    pub texture: &'a Texture,
    pub moveset: MoveSet
}
impl<'a> PieceData<'a> {
    pub fn new(pos:Position, texture:&'a Texture, directions:Vec<Movement::Direction>, step_amount:u8) -> Self {
        PieceData {
            pos,
            texture,
            moveset:MoveSet {
                dir: directions,
                amount: step_amount
            }
        }
    }
}

pub struct Player<'a> {
    pub pieces: Pieces<'a>,
    pub lost_pieces: Vec<String>
}
impl<'a> Player<'a> {
    pub fn new(names:Vec<String>,data:Vec<PieceData<'a>>) -> Self
    {
        let mut pieces = HashMap::new();
        for i in 0 .. names.len() 
        {
            pieces.insert(names[i].clone(), data[i].clone());
        }
        Player {
            pieces,
            lost_pieces: Vec::new()
        }
    }
}

pub fn other_turn(turn:&char) -> char
{
    match turn 
    {
        'w' => 'b',
        'b' => 'w',
        _ => '_'    //this shouldn't happen
    }
}