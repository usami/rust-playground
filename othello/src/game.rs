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

pub struct Move(Column, Row, Piece);

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
