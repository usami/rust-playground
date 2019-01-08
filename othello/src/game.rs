pub enum Result {
    O,
    X,
    Draw,
}

pub enum Player {
    Black(Strategy),
    White(Strategy),
}

pub enum Strategy {
    Manual,
    Monkey,
}

enum Piece {
    Black,
    White,
}

pub struct Board {
    repl: [Option<Piece>; 64],
    players: [Player; 2],
    current_player_index: bool,
}

impl Board {
    pub fn new(bs: Strategy, ws: Strategy) -> Board {
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
            players: [Player::Black(bs), Player::White(ws)],
            current_player_index: false,
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

        println!("  {}", c_header);
    }
}
