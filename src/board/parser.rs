use std::collections::HashMap;

use super::{
    board::{self, Board},
    pieces::{Color, Piece, PieceType},
};

//fn serialize_board_fen(board: Board) -> str {}

pub fn parse_fen_string(fen_string: String) -> Board {
    let mut black_piece_map: HashMap<char, PieceType> = HashMap::new();
    black_piece_map.insert('p', PieceType::PAWN);
    black_piece_map.insert('n', PieceType::KNIGHT);
    black_piece_map.insert('b', PieceType::BISHOP);
    black_piece_map.insert('q', PieceType::QUEEN);
    black_piece_map.insert('k', PieceType::KING);
    black_piece_map.insert('r', PieceType::ROOK);

    let mut white_piece_map: HashMap<char, PieceType> = HashMap::new();
    white_piece_map.insert('P', PieceType::PAWN);
    white_piece_map.insert('N', PieceType::KNIGHT);
    white_piece_map.insert('B', PieceType::BISHOP);
    white_piece_map.insert('Q', PieceType::QUEEN);
    white_piece_map.insert('K', PieceType::KING);
    white_piece_map.insert('R', PieceType::ROOK);

    let mut fen_string_parts = fen_string.split_whitespace();
    let piece_placement = fen_string_parts.next().unwrap();

    // Parse piece placement
    let ranks = piece_placement.split('/');

    let mut board_pieces: [[Piece; 8]; 8] = [[Piece {
        color: Color::EMPTY,
        piece_type: PieceType::EMPTY,
        has_moved: false,
    }; 8]; 8];

    for (rank, pieces) in ranks.enumerate() {
        // uppercase characters = white, and lowercase characters = black
        let mut board_row: [Piece; 8] = [Piece {
            color: Color::EMPTY,
            piece_type: PieceType::EMPTY,
            has_moved: false,
        }; 8];

        let mut offset_by_empty_spots: usize = 0;
        for (file, piece) in pieces.chars().enumerate() {
            if piece.is_numeric() {
                let n_empty_spots = piece.to_digit(10).unwrap() - 1;

                for i in 0..n_empty_spots {
                    let i: usize = i as usize;
                    board_row[(file + i) as usize] = Piece {
                        color: Color::EMPTY,
                        piece_type: PieceType::EMPTY,
                        has_moved: false,
                    };
                }
                offset_by_empty_spots = offset_by_empty_spots + (n_empty_spots as usize);
            } else if black_piece_map.get(&piece).is_some() {
                let piece_type = *(black_piece_map.get(&piece).unwrap());
                board_row[file + offset_by_empty_spots] = Piece {
                    color: Color::BLACK,
                    has_moved: piece_type == PieceType::PAWN && rank != 1,
                    piece_type,
                };
            } else if white_piece_map.get(&piece).is_some() {
                let piece_type = *(white_piece_map.get(&piece).unwrap());
                board_row[file + offset_by_empty_spots] = Piece {
                    color: Color::WHITE,
                    has_moved: piece_type == PieceType::PAWN && rank != 6,
                    piece_type,
                };
            }
        }

        board_pieces[rank] = board_row;
    }
    let board = Board {
        pieces: board_pieces,
    };
    return board;
}

pub fn print_row(pieces: [Piece; 8]) {
    for (index, piece) in pieces.iter().enumerate() {
        match piece.color {
            Color::EMPTY => print!(" "),
            Color::BLACK => print!("{} Black", piece.piece_type),
            Color::WHITE => print!("{} White", piece.piece_type),
        }
        if index != pieces.len() - 1 {
            print!(" | ");
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::board::{
        parser::print_row,
        pieces::{Color, PieceType},
    };

    use super::parse_fen_string;

    #[test]
    fn board_creation_success() {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR");
        for (i, c) in "4P3".chars().enumerate() {
            if c.is_numeric() {
                let empty_spots = c.to_digit(10).unwrap();
                for j in 0..empty_spots {
                    let j: usize = j as usize;
                    println!("{}", j + i);
                }
                println!("{}", c.to_digit(10).unwrap());
            }
        }

        let board = parse_fen_string(fen);
        for i in 0..8 {
            print_row(board.pieces[i]);
        }
        assert_eq!(board.pieces.len(), 8);
        assert_ne!(board.pieces[0][1].piece_type, PieceType::EMPTY);
        assert_eq!(board.pieces[0][1].piece_type, PieceType::KNIGHT);
        assert_eq!(board.pieces[4][4].piece_type, PieceType::PAWN);
        assert_eq!(board.pieces[4][4].color, Color::WHITE);
    }
}
