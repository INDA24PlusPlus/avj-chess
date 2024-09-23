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
    pub white_en_passant: bool,
    pub black_en_passant: bool,
    pub check_mate_white: bool,
    pub check_mate_black: bool,
    pub white_repetitions: i32,
    pub black_repetitions: i32,
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
            white_en_passant: false,
            black_en_passant: false,
            check_mate_white: false,
            check_mate_black: false,
            white_repetitions: 0,
            black_repetitions: 0,
        }
    }

    pub fn game_over(&self) -> Option<Color> {
        if self.check_mate_white {
            return Some(Color::BLACK);
        }
        if self.check_mate_black {
            return Some(Color::WHITE);
        }
        if self.white_repetitions >= 3 {
            return Some(Color::BLACK);
        }
        if self.black_repetitions >= 3 {
            return Some(Color::WHITE);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{
        self,
        pieces::{make_castle_move, promote_pawn},
    };

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

    #[test]
    fn white_pawn_promotion_updates_game_state() {
        let mut game = Game::new(Some(String::from(
            "r3kbnr/1PQ1pppp/1pnp4/p3P3/2B5/7N/P1PP1PPP/RNB1K2R",
        )));

        let promotion_move = Move(1, 0);
        let result = board::pieces::move_piece(promotion_move, 1, 1, &mut game);

        assert!(result.is_ok());
        assert_eq!(game.white_pawn_promotion, Some((1, 0)));
        assert_eq!(game.board.pieces[0][1].piece_type, PieceType::PAWN);
    }

    #[test]
    fn make_white_pawn_promotion() {
        let mut game = Game::new(Some(String::from(
            "r3kbnr/1PQ1pppp/1pnp4/p3P3/2B5/7N/P1PP1PPP/RNB1K2R",
        )));

        let promotion_move = Move(1, 0);
        board::pieces::move_piece(promotion_move, 1, 1, &mut game).ok();

        assert!(promote_pawn(&mut game, PieceType::QUEEN, Color::WHITE).is_ok());
        assert_eq!(game.board.pieces[0][1].piece_type, PieceType::QUEEN);
        assert_eq!(game.white_pawn_promotion, None);
        // test that black pawn promotion fails
        assert!(promote_pawn(&mut game, PieceType::QUEEN, Color::BLACK).is_err())
    }

    #[test]
    fn white_right_perform_castling() {
        let mut game = Game::new(Some(String::from(
            "rnbqkbnr/2p1pppp/1p1p4/p3P2Q/8/7N/PPPP1PPP/RNB1KB1R",
        )));

        // make space for castling
        board::pieces::move_piece(Move(4, 6), 5, 7, &mut game).ok();

        let result = make_castle_move(&mut game, Color::WHITE, 1);
        assert!(result.is_ok());
        assert_eq!(game.can_castle_white, (true, false));

        assert_eq!(game.board.pieces[7][6].piece_type, PieceType::KING);
        assert_eq!(game.board.pieces[7][5].piece_type, PieceType::ROOK);
    }

    #[test]
    fn test_white_pawn_en_passant() {
        let mut game = Game::new(Some(String::from(
            "rnbqkbnr/pppppppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR",
        )));
        game.turn = Color::BLACK;

        let en_passant_move = Move(3, 3);
        let result = board::pieces::move_piece(en_passant_move, 3, 1, &mut game);

        println!("{:?}", game.black_moves);
        assert!(result.is_ok());
        assert_eq!(game.board.pieces[3][3].piece_type, PieceType::PAWN);
        assert_eq!(game.board.pieces[3][3].color, Color::BLACK);
        assert_eq!(game.white_en_passant, true);
    }

    #[test]
    fn test_white_make_en_passant() {
        let mut game = Game::new(Some(String::from(
            "rnbqkbnr/pppppppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR",
        )));
        game.turn = Color::BLACK;

        let en_passant_move = Move(3, 3);
        let black_move_result = board::pieces::move_piece(en_passant_move, 3, 1, &mut game);

        assert!(black_move_result.is_ok());
        assert_eq!(game.white_en_passant, true);

        board::pieces::en_passant_move(&mut game, Color::WHITE, 4, 3);

        assert_eq!(game.board.pieces[3][3].piece_type, PieceType::EMPTY);
        assert_eq!(game.board.pieces[3][3].color, Color::EMPTY);
        assert_eq!(game.white_en_passant, false);
        assert_eq!(game.white_captures.len(), 1);
        assert!(game.white_captures.contains(&PieceType::PAWN));
        assert_eq!(game.board.pieces[2][3].piece_type, PieceType::PAWN);
        assert_eq!(game.board.pieces[2][3].color, Color::WHITE);
    }

    #[test]
    fn test_white_check_mate() {
        let mut game = Game::new(Some(String::from(
            "rnbqkbnr/pppp1ppp/4p3/8/6P1/5P2/PPPPP2P/RNBQKBNR",
        )));
        game.turn = Color::BLACK;

        // move black queen to mate position
        let queen_move = Move(7, 4);
        let result = board::pieces::move_piece(queen_move, 3, 0, &mut game);

        assert!(result.is_ok());
        assert_eq!(game.check_mate_white, true);
        assert_eq!(game.check_mate_black, false)
    }

    #[test]
    fn test_black_check_mate() {
        let mut game = Game::new(Some(String::from(
            "rnbqkbnr/ppppp2p/5p2/6p1/8/4P3/PPPP1PPP/RNBQKBNR",
        )));

        let queen_move = Move(7, 3);
        let result = board::pieces::move_piece(queen_move, 3, 7, &mut game);

        println!("{:?}", result.err());
        assert!(result.is_ok());
        assert_eq!(game.check_mate_white, false);
        assert_eq!(game.check_mate_black, true)
    }
}
