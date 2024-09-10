use crate::utils::sets::cartesian_product;

use super::board::Board;

#[derive(Clone, PartialEq, Copy)]
pub enum PieceType {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    KING,
    QUEEN,
    EMPTY,
}
// x, y
pub struct Move(i32, i32);

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    WHITE,
    BLACK,
    EMPTY,
}
#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool,
}

pub fn bishop_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();
    // Go row by row and check where bishop can go, if no legal moves in the next row then break out of loops

    // 1. go right up, loop over all rows i.e 8 rows
    for i in (y + 1)..7 {
        let col = x + 1 + (i - y);
        // check if anything is to the right up
        if col <= 7 && board.pieces[i as usize][col as usize].color != color {
            valid_moves.push(Move(col, i));
        } else {
            break;
        }
    }
    // 2. go right down
    for i in (0..(y - 1)).rev() {
        let col = x + 1 + (y - i);
        // check if anything is to the right up
        if col <= 7 && board.pieces[i as usize][col as usize].color != color {
            valid_moves.push(Move(col, i));
        } else {
            break;
        }
    }

    // 3. go left up
    for i in (y + 1)..7 {
        let col = x - 1 + (i - y);

        // check if anything is to the left up
        if col >= 0 && board.pieces[i as usize][col as usize].color != color {
            valid_moves.push(Move(col, i));
            // add valid move
        } else {
            break;
        }
    }

    // 4. go left down
    for i in (0..(y - 1)).rev() {
        let col = x - 1 + (y - i);
        // check if anything is to the right up
        if col >= 0 && board.pieces[i as usize][col as usize].color != color {
            valid_moves.push(Move(col, i));
        } else {
            break;
        }
    }
    return valid_moves;
}

pub fn pawn_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();
    let piece = board.pieces[y as usize][x as usize];

    if board.pieces[(y + 1) as usize][x as usize].color != color
        && board.pieces[(y + 1) as usize][x as usize].piece_type == PieceType::EMPTY
    {
        valid_moves.push(Move(x, y + 1));
        if !piece.has_moved
            && board.pieces[(y + 2) as usize][x as usize].color != color
            && board.pieces[(y + 2) as usize][x as usize].piece_type == PieceType::EMPTY
        {
            valid_moves.push(Move(x, y + 2));
        }
    }
    if x != 0
        && board.pieces[(y + 1) as usize][(x - 1) as usize].color != color
        && board.pieces[(y + 1) as usize][(x - 1) as usize].piece_type != PieceType::EMPTY
    {
        valid_moves.push(Move(x - 1, y + 1));
    }
    if board.pieces[(y + 1) as usize][(x + 1) as usize].color != color
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

        if board.pieces[i as usize][x as usize].color != color {
            valid_moves.push(Move(x, i));
        } else {
            break;
        }
    }
    // 1.2 check down dir
    for i in (0..(y - 1)).rev() {
        // check if anything here, if not add move and continue loop, otherwise break out

        if board.pieces[i as usize][x as usize].color != color {
            valid_moves.push(Move(x, i));
        } else {
            break;
        }
    }

    // 2. check straight line in x-axis

    //2.1 check to right dir
    for i in (x + 1)..7 {
        if board.pieces[y as usize][i as usize].color != color {
            valid_moves.push(Move(i, y))
        } else {
            break;
        }
    }

    // 2.2 check left dir
    for i in (0..(x - 1)).rev() {
        if board.pieces[y as usize][i as usize].color != color {
            valid_moves.push(Move(i, y));
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

    let two_steps_vertical = cartesian_product(&a, &b);
    let two_steps_horizontal = cartesian_product(&b, &a);
    let possible_moves = [two_steps_horizontal, two_steps_vertical].concat();
    for possible_move in possible_moves {
        if possible_move.0 + x >= 0
            && possible_move.0 + x <= 7
            && possible_move.1 >= 0
            && possible_move.1 <= 7
            && board.pieces[(y + possible_move.1) as usize][(x + possible_move.0) as usize].color
                != color
        {
            valid_moves.push(Move(x + possible_move.0, y + possible_move.1));
        }
    }
    // Case 1: two steps horizontally
    // Valid transforms: (-2, 1) (-2, -1), (2, -1), (2, 1)
    // Basically, cartesian product: {2, -2} x {1, -1}
    // Case 2: two steps vertically
    // Cartesian product: {1, -1} x {2, -2}

    return valid_moves;
}

pub fn king_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();

    return valid_moves;
}

pub fn queen_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();

    return valid_moves;
}

// Make compiler happy
pub fn empty_legal_moves(x: i32, y: i32, board: Board, color: Color) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();

    return valid_moves;
}
