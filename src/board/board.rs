use std::any::Any;

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
pub fn in_check(board: Board, color: Color) -> bool {
    // generate valid moves for all the pieces in the other team
    let mut opposing_color = Color::EMPTY;
    if color == Color::WHITE {
        opposing_color = Color::BLACK;
    } else if color == Color::BLACK {
        opposing_color = Color::WHITE;
    }
    let opposing_pieces: Vec<(&Piece, usize)> = board
        .pieces
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, piece)| (piece, i))
        .filter(|(piece, _index)| piece.color == opposing_color)
        .collect();

    let possible_moves: Vec<Move> = opposing_pieces
        .iter()
        .map(|(_piece, index)| {
            let (row, col) = index_to_col_row(*index).unwrap();

            let pseudo_moves = get_legal_moves(board, col, row, opposing_color);
            if pseudo_moves.contains(&Move(4, 0)) && color == Color::BLACK {
                println!(
                    "Row: {:?}, Col: {:?}, Piece: {:?}, Index: {:?}, Color: {:?}",
                    row, col, _piece.piece_type, index, _piece.color
                );
                println!("Found a king in check, moves {:?}", pseudo_moves);
            }
            return pseudo_moves;
        })
        .flatten()
        .collect();

    let mut check = false;
    // Go through all possible moves and check if any of the moves can get to the king
    for (index, possible_move) in possible_moves.iter().enumerate() {
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

    use std::collections::HashMap;

    use crate::board::{
        parser::{self, parse_fen_string},
        pieces::{get_legal_moves, move_piece, Move},
    };

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
    fn bishop_can_move() {
        let fen = String::from("rnbqkbnr/1pp1pppp/3p4/p6Q/4P3/7N/PPPP1PPP/RNB1KB1R");

        let board = parser::parse_fen_string(fen);

        let white_bishop_legal_moves = get_legal_moves(board, 5, 7, Color::WHITE);
        // White legal moves should be: (4, 6) (3, 5) (2, 4) (1, 3) (0, 2)

        let black_bishop_legal_moves = get_legal_moves(board, 2, 0, Color::BLACK);

        assert_eq!(white_bishop_legal_moves.len(), 5);
        assert_eq!(black_bishop_legal_moves.len(), 5);
    }
    #[test]
    fn test_situation() {
        let fen = String::from("rnbqkbnr/2p1pppp/1p1p4/p6Q/4P3/7N/PPPP1PPP/RNB1KB1R");

        let board = parse_fen_string(fen);
        let white_knight_legal_moves = get_legal_moves(board, 7, 5, Color::BLACK);

        let black_bishop_legal_moves = get_legal_moves(board, 2, 0, Color::BLACK);

        // right: (1,1) (0, 2)

        let white_queen_legal_moves = get_legal_moves(board, 7, 3, Color::WHITE);
        let white_rook_legal_moves = get_legal_moves(board, 7, 7, Color::WHITE);
        let black_rook_legal_moves = get_legal_moves(board, 0, 0, Color::BLACK);
        let white_bishop_legal_moves = get_legal_moves(board, 5, 7, Color::WHITE);

        assert_eq!(white_rook_legal_moves.len(), 1);
        assert!(white_rook_legal_moves.contains(&Move(6, 7)));
        assert_eq!(black_rook_legal_moves.len(), 2);
        assert_eq!(black_bishop_legal_moves.len(), 7);
        assert_eq!(white_bishop_legal_moves.len(), 5);
        // right: (1,1) (0, 2)

        assert_eq!(white_knight_legal_moves.len(), 3);

        let expected_queen_moves: Vec<Move> = vec![
            Move(6, 3),
            Move(5, 3),
            Move(4, 3),
            Move(3, 3),
            Move(2, 3),
            Move(1, 3),
            Move(0, 3),
            Move(7, 2),
            Move(6, 2),
            Move(7, 4),
            Move(6, 4),
            Move(5, 5),
            Move(4, 6),
            Move(7, 1),
            Move(5, 1),
        ];
        println!("Checking white queen moves:{:?} ", white_queen_legal_moves);
        for piece_move in &expected_queen_moves {
            assert!(white_queen_legal_moves.contains(piece_move));
        }
        assert_eq!(white_queen_legal_moves.len(), expected_queen_moves.len());
        assert!(white_queen_legal_moves.contains(&Move(4, 0)));

        // test that none of the colors are in check
        println!("Looking at white");
        assert_eq!(in_check(board, Color::WHITE), false);
        println!("Looking at black");
        assert_eq!(in_check(board, Color::BLACK), false);
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
    fn white_knight_can_move() {
        let fen = String::from("rnbqkbnr/2pppppp/1p6/p6Q/4P3/7N/PPPP1PPP/RNB1KB1R");

        let board = parser::parse_fen_string(fen);

        let knight_legal_moves = get_legal_moves(board, 7, 5, Color::WHITE);
        for piece_move in &knight_legal_moves {
            println!("{:?}", piece_move);
        }

        assert_eq!(knight_legal_moves.len(), 3);
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
