use crate::structs::player::{Player};
use crate::structs::visuals as Visuals;
use crate::actions::movement as Movement;

use crate::actions::movement::{get_legal_moves};


pub fn get_currently_clicked_piece<'a>(board:&'a Visuals::Squares,pos:&Movement::Position) -> Option<String>
{
    match &board[pos.0 as usize][pos.1 as usize].current_piece
    {
        Some(clicked_piece) => Some(clicked_piece.to_string()),
        None => {None}
    }
}
pub fn get_moves_for_piece_by_player<'a>(board:&'a Visuals::Squares,curr_piece:&mut Option<String>,player:&'a Player,pos:&Movement::Position) -> (Option<Movement::Moves>,Option<String>)
{
    match &board[pos.0 as usize][pos.1 as usize].current_piece
    {
        Some(clicked_piece) => {
            let do_change = match curr_piece
            {
                Some(curr) => {
                    if clicked_piece == curr {false}
                    else {true}
                },
                None => {true}
            };
            let mut ret = None;
            if do_change 
            {
                if let Some(ms) = player.pieces.get(clicked_piece)
                {
                    ret = Some(get_legal_moves(&ms.moveset.clone(), &pos, &board,&clicked_piece));
                }
            }
            else
            {
                return (None,None);
            }
            (ret,Some(clicked_piece.to_string()))
        },
        None => {(None,None)}
    }
}
/// translate mouse pos to a square on the board
pub fn get_current_square<'a>(pos: [f64; 2]) -> Movement::Position
{
    let (mut row, mut col) = (0,0);
    
    for i in 0 .. Visuals::BOARD_PIXELS_COUNT {
        if ((Visuals::PIXEL_SIZE * i) as f64 ) < pos[0] && ((Visuals::PIXEL_SIZE * (i+1)) as f64) >= pos[0]
        {
            row = i;
            break;
        }
    }
    for j in 0 .. Visuals::BOARD_PIXELS_COUNT {
        if ((Visuals::PIXEL_SIZE * j) as f64 ) < pos[1] && ((Visuals::PIXEL_SIZE * (j+1))as f64) >= pos[1]
        {
            col = j;
            break;
        }
    }
    (row as u8,col as u8)
}