use crate::utils::matrix::index_to_col_row;

use super::pieces::{get_legal_moves, Color, Move, Piece, PieceType};

#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [[Piece; 8]; 8],
}

fn fill_pawns(color: Color) -> [Piece; 8] {
    let pawns = [Piece {
        color,
        piece_type: PieceType::PAWN,
        has_moved: false,
    }; 8];

    return pawns;
}

// Define check as if the king stands on a position that can be reached through a legal move then we are in check
// Can use this to remove moves that puts team in check
// Check mate is when a team is in check and there no legal moves left
fn in_check(board: Board, color: Color) -> bool {
    // generate valid moves for all the pieces in the other team
    let opposing_color = Color::EMPTY;
    let opposing_pieces: Vec<(&Piece, usize)> = board
        .pieces
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, piece)| (piece, i))
        .filter(|(piece, index)| piece.color == opposing_color)
        .collect();

    let possible_moves: Vec<Move> = opposing_pieces
        .iter()
        .map(|(piece, index)| {
            let (row, col) = index_to_col_row(*index);
            return get_legal_moves(board, col, row, opposing_color);
        })
        .flatten()
        .collect();

    let mut check = false;
    // Go through all possible moves and check if any of the moves can get to the king
    for possible_move in possible_moves {
        if board.pieces[possible_move.1 as usize][possible_move.0 as usize].color == color
            && board.pieces[possible_move.1 as usize][possible_move.0 as usize].piece_type
                == PieceType::KING
        {
            // in check and want to return something
            check = true;
        }
    }

    return check;
}

// Illegal moves here means a move, where after the move the team is in check
fn filter_illegal_moves(board: Board) {
    let board_copy = board.clone();
}

pub fn board_from_fen() {}

// Set up the board with the right number of pieces and stuff
pub fn init_board() -> Board {
    let black_pieces = [
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::ROOK,
            has_moved: false,
        },
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::KNIGHT,
            has_moved: false,
        },
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::BISHOP,
            has_moved: false,
        },
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::QUEEN,
            has_moved: false,
        },
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::KING,
            has_moved: false,
        },
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::KNIGHT,
            has_moved: false,
        },
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::BISHOP,
            has_moved: false,
        },
        Piece {
            color: Color::BLACK,
            piece_type: PieceType::ROOK,
            has_moved: false,
        },
    ];
    let black_pawns = fill_pawns(Color::BLACK);

    let white_pieces = [
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::ROOK,
            has_moved: false,
        },
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::KNIGHT,
            has_moved: false,
        },
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::BISHOP,
            has_moved: false,
        },
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::QUEEN,
            has_moved: false,
        },
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::KING,
            has_moved: false,
        },
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::KNIGHT,
            has_moved: false,
        },
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::BISHOP,
            has_moved: false,
        },
        Piece {
            color: Color::WHITE,
            piece_type: PieceType::ROOK,
            has_moved: false,
        },
    ];
    let white_pawns = fill_pawns(Color::WHITE);

    let mut pieces = [[Piece {
        color: Color::EMPTY,
        piece_type: PieceType::EMPTY,
        has_moved: false,
    }; 8]; 8];
    pieces[0] = white_pieces;
    pieces[1] = white_pawns;
    // Empty space in between
    pieces[6] = black_pawns;
    pieces[7] = black_pieces;

    let board = Board { pieces: pieces };
    return board;
}

#[cfg(test)]
mod tests {

    use std::fmt::Error;

    use crate::board::pieces::{get_legal_moves, move_piece, Move};

    use super::*;

    #[test]
    fn init_success() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");

        let white_pawns: Vec<&Piece> = board
            .pieces
            .iter()
            .flatten()
            .filter(|p| p.piece_type == PieceType::PAWN && p.color == Color::WHITE)
            .collect();
        assert_eq!(white_pawns.len(), 8);
    }

    #[test]
    fn bishop_no_moves_game_start() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");
        let legal_moves = get_legal_moves(board, 2, 0, Color::WHITE);

        assert_eq!(legal_moves.len(), 0);
    }
    #[test]
    fn rook_no_moves_game_start() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");
        let legal_moves = get_legal_moves(board, 0, 0, Color::WHITE);

        assert_eq!(legal_moves.len(), 0);
    }

    #[test]
    fn white_pawn_can_move_from_start() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");
        let legal_moves = get_legal_moves(board, 0, 1, Color::WHITE);

        assert_eq!(legal_moves.len(), 2);
    }

    #[test]
    fn black_pawn_can_move_from_start() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");
        let legal_moves = get_legal_moves(board, 0, 6, Color::BLACK);

        assert_eq!(legal_moves.len(), 2);
    }

    #[test]
    fn knight_can_move_from_start() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");
        let legal_moves = get_legal_moves(board, 1, 0, Color::WHITE);

        assert_eq!(legal_moves.len(), 2);
    }

    #[test]
    fn black_knight_can_move_from_start() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");
        let legal_moves = get_legal_moves(board, 1, 7, Color::BLACK);

        assert_eq!(legal_moves.len(), 2);
    }

    #[test]
    fn reject_invalid_pawn_move() {
        let mut board = init_board();

        let invalid_move = Move(7, 7);

        let result = move_piece(invalid_move, 1, 1, &mut board);
        assert!(result.is_err());
    }
}
