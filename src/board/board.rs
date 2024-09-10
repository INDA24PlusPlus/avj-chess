use std::borrow::Borrow;

use super::pieces::{
    bishop_legal_moves, empty_legal_moves, king_legal_moves, knight_legal_moves, pawn_legal_moves,
    queen_legal_moves, rook_legal_moves, Color, Move, Piece, PieceType,
};

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

// Given a position, return the next valid positions
pub fn get_legal_moves(board: Board, x: u32, y: u32, color: Color) -> Vec<Move> {
    let piece = board.pieces[y as usize][x as usize];

    let moves = match piece.piece_type {
        PieceType::BISHOP => {
            bishop_legal_moves(x.try_into().unwrap(), y.try_into().unwrap(), board, color)
        }
        PieceType::EMPTY => {
            empty_legal_moves(x.try_into().unwrap(), y.try_into().unwrap(), board, color)
        }
        PieceType::KING => {
            king_legal_moves(x.try_into().unwrap(), y.try_into().unwrap(), board, color)
        }
        PieceType::KNIGHT => {
            knight_legal_moves(x.try_into().unwrap(), y.try_into().unwrap(), board, color)
        }
        PieceType::PAWN => {
            pawn_legal_moves(x.try_into().unwrap(), y.try_into().unwrap(), board, color)
        }
        PieceType::QUEEN => {
            queen_legal_moves(x.try_into().unwrap(), y.try_into().unwrap(), board, color)
        }
        PieceType::ROOK => {
            rook_legal_moves(x.try_into().unwrap(), y.try_into().unwrap(), board, color)
        }
    };
    return moves;
}

fn move_piece() {}

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
    fn pawn_can_move_from_start() {
        let board = init_board();
        let piece_size = board.pieces.len();
        println!("{piece_size}");
        let legal_moves = get_legal_moves(board, 0, 1, Color::WHITE);

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
}
