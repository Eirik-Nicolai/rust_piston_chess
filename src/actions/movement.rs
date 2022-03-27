
use std::collections::HashMap;

use crate::structs::player as Player;
use crate::structs::visuals as Visuals;


pub type Position = (u8,u8);
pub type Moves = Vec<(Vec<Position>,Vec<Position>)>;

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Horse
}

pub fn get_horse_moves(pos:&(u8,u8)) -> Vec<(u8,u8)>
{
    // fuck you horse
    let mut mvs = Vec::new(); 
    if pos.0 < 6 && pos.1 < 7 {mvs.push(((pos.0+2),(pos.1+1)));}
    if pos.0 < 7 && pos.1 < 6 {mvs.push(((pos.0+1),(pos.1+2)));}
    if pos.0 < 6 && pos.1 > 0 {mvs.push(((pos.0+2),(pos.1-1)));}
    if pos.0 < 7 && pos.1 > 1 {mvs.push(((pos.0+1),(pos.1-2)));}
    if pos.0 > 1 && pos.1 > 0 {mvs.push(((pos.0-2),(pos.1-1)));}
    if pos.0 > 0 && pos.1 > 1 {mvs.push(((pos.0-1),(pos.1-2)));}
    if pos.0 > 1 && pos.1 < 7 {mvs.push(((pos.0-2),(pos.1+1)));}
    if pos.0 > 0 && pos.1 < 6 {mvs.push(((pos.0-1),(pos.1+2)));}
    mvs
}
/// Basically, we take the range of piece-position to the edge of the board in a position,
/// and add all the squares in that path to a vector, starting at our piece an going outwards
/// This is so it's easier to remove obstructed paths for later
pub fn get_straight_moves_from_range(pos:&u8,steps:Vec<u8>, amount:&u8, get_tuple: &dyn Fn(u8,u8)->(u8,u8)) -> Vec<(u8,u8)>
{
    let mut mvs = Vec::new();
    let mut moves_left = *amount;
    for i in steps
    {
        moves_left = moves_left-1;
        mvs.push(get_tuple(*pos,i));
        if moves_left == 0
        {
            break;
        }
    }
    mvs
}
/// Same as straight_moves, but moving at an angle needs more consideration for where the piece is
/// in relation to the path
pub fn get_angle_moves_from_range(pos:&(u8,u8), steps:Vec<u8>,amount:&u8,is_westbound:bool,get_tuple: &dyn Fn(u8,u8)->(u8,u8)) -> Vec<(u8,u8)>
{
    let mut mvs = Vec::new();
    let mut moves_left = *amount+1;
    let x:i8 = if is_westbound 
        {
            -1
        }
        else
        {
            1
        };
    for i in steps
    {
        moves_left = moves_left - 1;
        if i < pos.1 
        {
            let pos_x = pos.0 as i8+(pos.1 as i8-i as i8)*x;
            if pos_x >= 0 && pos_x < 8 
            {
                mvs.push(get_tuple((pos.0 as i8+((pos.1 as i8-i as i8)*x))as u8,i));
            }
        }
        if i > pos.1 
        {
            let pos_x = pos.0 as i8+(i as i8-pos.1 as i8)*x;
            if pos_x >= 0 && pos_x < 8 
            {
                mvs.push(get_tuple((pos.0 as i8+((i as i8-pos.1 as i8)*x))as u8,i));
            }
        }
        if moves_left == 0
        {
            break;
        }
    }

    mvs
}
pub fn tuple(x:u8,y:u8) -> (u8,u8)
{(x,y)}
pub fn rev_tuple(x:u8,y:u8) -> (u8,u8)
{(y,x)}

pub fn get_legal_moves<'a>(ms: &Player::MoveSet, pos: &'a Position, squares: &Visuals::Squares, piece:&String) -> Moves
{
    let mut m:Moves = Vec::new();
    // --- GET ALL MOVES ---
    for dir in &ms.dir
    {
        //TODO this is shit we hate this
        let dir_moves = match dir {
            Direction::North    => get_straight_moves_from_range(&pos.0, 
                (0..pos.1).rev().collect(), 
                &ms.amount, &tuple),
            Direction::South    => get_straight_moves_from_range(&pos.0, 
                (pos.1+1..(Visuals::BOARD_PIXELS_COUNT as u8)).collect(), 
                &ms.amount, 
                &tuple),
            Direction::West     => get_straight_moves_from_range(&pos.1, 
                (0..pos.0).rev().collect(), 
                &ms.amount,
            &rev_tuple),
            Direction::East     => get_straight_moves_from_range(&pos.1, 
                (pos.0+1..(Visuals::BOARD_PIXELS_COUNT as u8)).collect(), 
                &ms.amount,
                &rev_tuple),
            Direction::NorthWest    => get_angle_moves_from_range(&pos,
                (0..pos.1).rev().collect(),
                &ms.amount,
                true,
                &tuple),
            Direction::NorthEast    => get_angle_moves_from_range(&pos, 
                (0..pos.1).rev().collect(),
                &ms.amount,
                false,
                &tuple),
            Direction::SouthWest    => get_angle_moves_from_range(&pos, 
                (pos.1..Visuals::BOARD_PIXELS_COUNT as u8).collect(), 
                &ms.amount,
                true,
                &tuple),
            Direction::SouthEast    => get_angle_moves_from_range(&pos, 
                (pos.1..Visuals::BOARD_PIXELS_COUNT as u8).collect(),
                &ms.amount,
                false,
                &tuple),
            Direction::Horse => get_horse_moves(&pos)
        };
        m.push((dir_moves,vec![]));
    }

    
    // --- REMOVE ILLEGAL MOVES ---
    let mut obstructed_vec = Vec::new();
    let mut targets = Vec::new();
    if let Direction::Horse = ms.dir[0]
    { //fuck you horse
        let mut illegal_moves = Vec::new();
        for i in 0..m.len()
        {
            for j in 0..m[i].0.len()
            {
                match &squares[m[i].0[j].0 as usize][m[i].0[j].1 as usize].current_piece 
                {
                    Some(s) => {
                        if s.chars().next().unwrap() != piece.chars().next().unwrap()
                        { 
                            targets.push(((m[i].0[j].0,m[i].0[j].1),i as i16));
                        }
                        illegal_moves.push(j);
                    }
                    None => {}
                }
            }
            illegal_moves.reverse();
        }
        for i_m in illegal_moves
        {
            m[0].0.remove(i_m);
        }
    }
    else
    {   // basically we're checking if the path in any direction hits another piece
        // and if it does we mark the index. Since we set the moves going outward
        // from our piece we can just tick them off after the index.
        // 
        // exception is pawns, who moves angular to take targets, we only add
        // that if we find a target
        //
        // if a opposite-player piece is found we store it instead 
        for i in 0..m.len()
        {
           let mut found = false;
            for j in 0..m[i].0.len()
            {
                match &squares[m[i].0[j].0 as usize][m[i].0[j].1 as usize].current_piece
                {
                    Some(s) => {
                        if piece.chars().nth(1).unwrap() == 'p'
                        {
                            //something in our angular paths, as pawns
                            if s.chars().next().unwrap() != piece.chars().next().unwrap()
                                && i != 0
                            {
                                targets.push(((m[i].0[j].0,m[i].0[j].1),i as i16));
                            }
                        }
                        //basically something in our path and we're not a pawn
                        if s.chars().next().unwrap() != piece.chars().next().unwrap() && piece.chars().nth(1).unwrap() != 'p' 
                        {
                            targets.push(((m[i].0[j].0,m[i].0[j].1),i as i16));
                        }
                        obstructed_vec.push(Some(j));
                        found = true;
                        break;
                    }
                    None => {
                        // as pawns if our angular path is unobstructed, we don't want to see it
                        if piece.chars().nth(1).unwrap() == 'p' 
                        && i != 0
                        {
                            obstructed_vec.push(Some(j));
                            found = true;
                            break;
                            
                        }
                    },
                }
            }
            if !found
            {
                obstructed_vec.push(None);
            }
        }
        // we have all our obstructions, remove the paths after
        for i in 0..obstructed_vec.len()
        {
            match obstructed_vec[i] 
            {
                None => continue,
                Some(pos) => {
                    let len = m[i].0.len();
                    for _ in pos as usize..len
                    {
                        m[i].0.remove(pos);
                    }
                }    
            }
        }
    };
    // add back targets as lgal moves
    // for visual purposes
    for (target,indx) in targets
    {
        m[indx as usize].1.push(target);
    }
    m
}

pub fn move_piece_to_target<'a>(players:&'a mut HashMap<char,Player::Player>, curr_turn:&char, attacking_piece:&Option<String>, prey_piece:&String) -> char
{
    let oth_turn = Player::other_turn(&curr_turn);
    match players.get_mut(&oth_turn).unwrap().pieces.remove_entry(prey_piece)
    {
        Some((id,data)) => {
            players.get_mut(&oth_turn).unwrap().lost_pieces.push(id);
            move_piece(players,&curr_turn, &data.pos, attacking_piece);
        },
        None => println!("Somethig went super wrong")
    }
    
    oth_turn
}

pub fn move_piece<'a>(players:&mut HashMap<char,Player::Player>,curr_turn:&char, pos:&Position, maybe_piece:&'a Option<String>) -> char
{
    if let Some(piece) = maybe_piece
    {
        let piecedata = players.get_mut(&curr_turn).unwrap().pieces.get_mut(piece).unwrap();
        piecedata.pos = *pos;
        if piece.as_str().chars().nth(1) == Some('p') && piecedata.moveset.amount == 2
        {
            piecedata.moveset.amount = piecedata.moveset.amount-1;
        }
        if piece.chars().nth(1).unwrap() == 'p'
        {
            // TODO pawns are supposed to turn into queens here? I think
            if piece.chars().nth(0).unwrap() == 'b'
            {
                if pos.1 == 7
                {
                    println!("queen slay");
                }
            }
            else 
            {
                if pos.1 == 0
                {
                    println!("queen slay");
                }
            }
        }
    }
    Player::other_turn(&curr_turn)
}