pub struct Player {
    pub strategy: super::Strategy,
    pub piece: Piece,
}

impl Player {
    pub fn select_move(&self, b: &Board) -> Option<super::Move> {
        let mut possible_moves = b.possible_moves(&self.piece);

        self.strategy.pick_move(b, &mut possible_moves)
        // if possible_moves.is_empty() {
        //     None
        // } else {
        //     Some(self.strategy.pick_move(b, &mut possible_moves))
        // }
    }
}

#[derive(Debug)]
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

    fn same(&self, p: &Piece) -> bool {
        match (self, p) {
            (Piece::Black, Piece::Black) => true,
            (Piece::White, Piece::White) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn same_row(i: i8, j: i8) -> bool {
        i - i % 8 == j - j % 8
    }

    pub fn forward(i: i8, d: &Direction) -> Option<usize> {
        match d {
            Direction::Up => {
                let j = i - 8;
                if j < 0 {
                    None
                } else {
                    Some(j as usize)
                }
            },
            Direction::UpRight => {
                let j = i - 7;
                if j < 0 || Direction::same_row(i, j) {
                    None
                } else {
                    Some(j as usize)
                }
            },
            Direction::Right => {
                let j = i + 1;
                if j > 63 || Direction::same_row(i, j) {
                    None
                } else {
                    Some(j as usize)
                }
            },
            Direction::DownRight => {
                let j = i + 9;
                if j > 63 || !Direction::same_row(i, i + 1) {
                    None
                } else {
                    Some(j as usize)
                }
            },
            Direction::Down => {
                let j = i + 8;
                if j > 63 {
                    None
                } else {
                    Some(j as usize)
                }
            },
            Direction::DownLeft => {
                let j = i + 7;
                if j > 63 || Direction::same_row(i, j) {
                    None
                } else {
                    Some(j as usize)
                }
            },
            Direction::Left => {
                let j = i - 1;
                if j < 0 || Direction::same_row(i, j) {
                    None
                } else {
                    Some(j as usize)
                }
            },
            Direction::UpLeft => {
                let j = i - 9;
                if j < 0 || !Direction::same_row(i, i - 1) {
                    None
                } else {
                    Some(j as usize)
                }
            },
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

        for d in [Direction::Up, Direction::UpRight, Direction::Right, Direction::DownRight, Direction::Down, Direction::DownLeft, Direction::Left, Direction::UpLeft].iter() {
            if board.check_dir(i, d, p) {
                board.flips(i, d, p);
            }
        }
        return board
    }

    fn flips(&mut self, i: usize, d: &Direction, p: &Piece) {
        let mut i = i;

        loop {
            if let Some(j) = Direction::forward(i as i8, d) {
                i = j;
                match &self.repl[i] {
                    None => break,
                    Some(x) => {
                        if !x.same(p) {
                            self.repl[i] = Some(p.clone());
                        } else {
                            break;
                        }
                    },
                }
            } else {
                break;
            }
        }
    }

    pub fn possible_moves(&self, p: &Piece) -> Vec<super::Move> {
        let mut moves = Vec::new();

        for i in 0..64 {
            if let None = self.repl[i] {
                for d in [Direction::Up, Direction::UpRight, Direction::Right, Direction::DownRight, Direction::Down, Direction::DownLeft, Direction::Left, Direction::UpLeft].iter() {
                    if self.check_dir(i, d, p) {
                        let m = super::Move::new(i, p);
                        moves.push(m);
                        break;
                    }
                }
            }
        }

        return moves
    }

    fn check_dir(&self, i: usize, d: &Direction, p: &Piece) -> bool {
        if let Some(j) = Direction::forward(i as i8, d) {
            // next should be opposite color
            match &self.repl[j] {
                None => false,
                Some(x) => {
                    if !x.same(p) {
                        let mut i = j;
                        // look for same color
                        loop {
                            if let Some(j) = Direction::forward(i as i8, d) {
                                i = j;
                                match &self.repl[j] {
                                    None => return false,
                                    Some(x) => {
                                        if x.same(p) {
                                            return true
                                        }
                                    },
                                }
                            } else {
                                return false
                            }
                        }
                    } else {
                        false
                    }
                },
            }
        } else {
            false
        }
    }
}
