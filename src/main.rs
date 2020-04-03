
mod board;
mod iterrange;

use crate::board::{Board, Tile, Direction};

fn main() {
    let mut board: Board = Board::new(3, 1);
    
    board.set_tile(1, 0, Tile::Value(2));
    board.set_tile(2, 0, Tile::Value(2));
    println!("can move?: {}", board.can_move());
    board.print();
    board.make_move(Direction::Right);
    board.print();
    /*
    for _ in 0..1 {
        board.take_verbose_turn(Direction::Up);
        board.take_verbose_turn(Direction::Down);
        board.take_verbose_turn(Direction::Left);
        board.take_verbose_turn(Direction::Right);
    }*/
}
