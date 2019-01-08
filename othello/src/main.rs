mod game;
use crate::game::{Board, Strategy};

fn main() {
    let board = Board::new(Strategy::Monkey, Strategy::Monkey);

    board.print();
}
