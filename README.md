# chess-lib

Simple chess library

## Structs and enums

### Move type

A struct that describes the new position of a piece. See examples for more concrete use case

```rust
pub struct Move(pub i32, pub i32);
```

### PieceType enum

Describes all the available piece types in chess. Empty represents an empty spot on the board

```rust
pub enum PieceType {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    KING,
    QUEEN,
    EMPTY,
}
```

### Color enum

Enum to represent all the available colors. EMPTY is used for empty pieces

```rust
pub enum Color {
    WHITE,
    BLACK,
    EMPTY,
}
```

### Piece struct

An individual piece on the board.

```rust
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool,
}
```

### Board struct

Handles the board of the games and contains all the pieces. Stored in 8x8 matrix (2d array). An empty piece is represented as a Piece without an empty PieceType and Color.

```rust
pub struct Board {
    pub pieces: [[Piece; 8]; 8],
}
```

### Game struct

Handles the game state of the game.

```rust
pub struct Game {
    // The board of the game
    pub board: Board,
    // Whose turn it is, either White or Black
    pub turn: Color,
    // Describes if the the given sides can castle
    // First element says if castle is possible to the right
    // Second element says if castle is possible to the left
    pub can_castle_white: (bool, bool),
    pub can_castle_black: (bool, bool),
    // If the given sides are in check or not
    pub white_in_check: bool,
    pub black_in_check: bool,
    // The piece types that have been captured
    pub white_captures: Vec<PieceType>,
    pub black_captures: Vec<PieceType>,
    // If a pawn can be promoted
    // If not then it will be none
    // If it can be promoted then the value will be the position of that pawn
    pub white_pawn_promotion: Option<(i32, i32)>,
    pub black_pawn_promotion: Option<(i32, i32)>,
    // List of the moves been by a given side
    pub white_moves: Vec<(Move, PieceType)>,
    pub black_moves: Vec<(Move, PieceType)>,
    // If a given side can en passant
    pub white_en_passant: bool,
    pub black_en_passant: bool,
    // If a given side is in check mate
    pub check_mate_white: bool,
    pub check_mate_black: bool,
}
```

Game methods:

`Game::new(fen: Option<String>) -> Game` - Creates and initializes a new game with standard values. Pass in None to have completely empty board or pass in fen string to initialize board from fen string.

## Functions

`get_legal_moves(board: Board, x: i32, y: i32, color: Color) -> Vec<Move>`

Based on a position the function generates all legal moves from that position. Returns a Vec with with the allowed moves.

`move_piece(piece_move: Move, x: i32, y: i32, game: &mut Game) -> Result<(), &'static str>`

The actual function that moves the piece across the board. Takes in the move to perform and the current position of the piece and the game. Will return an error if a move that is not part of `get_legal_moves` is passed in. This function covers almost all of the possible moves that you can make, however for the special moves such as en passant, prawn promotion and castling.

`fn promote_pawn(game: &mut Game, new_piece: PieceType, color: Color,) -> Result<(), &'static str>`

The function used to promote the pawn. Takes in game, the new piece and the color of the team that wants to promote the pawn. The function will return an error if `game.[color]_pawn_promotion`is false.

`en_passant_move(game: &mut Game, color: Color, x: i32, y: i32)`

The function used to perform an en en passant move. Can only be performed if `game.[color]_en_passant`is true. Takes in the current position of the pawn to perform the move with.

`make_castle_move(game: &mut Game, color: Color, dir: i32) -> Result<(), &'static str>`

The function to perform the castling. Pass in the game, color and the direction. The direction is either 1 or -1. 1 for castling to the right and -1 for castling to the left. The dir must correspond to the values in tuple `game.can_castle_[color]`. If dir is 1 then `game.can_castle_[color].0` must be true and same for if dir is -1

## Examples

```

```
