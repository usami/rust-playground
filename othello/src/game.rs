mod object;

use self::object::{Player, Piece, Board};

pub enum Result {
    O,
    X,
    Draw,
}

pub enum Strategy {
    Manual,
    Monkey,
}

#[derive(Debug)]
enum Row {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug)]
enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug)]
pub struct Move(Column, Row, Piece);

impl Move {
    pub fn new(i: usize, p: &Piece) -> Move {
        let rem = i % 8;

        let row = match (i - rem) / 8 {
            0 => Row::One,
            1 => Row::Two,
            2 => Row::Three,
            3 => Row::Four,
            4 => Row::Five,
            5 => Row::Six,
            6 => Row::Seven,
            _ => Row::Eight,
        };

        let column = match rem {
            0 => Column::A,
            1 => Column::B,
            2 => Column::C,
            3 => Column::D,
            4 => Column::E,
            5 => Column::F,
            6 => Column::G,
            _ => Column::H,
        };

        match p {
            Piece::Black => Move(column, row, Piece::Black),
            Piece::White => Move(column, row, Piece::White),
        }
    }
}

pub struct Game {
    pub board: Board,
    players: [Player; 2],
    current_player_index: usize,
}

impl Game {
    pub fn new(bs: Strategy, ws: Strategy) -> Game {
        Game {
            board: Board::new(),
            players: [
                Player { strategy: bs, piece: Piece::Black },
                Player { strategy: ws, piece: Piece::White },
            ],
            current_player_index: 0,
        }
    }

    pub fn print_board(&self) {
        self.board.print();
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player_index]
    }

    pub fn next_turn(&mut self) {
        self.current_player_index = self.current_player_index ^ 1
    }

    pub fn take(&mut self, m: &Move) {
        let Move(c, r, p) = m;

        let i = match c {
            Column::A => 0,
            Column::B => 1,
            Column::C => 2,
            Column::D => 3,
            Column::E => 4,
            Column::F => 5,
            Column::G => 6,
            Column::H => 7,
        };

        let j = match r {
            Row::One => 0,
            Row::Two => 1,
            Row::Three => 2,
            Row::Four => 3,
            Row::Five => 4,
            Row::Six => 5,
            Row::Seven => 6,
            Row::Eight => 7,
        };

        let index = 8 * i + j;

        self.board = self.board.put(index, p);
    }
}
