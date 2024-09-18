use std::fmt::Error;

use crate::utils::sets::cartesian_product;

use super::board::{in_check, Board};
use std::fmt;

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum PieceType {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    KING,
    QUEEN,
    EMPTY,
}

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub fn uniq_moves(list: Vec<Move>) -> Vec<Move> {
    list.into_iter()
        .collect::<HashSet<Move>>()
        .into_iter()
        .collect()
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::PAWN => write!(f, "Pawn"),
            PieceType::ROOK => write!(f, "Rook"),
            PieceType::KNIGHT => write!(f, "Knight"),
            PieceType::BISHOP => write!(f, "Bishop"),
            PieceType::KING => write!(f, "King"),
            PieceType::QUEEN => write!(f, "Queen"),
            PieceType::EMPTY => write!(f, "Empty"),
        }
    }
}
// x, y
// The new position that a piece moves to
#[derive(Clone, Copy, PartialOrd, Ord, Debug)]
pub struct Move(pub i32, pub i32);

impl Eq for Move {}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("x: {}, y: {}", self.0, self.1);
        Ok(())
    }
}
impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == self.1
    }
}
impl Hash for Move {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    WHITE,
    BLACK,
    EMPTY,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::BLACK => write!(f, "Black"),
            Color::EMPTY => write!(f, "Empty"),
            Color::WHITE => write!(f, "White"),
        }
    }
}
#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool,
}
// Illegal moves here means a move, where after the move the team is in check
fn filter_illegal_moves(
    board: &Board,
    psuedo_legal_moves: Vec<Move>,
    color: Color,
    x: i32,
    y: i32,
) -> Vec<Move> {
    let mut board_copy = board.clone();
    let mut legal_moves: Vec<Move> = Vec::new();

    // check what happens after applying moves
    for piece_move in psuedo_legal_moves {
        simulate_piece_move(&mut board_copy, piece_move, x, y).ok();

        if !in_check(board_copy, color) {
            legal_moves.push(piece_move);
        }
        // undo move for next iteration
        board_copy = board.clone();
    }

    return legal_moves;
}

fn simulate_piece_move(board: &mut Board, piece_move: Move, x: i32, y: i32) -> Result<(), &str> {
    if x > 7 || y > 7 || x < 0 || x < 0 {
        return Err("Invalid move variable");
    }

    let piece = board.pieces[x as usize][y as usize];
    board.pieces[piece_move.1 as usize][piece_move.0 as usize] = piece;
    board.pieces[y as usize][x as usize] = Piece {
        color: Color::EMPTY,
        piece_type: PieceType::EMPTY,
        has_moved: false,
    };

    return Ok(());
}

// Given a position, return the next valid positions
pub fn get_legal_moves(board: Board, x: i32, y: i32, color: Color) -> Vec<Move> {
    let piece = board.pieces[y as usize][x as usize];

    let moves = match piece.piece_type {
        PieceType::BISHOP => bishop_legal_moves(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            board,
            piece.color,
        ),
        PieceType::EMPTY => vec![],
        PieceType::KING => king_legal_moves(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            board,
            piece.color,
        ),
        PieceType::KNIGHT => knight_legal_moves(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            board,
            piece.color,
        ),
        PieceType::PAWN => pawn_legal_moves(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            &board,
            piece.color,
        ),
        PieceType::QUEEN => queen_legal_moves(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            board,
            piece.color,
        ),
        PieceType::ROOK => rook_legal_moves(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            board,
            piece.color,
        ),
    };
    //return filter_illegal_moves(&board, moves, piece.color, x, y);
    return uniq_moves(moves);
}

pub fn move_piece(piece_move: Move, x: i32, y: i32, board: &mut Board) -> Result<(), &'static str> {
    if x > 7 || y > 7 || x < 0 || x < 0 {
        return Err("Invalid move variable");
    }
    let piece = board.pieces[x as usize][y as usize];
    let legal_moves = get_legal_moves(*board, x, y, piece.color);

    // Check if it is actually is a legal move
    if !legal_moves.contains(&piece_move) {
        return Err("Illegal move");
    }

    board.pieces[piece_move.1 as usize][piece_move.0 as usize] = piece;
    board.pieces[y as usize][x as usize] = Piece {
        color: Color::EMPTY,
        piece_type: PieceType::EMPTY,
        has_moved: false,
    };
    return Ok(());
}

pub fn bishop_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();
    // Go row by row and check where bishop can go, if no legal moves in the next row then break out of loops

    // 1. go right up, loop over all rows i.e 8 rows
    // (2, 0) => (1, 1)
    // i = 1 => 2 + (0 -1) = 1, i = 2 => 2 + (0 - 2) = 0,
    for i in (y + 1)..7 {
        let col: i32 = x + (y - i);
        if col > 7 || col < 0 {
            break;
        }
        // check if anything is to the right up
        let piece = board.pieces[i as usize][col as usize];
        if col >= 0 && col <= 7 && piece.color == Color::EMPTY {
            valid_moves.push(Move(col, i));
        } else if piece.color != color {
            valid_moves.push(Move(col, i));
            break;
        } else {
            break;
        }
    }
    // 2. go right down
    for i in (0..y).rev() {
        let col = x + (y - i);
        if col > 7 || col < 0 {
            break;
        }
        // check if anything is to the right up
        let piece = board.pieces[i as usize][col as usize];
        if col <= 7 && col >= 0 && piece.color == Color::EMPTY {
            valid_moves.push(Move(col, i));
        } else if piece.color != color {
            valid_moves.push(Move(col, i));
            break;
        } else {
            break;
        }
    }

    // 3. go left up
    for i in (y + 1)..7 {
        let col: i32 = x + (i - y);
        if col > 7 || col < 0 {
            break;
        }

        // check if anything is to the left up
        let piece = board.pieces[i as usize][col as usize];
        if col >= 0 && col <= 7 && piece.color == Color::EMPTY {
            valid_moves.push(Move(col, i));
            // add valid move
        } else if piece.color != color {
            valid_moves.push(Move(col, i));
            break;
        } else {
            break;
        }
    }

    // 4. go left down
    for i in (0..y).rev() {
        // problem when y = 7
        let col: i32 = x + (i - y);
        if col > 7 || col < 0 {
            break;
        }
        // check if anything is to the right up
        let piece = board.pieces[i as usize][col as usize];
        if col >= 0 && col <= 7 && piece.color == Color::EMPTY {
            valid_moves.push(Move(col, i));
        } else if piece.color != color {
            valid_moves.push(Move(col, i));
            break;
        } else {
            break;
        }
    }
    return valid_moves;
}

pub fn pawn_legal_moves(x: i32, y: i32, board: &Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();
    let piece = board.pieces[y as usize][x as usize];

    if y < 7
        && board.pieces[(y + 1) as usize][x as usize].color != color
        && board.pieces[(y + 1) as usize][x as usize].piece_type == PieceType::EMPTY
    {
        valid_moves.push(Move(x, y + 1));
        if !piece.has_moved
            && y < 6
            && board.pieces[(y + 2) as usize][x as usize].color != color
            && board.pieces[(y + 2) as usize][x as usize].piece_type == PieceType::EMPTY
        {
            valid_moves.push(Move(x, y + 2));
        }
    }
    if x > 0
        && y < 7
        && board.pieces[(y + 1) as usize][(x - 1) as usize].color != color
        && board.pieces[(y + 1) as usize][(x - 1) as usize].piece_type != PieceType::EMPTY
    {
        valid_moves.push(Move(x - 1, y + 1));
    }
    if x < 7
        && y < 7
        && board.pieces[(y + 1) as usize][(x + 1) as usize].color != color
        && board.pieces[(y + 1) as usize][(x + 1) as usize].piece_type != PieceType::EMPTY
    {
        valid_moves.push(Move(x + 1, y + 1));
    }

    return valid_moves;
}

pub fn rook_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();

    // 1. check straight line, y axis

    // 1.1 check up dir
    for i in (y + 1)..7 {
        // check if anything here, if not add move and continue loop, otherwise break out

        if board.pieces[i as usize][x as usize].color == Color::EMPTY {
            valid_moves.push(Move(x, i));
        } else if board.pieces[i as usize][x as usize].color != color {
            valid_moves.push(Move(x, i));
            break;
        } else {
            break;
        }
    }
    // 1.2 check down dir
    for i in (0..y).rev() {
        // check if anything here, if not add move and continue loop, otherwise break out
        if board.pieces[i as usize][x as usize].color == Color::EMPTY {
            valid_moves.push(Move(x, i));
        } else if board.pieces[i as usize][x as usize].color != color {
            valid_moves.push(Move(x, i));
            break;
        } else {
            break;
        }
    }

    // 2. check straight line in x-axis

    //2.1 check to right dir
    for i in (x + 1)..7 {
        if board.pieces[y as usize][i as usize].color == Color::EMPTY {
            valid_moves.push(Move(i, y));
        } else if board.pieces[y as usize][i as usize].color != color {
            valid_moves.push(Move(i, y));
            break;
        } else {
            break;
        }
    }

    // 2.2 check left dir
    for i in (0..x).rev() {
        if board.pieces[y as usize][i as usize].color == Color::EMPTY {
            valid_moves.push(Move(i, y));
        } else if board.pieces[y as usize][i as usize].color != color {
            valid_moves.push(Move(i, y));
            break;
        } else {
            break;
        }
    }

    return valid_moves;
}

pub fn knight_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();

    let a = vec![-2, 2];
    let b = vec![-1, 1];

    // The knight can either move 2 steps horizontally or 2 steps vertically
    // Case 1: two steps horizontally
    // Valid transforms: (-2, 1) (-2, -1), (2, -1), (2, 1)
    // Basically, cartesian product: {2, -2} x {1, -1}
    // Case 2: two steps vertically
    // Valid moves (-1, -2), (-1, 2) and so on
    // Cartesian product: {1, -1} x {2, -2}
    // Valid moves: {1, -1} x {2, -2} + {-2, 2} x {-1, 1}
    let two_steps_vertical = cartesian_product(&a, &b);
    let two_steps_horizontal = cartesian_product(&b, &a);
    let possible_moves = [two_steps_horizontal, two_steps_vertical].concat();
    for possible_move in possible_moves {
        if possible_move.0 + x >= 0
            && (possible_move.0 + x) <= 7
            && (possible_move.1 + y) >= 0
            && (possible_move.1 + y) <= 7
            && board.pieces[(y + possible_move.1) as usize][(x + possible_move.0) as usize].color
                != color
        {
            valid_moves.push(Move(x + possible_move.0, y + possible_move.1));
        }
    }

    return valid_moves;
}

// TODO: Fix naming, little confusing with valid and possible moves
pub fn king_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();

    // Pseudo legal moves
    // up, down, right, left
    if (y + 1) <= 7 && board.pieces[(y + 1) as usize][x as usize].color != color {
        valid_moves.push(Move(x, y + 1));
    }
    if (y - 1) >= 0 && board.pieces[(y - 1) as usize][x as usize].color != color {
        valid_moves.push(Move(x, y - 1));
    }
    if (x + 1) <= 7 && board.pieces[(x + 1) as usize][x as usize].color != color {
        valid_moves.push(Move(x + 1, y));
    }
    if (x - 1) >= 0 && board.pieces[(x - 1) as usize][x as usize].color != color {
        valid_moves.push(Move(x - 1, y));
    }

    // moves can be represented as {-1, 1} x {-1, 1}
    let a = vec![-1, 1];
    for possible_move in cartesian_product(&a, &a) {
        if possible_move.0 + x >= 0
            && (possible_move.0 + x) <= 7
            && (possible_move.1 + y) >= 0
            && (possible_move.1 + y) <= 7
            && board.pieces[(y + possible_move.1) as usize][(x + possible_move.0) as usize].color
                != color
        {
            valid_moves.push(Move(x + possible_move.0, y + possible_move.1));
        }
    }
    return valid_moves;
}

pub fn queen_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    // https://www.chess.com/terms/chess-queen
    // The valid moves for the queen is basically the union of the valid moves for the bishop and rook
    let diagonal_moves = bishop_legal_moves(x, y, board, color);
    let horizontal_vertical_moves = rook_legal_moves(x, y, board, color);
    return [diagonal_moves, horizontal_vertical_moves].concat();
}
