pub struct Player {
    pub strategy: super::Strategy,
    pub piece: Piece,
}

impl Player {
    pub fn select_move(&self, b: &Board) -> Option<super::Move> {
        let possible_moves = b.possible_moves(&self.piece);

        if possible_moves.is_empty() {
            None
        } else {
            Some(super::Move(super::Column::A, super::Row::One, self.piece.clone()))
        }
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
    pub fn forward(i: i8, d: &Direction) -> Option<usize> {
        let delta = match d {
            Direction::Up => -8,
            Direction::UpRight => -7,
            Direction::Right => 1,
            Direction::DownRight => 9,
            Direction::Down => 8,
            Direction::DownLeft => 7,
            Direction::Left => -1,
            Direction::UpLeft => -9,
        };
        return Direction::validate(i + delta)
    }

    fn validate(i: i8) -> Option<usize> {
        if i < 0 || i >= 64 {
            return None
        } else {
            return Some(i as usize)
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

    pub fn possible_moves(&self, p: &Piece) -> Vec<super::Move> {
        let mut moves = Vec::new();

        for i in 0..64 {
            println!("s: {}", i);
            if let None = self.repl[i] {
                for d in [Direction::Up, Direction::UpRight, Direction::Right, Direction::DownRight, Direction::Down, Direction::DownLeft, Direction::Left, Direction::UpLeft].iter() {
                    if self.check_dir(i, d, p) {
                        let m = super::Move::new(i, p);
                        println!("{:?}", m);
                        break;
                    }
                }
            }
        }

        return moves
    }

    fn check_dir(&self, i: usize, d: &Direction, p: &Piece) -> bool {
        println!("i: {}, d: {:?}", i, d);

        let j = Direction::forward(i as i8, d);

        match j {
            None => println!("no"),
            Some(x) => println!("{}", x),
        };

        match j {
            None => false,
            Some(k) => {
                if let Some(x) = &self.repl[k] {
                    if !x.same(p) {
                        let mut j = k;
                        loop {
                            let l = Direction::forward(k as i8, d);
                            match l {
                                None => return false,
                                Some(k) => {
                                    j = k;
                                    println!("{}", k);
                                    if let Some(x) = &self.repl[k] {
                                        if x.same(p) {
                                            return true
                                        }
                                    } else {
                                        return false
                                    }
                                },
                            };
                        }
                    } else {
                        return false
                    }
                } else {
                    return false
                }
            }
        }
    }
}
