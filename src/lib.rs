use rand::prelude::*;
use std::str::FromStr;

use chess::{Board, Error, MoveGen};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_next_move(fen: &str) -> JsValue {
    let res: Result<Board, Error> = Board::from_str(fen);
    let board = match res {
        Ok(b) => b,
        Err(_) => Board::default(),
    };
    let movegen = MoveGen::new_legal(&board);
    let mut rng = rand::thread_rng();
    // return first random move
    let chess_move = movegen.choose(&mut rng).unwrap();
    return JsValue::from_str(&chess_move.to_string());
}
