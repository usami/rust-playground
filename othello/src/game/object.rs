pub struct Player {
    pub strategy: super::Strategy,
    pub piece: Piece,
}

impl Player {
    pub fn select_move(&self, b: &Board) -> Option<super::Move> {
        // possible moves
        // if not possibl
        return Some(super::Move(super::Column::A, super::Row::One, self.piece.clone()))
            // self.strategy.pick_move(b, possible_moves)
    }
}

pub enum Piece {
    Black,
    White,
}

impl Piece {
    fn clone(&self) -> Piece {
        match self {
            Piece::Black => Piece::Black,
            Piece::White => Piece::White,
        }
    }
}

pub struct Board {
    repl: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            repl: [
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, Some(Piece::White), Some(Piece::Black), None, None, None,
                None, None, None, Some(Piece::Black), Some(Piece::White), None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
            ],
        }
    }

    pub fn print(&self) {
        let c_header = " a b c d e f g h ";
        let r_sep = "+-+-+-+-+-+-+-+-+";
        let sep = '|';

        println!("  {}", c_header);
        println!("  {}", r_sep);

        for i in 0..8 {
            let mut row = String::from(format!("{}{}", i+1, sep));
            for j in 0..8 {
                match self.repl[8 * i + j] {
                    None => row.push(' '),
                    Some(Piece::Black) => row.push('○'),
                    Some(Piece::White) => row.push('●'),
                }
                row.push(sep);
            }
            println!(" {}", row);
            println!("  {}", r_sep);
        }

        println!("  {}\n", c_header);
    }

    pub fn clone(&self) -> Board {
        let mut board = Board::new();
        for i in 0..64 {
            board.repl[i] = match self.repl[i] {
                None => None,
                Some(Piece::Black) => Some(Piece::Black),
                Some(Piece::White) => Some(Piece::White),
            };
        }
        return board
    }

    pub fn put(&mut self, i: usize, p: &Piece) -> Board {
        let mut board = self.clone();
        board.repl[i] = Some(p.clone());
        return board
    }
}
