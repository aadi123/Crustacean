use std::str::FromStr;

use chess::{Board, ChessMove, Error, MoveGen, Piece, Square};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_next_move(fen: &str) -> String {
    let res: Result<Board, Error> = Board::from_str(fen);
    let board = match res {
        Ok(b) => b,
        Err(_) => Board::default(),
    };
    let movegen = MoveGen::new_legal(&board);

    for chess_move in movegen {
        // This move does not capture anything
        return chess_move.to_string();
    }
    return ChessMove::new(Square::E4, Square::E4, Some(Piece::Queen)).to_string();
}
