use crate::board::{
    board::Board,
    pieces::{Color, Move, PieceType},
};

pub struct Game {
    pub board: Board,
    pub turn: Color,
    // (right, left)
    pub can_castle_white: (bool, bool),
    pub can_castle_black: (bool, bool),
    pub white_in_check: bool,
    pub black_in_check: bool,
    pub white_captures: Vec<PieceType>,
    pub black_captures: Vec<PieceType>,
    pub white_pawn_promotion: Option<(i32, i32)>,
    pub black_pawn_promotion: Option<(i32, i32)>,
    pub white_moves: Vec<(Move, PieceType)>,
    pub black_moves: Vec<(Move, PieceType)>,
}

impl Game {
    pub fn new(fen: Option<String>) -> Game {
        Game {
            board: Board::init_board(fen),
            turn: Color::WHITE,
            can_castle_white: (false, false),
            can_castle_black: (false, false),
            white_in_check: false,
            black_in_check: false,
            white_captures: Vec::new(),
            black_captures: Vec::new(),
            white_moves: Vec::new(),
            black_moves: Vec::new(),
            white_pawn_promotion: None,
            black_pawn_promotion: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board;

    use super::*;

    #[test]
    fn reject_invalid_pawn_move() {
        let mut game = Game::new(None);

        let invalid_move = Move(7, 7);

        let result = board::pieces::move_piece(invalid_move, 1, 1, &mut game);
        assert!(result.is_err());
    }

    #[test]
    fn pawn_move_updates_game_state() {
        let mut game = Game::new(None);

        // Move white pawn from e2 to e3
        let pawn_move = Move(4, 4);
        let result = board::pieces::move_piece(pawn_move, 4, 6, &mut game);

        assert!(result.is_ok());

        println!("{:?}", game.board.pieces[6][4].piece_type);

        // Check that the game state has been updated
        assert_eq!(game.turn, Color::BLACK);
        assert_eq!(game.white_moves.len(), 1);
        assert_eq!(game.white_moves[0], (pawn_move, PieceType::PAWN));
        assert!(game.black_moves.is_empty());
        assert!(game.white_captures.is_empty());
        assert!(game.black_captures.is_empty());

        // Verify board state
        assert_eq!(game.board.pieces[4][4].piece_type, PieceType::PAWN);
        assert_eq!(game.board.pieces[4][4].color, Color::WHITE);
        assert_eq!(game.board.pieces[5][4].piece_type, PieceType::EMPTY);
        assert_eq!(game.board.pieces[5][4].color, Color::EMPTY);
    }
}
