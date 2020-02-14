
mod board;

use crate::board::{Board, Tile, Direction};

fn main() {

    let mut board: Board = Board::new_square(4);


    
    board.set_tile(0,0,Tile::Value(10));
    board.set_tile(2,0,Tile::Value(10));
    board.print();
    println!("");
    board.make_move(Direction::Right);
    board.print();

}
