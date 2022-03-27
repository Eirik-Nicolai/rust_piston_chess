extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use std::collections::{HashMap};
use std::path::Path;

use graphics::{Context, Image, DrawState};
use piston::{ButtonEvent,RenderEvent,WindowSettings, MouseCursorEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonState};
use piston::input::{MouseButton};
use graphics::rectangle::square;

use glutin_window::GlutinWindow;
use opengl_graphics::{Filter,GlGraphics,OpenGL, TextureSettings, Texture};

extern crate image as im;

mod structs;
mod actions;

use structs::visuals as Visuals;
use structs::player as Player;
use actions::actions as Actions;
use actions::movement as Movement;

/// Translate board positiont to screen position for a piece
fn translate_piece_position(pos:Movement::Position) -> (f64,f64)
{
    ((Visuals::PIXEL_SIZE * pos.0 as i32) as f64,(Visuals::PIXEL_SIZE * pos.1 as i32) as f64)
}

// TODO make good
fn display_taken_pieces(
    players:&mut HashMap<char,Player::Player>,
    textures:&HashMap<char,HashMap<char,&Texture>>,
    c: & Context, 
    g: &mut GlGraphics,
    drawstate: &DrawState)
{
    let mut player_offs = 0;
    for (id,data) in players
    {
        data.lost_pieces.sort();
        let mut amount_offs = 0;
        for piece in &data.lost_pieces
        {
            let piece_type =  piece.chars().nth(1).unwrap();
            let (offs_x,offs_y) = match piece_type
            {
                'p' => (0,0),
                'k' => (30,0),
                'q' => (60,0),
                'b' => (0,40),
                'r' => (30,40),
                'n' => (60,40),
                _ => (0,0)
            };
            
            //Create the image object and attach a square Rectangle object inside.
            let image = Image::new().rect(square(15.0+(offs_x
                +(amount_offs*10) + (700*player_offs))as f64,
                 1025.0+offs_y as f64,(Visuals::PIXEL_SIZE/3) as f64));
            image.draw(textures[&id][&piece_type], &drawstate, c.transform, g);
            amount_offs = amount_offs + 1;                
        }
        player_offs = player_offs+1;
    }
    
}

fn display_players(
    black_player:Option<&Player::Pieces>,
    white_player:Option<&Player::Pieces>,
    c: & Context, 
    g: &mut GlGraphics,
    drawstate: &DrawState)
{
    if black_player.is_some()
    {
        for (_,data) in black_player.unwrap()
        {
            let (x,y) = translate_piece_position(data.pos);
            //Create the image object and attach a square Rectangle object inside.
            let image = Image::new().rect(square(x, y,Visuals::PIXEL_SIZE as f64));
            image.draw(data.texture, &drawstate, c.transform, g);                
        }
    }
    if white_player.is_some()
    {
        for (_,data) in white_player.unwrap() 
        {
            let (x,y) = translate_piece_position(data.pos);
            //Create the image object and attach a square Rectangle object inside.
            let image = Image::new().rect(square(x, y,Visuals::PIXEL_SIZE as f64));
            image.draw(data.texture, &drawstate, c.transform, g);                
        }
    }
}

/// If we have moves to show for a piece, render them
/// Legal moves are shown in varying shades of red 
fn display_selected_legal_moves(
    maybe_moves: &Option<Movement::Moves>,
    mouse_pos:&(u8,u8),
    c: & Context, 
    g: &mut GlGraphics)
{
    match maybe_moves
    {
        Some(moves) => 
        {
            for dir in moves
            {
                for pos in &dir.0
                {
                    let mut col = Visuals::COL_RED;
                    if pos == mouse_pos
                    {
                        col = Visuals::COL_RED_HALF;
                    }
                    let vt = Visuals::VisualTile::generate_square(&pos,col);
                    graphics::Rectangle::new(vt.outer_tile.col).draw(
                        vt.outer_tile.pos,
                        &c.draw_state,
                        c.transform,
                        g
                    );           
                }
                for pos in &dir.1
                {
                    let mut col = Visuals::COL_RED_HALF;
                    if pos == mouse_pos
                    {
                        col = Visuals::COL_RED_SOLID;
                    }
                    let vt = Visuals::VisualTile::generate_square(&pos,col);
                    graphics::Rectangle::new(vt.outer_tile.col).draw(
                        vt.outer_tile.pos,
                        &c.draw_state,
                        c.transform,
                        g
                    );           
                }
            }
        },
        None => {return;}
    }
}

fn update_square_piece_relations<'a>(squares: &'a mut Visuals::Squares, players: &'a HashMap<char,Player::Player>)
{
    for i in 0 .. Visuals::BOARD_PIXELS_COUNT
    {
        for j in 0 .. Visuals::BOARD_PIXELS_COUNT
        {
            squares[i as usize][j as usize].current_piece = None;
        }
    }

    for (_,p) in players
    {
        for (name, data) in &p.pieces
        {
            squares[data.pos.0 as usize][data.pos.1 as usize].current_piece = Some(name.into());
        }
    }
}

fn main() 
{
    // init game objects
    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new(
        "CHESS TIME BABEY",
        [Visuals::WINDOW_SIZE as f64,(Visuals::WINDOW_SIZE + 100) as f64])
        .exit_on_esc(true)
        .resizable(false);
    let mut window:GlutinWindow = settings.build().expect("Couldn't create window");

    let mut gl = GlGraphics::new(opengl);

    // game events
    let eventsettings = EventSettings::new();
    let mut events = Events::new(eventsettings);

    // init game assets
    //LOAD IMAGES/TEXTURES
    let texture_settings = TextureSettings::new().filter(
        Filter::Nearest);
    let drawstate = DrawState::new_alpha();

    let mut board_squares = vec![vec![Visuals::Square::empty(); Visuals::BOARD_PIXELS_COUNT as usize];Visuals::BOARD_PIXELS_COUNT as usize];
    for i in 0 .. Visuals::BOARD_PIXELS_COUNT 
    {
        for j in 0 .. Visuals::BOARD_PIXELS_COUNT
        {
            board_squares[i as usize][j as usize] = Visuals::Square::new(
                Visuals::VisualTile::new((i as u8,j as u8),4)
            );
            if i % 2 != 0 && j % 2 == 0
            {
                board_squares[i as usize][j as usize].tile.inner_tile.col = Visuals::COL_BLACK;
            }
            if j % 2 != 0 && i % 2 == 0
            {
                board_squares[i as usize][j as usize].tile.inner_tile.col = Visuals::COL_BLACK;
            }
        }
    }

    // these images comes from
    // https://commons.wikimedia.org/wiki/Category:PNG_chess_pieces/Standard_transparent
    let tex_black_king  = Texture::from_path(Path::new("assets/imgs/bking.png"),&texture_settings).unwrap();
    let tex_black_queen = Texture::from_path(Path::new("assets/imgs/bqueen.png"),&texture_settings).unwrap();
    let tex_black_bish = Texture::from_path(Path::new("assets/imgs/bbishop.png"),&texture_settings).unwrap();
    let tex_black_rook = Texture::from_path(Path::new("assets/imgs/brook.png"),&texture_settings).unwrap();
    let tex_black_knig = Texture::from_path(Path::new("assets/imgs/bknight.png"),&texture_settings).unwrap();
    let tex_black_pawn = Texture::from_path(Path::new("assets/imgs/bpawn.png"),&texture_settings).unwrap();
    
    let tex_white_king  = Texture::from_path(Path::new("assets/imgs/wking.png"),&texture_settings).unwrap();
    let tex_white_queen = Texture::from_path(Path::new("assets/imgs/wqueen.png"),&texture_settings).unwrap();
    let tex_white_bish = Texture::from_path(Path::new("assets/imgs/wbishop.png"),&texture_settings).unwrap();
    let tex_white_rook = Texture::from_path(Path::new("assets/imgs/wrook.png"),&texture_settings).unwrap();
    let tex_white_knig = Texture::from_path(Path::new("assets/imgs/wknight.png"),&texture_settings).unwrap();
    let tex_white_pawn = Texture::from_path(Path::new("assets/imgs/wpawn.png"),&texture_settings).unwrap();

    let textures = HashMap::from([
        ('w',HashMap::from([
            ('k',&tex_white_king),
            ('q',&tex_white_queen),
            ('b',&tex_white_bish),
            ('r',&tex_white_rook),
            ('n',&tex_white_knig),
            ('p',&tex_white_pawn)
        ])),
        ('b',HashMap::from([
            ('k',&tex_black_king),
            ('q',&tex_black_queen),
            ('b',&tex_black_bish),
            ('r',&tex_black_rook),
            ('n',&tex_black_knig),
            ('p',&tex_black_pawn)
        ]))
    ]);

    // INIT PLAYERS
    let mut players = HashMap::from([
        ('b', 
        Player::Player::new(
            vec![
                "bk".to_string(),    
                "bq".to_string(),
                "bb1".to_string(),   
                "bb2".to_string(),
                "br1".to_string(),
                "br2".to_string(),
                "bn1".to_string(),
                "bn2".to_string(),
                "bp1".to_string(),
                "bp2".to_string(),
                "bp3".to_string(),
                "bp4".to_string(),
                "bp5".to_string(),
                "bp6".to_string(),
                "bp7".to_string(),
                "bp8".to_string(),
                ], 
            vec![
                Player::PieceData::new((4,0), &tex_black_king, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast,Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                1),
                Player::PieceData::new((3,0), &tex_black_queen, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast,
                Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                8),
                Player::PieceData::new((2,0), &tex_black_bish, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                8),
                Player::PieceData::new((5,0), &tex_black_bish, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                8),
                Player::PieceData::new((0,0), &tex_black_rook, 
                vec![Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                8),
                Player::PieceData::new((7,0), &tex_black_rook, 
                vec![Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                8),
                Player::PieceData::new((1,0), &tex_black_knig, 
                vec![Movement::Direction::Horse], 
                8),
                Player::PieceData::new((6,0), &tex_black_knig, 
                vec![Movement::Direction::Horse], 
                8),
                Player::PieceData::new((0,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
                Player::PieceData::new((1,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
                Player::PieceData::new((2,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
                Player::PieceData::new((3,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
                Player::PieceData::new((4,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
                Player::PieceData::new((5,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
                Player::PieceData::new((6,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
                Player::PieceData::new((7,1), &tex_black_pawn, 
                vec![Movement::Direction::South,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                2),
            ]
        )),
        ('w', 
        Player::Player::new(
            vec![
                "wk".to_string(),    
                "wq".to_string(),
                "wb1".to_string(),   
                "wb2".to_string(),
                "wr1".to_string(),
                "wr2".to_string(),
                "wn1".to_string(),
                "wn2".to_string(),
                "wp1".to_string(),
                "wp2".to_string(),
                "wp3".to_string(),
                "wp4".to_string(),
                "wp5".to_string(),
                "wp6".to_string(),
                "wp7".to_string(),
                "wp8".to_string(),
                ], 
            vec![
                Player::PieceData::new((4,7), &tex_white_king, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast,Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                1),
                Player::PieceData::new((3,7), &tex_white_queen, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast,
                Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                8),
                Player::PieceData::new((2,7), &tex_white_bish, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                8),
                Player::PieceData::new((5,7), &tex_white_bish, 
                vec![Movement::Direction::NorthWest,Movement::Direction::NorthEast,Movement::Direction::SouthWest,Movement::Direction::SouthEast], 
                8),
                Player::PieceData::new((0,7), &tex_white_rook, 
                vec![Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                8),
                Player::PieceData::new((7,7), &tex_white_rook, 
                vec![Movement::Direction::North, Movement::Direction::South, Movement::Direction::West, Movement::Direction::East], 
                8),
                Player::PieceData::new((1,7), &tex_white_knig, 
                vec![Movement::Direction::Horse], 
                8),
                Player::PieceData::new((6,7), &tex_white_knig, 
                vec![Movement::Direction::Horse], 
                8),
                Player::PieceData::new((0,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
                Player::PieceData::new((1,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
                Player::PieceData::new((2,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
                Player::PieceData::new((3,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
                Player::PieceData::new((4,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
                Player::PieceData::new((5,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
                Player::PieceData::new((6,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
                Player::PieceData::new((7,6), &tex_white_pawn, 
                vec![Movement::Direction::North,Movement::Direction::NorthWest,Movement::Direction::NorthEast], 
                2),
            ]
        )),
    ]);

    // SET GAME STATES
    let mut current_turn = 'w';
    let mut current_mouse_pos = [0.0; 2];
    let mut current_moves:Option<Movement::Moves> = None;
    let mut current_piece = None;
    while let Some(event) = events.next(&mut window)
    {
        if let Some(m) = event.mouse_cursor_args() 
        {
            current_mouse_pos = m;
        }
        
        if let Some(b) = event.button_args()
        {
            if b.state == ButtonState::Press
            {
                match b.button
                {
                    Button::Mouse(MouseButton::Left) => {
                        let sq_pos = Actions::get_current_square(current_mouse_pos);
                        // did we click to move
                        let mut wants_to_move = false;
                        match &current_moves
                        {
                            Some(maybe_moves) => {
                                'outer: for moves in maybe_moves
                                {
                                    for m in &moves.0
                                    {
                                        if *m == sq_pos
                                        {   // yes we did
                                            wants_to_move = true;
                                            break 'outer;
                                        }
                                    }
                                }
                            },
                            None => {
                                // no we didnt
                            }
                        }
                        if wants_to_move
                        {
                            current_turn = Movement::move_piece(&mut players,&current_turn,&sq_pos, &current_piece);
                        }
                        
                        // did we click piece
                        //are we taking a piece
                        let clicked_piece = Actions::get_currently_clicked_piece(&mut board_squares,&sq_pos);
                        if let Some(moves) = &current_moves
                        {
                            for (_,targets) in moves
                            {
                                for target in targets
                                {
                                    if *target == sq_pos
                                    {
                                        if let Some(p) = &clicked_piece 
                                        {
                                            current_turn = Movement::move_piece_to_target(
                                                &mut players,
                                                &current_turn,
                                                &current_piece,
                                                &p);
                                        }
                                    }
                                }
                            }
                        }
                        //or are we selecting a piece
                        (current_moves,current_piece) = Actions::get_moves_for_piece_by_player(&mut board_squares, 
                            &mut current_piece,
                            &players[&current_turn],
                            &sq_pos);
                    },
                    _ => ()
                }
            }
        }
        
        // things may or may not have happened
        // check and update the board before 
        // showing anything 
        update_square_piece_relations(&mut board_squares, &players);

        if let Some(r) = event.render_args()
        {    
            gl.draw(r.viewport(), |context, graphics|{
                graphics::clear(Visuals::COL_WHITE, graphics);

                // render out the chess board
                for i in 0 .. Visuals::BOARD_PIXELS_COUNT 
                {
                    for j in 0 .. Visuals::BOARD_PIXELS_COUNT
                    {
                        // TODO
                        // fix this inner/outer tile garbage shit
                        let vt = board_squares[i as usize][ j as usize].tile;
                        graphics::Rectangle::new(vt.outer_tile.col).draw(
                            vt.outer_tile.pos,
                            &context.draw_state,
                            context.transform,
                            graphics
                        );
                        graphics::Rectangle::new(vt.inner_tile.col).draw(
                            vt.inner_tile.pos,
                            &context.draw_state,
                            context.transform,
                            graphics
                        );
                    }
                }

                // render the pieces on the board
                display_players(Some(&players[&'b'].pieces), Some(&players[&'w'].pieces), 
                    &context, graphics, &drawstate);
                
                // if a player selected a piece, show where it can move
                display_selected_legal_moves(&current_moves,
                    &Actions::get_current_square(current_mouse_pos),
                    &context,graphics);

                // display any pieces that have been taken
                // TODO make not shit
                display_taken_pieces(&mut players,
                    &textures,
                    &context, graphics, &drawstate);
            });
        }
    }
}