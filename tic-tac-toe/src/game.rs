#[derive(Debug)]
pub enum Result {
    O,
    X,
    Draw,
}

pub enum Player {
    O,
    X,
}

impl Player {
    fn next(p: &Player) -> Player {
        match p {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Player::O => 'O',
            Player::X => 'X',
        }
    }

    fn copy(&self) -> Player {
        match self {
            Player::O => Player::O,
            Player::X => Player::X,
        }
    }
}

pub struct Board {
    repl: [Option<Player>; 9],
    pub current_player: Player,
}

impl Board {
    pub fn new() -> Board {
        Board {
            repl: [None, None, None, None, None, None, None, None, None],
            current_player: Player::O,
        }
    }

    fn show(&self, place: usize) -> char {
        match &self.repl[place] {
            Some(x) => x.to_char(),
            None => ' ',
        }
    }

    pub fn print(&self) {
        println!("+-+-+-+");
        println!("|{}|{}|{}|", self.show(0), self.show(1), self.show(2));
        println!("+-+-+-+");
        println!("|{}|{}|{}|", self.show(3), self.show(4), self.show(5));
        println!("+-+-+-+");
        println!("|{}|{}|{}|", self.show(6), self.show(7), self.show(8));
        println!("+-+-+-+");
    }

    pub fn put(&mut self, place: usize) {
        self.repl[place] = Some(self.current_player.copy());
        self.current_player = Player::next(&self.current_player);
    }

    pub fn get(&self, place: usize) -> &Option<Player> {
        &self.repl[place]
    }

    pub fn game_end(&self) -> Option<Result> {
        for i in 0..3 {
            match (&self.repl[i*3], &self.repl[i*3+1], &self.repl[i*3+2]) {
                (Some(Player::X), Some(Player::X), Some(Player::X)) => return Some(Result::X),
                (Some(Player::O), Some(Player::O), Some(Player::O)) => return Some(Result::O),
                _ => continue,
            }
        }

        for i in 0..3 {
            match (&self.repl[i], &self.repl[i+3], &self.repl[i+6]) {
                (Some(Player::X), Some(Player::X), Some(Player::X)) => return Some(Result::X),
                (Some(Player::O), Some(Player::O), Some(Player::O)) => return Some(Result::O),
                _ => continue,
            }
        }

        match (&self.repl[0], &self.repl[4], &self.repl[8]) {
            (Some(Player::X), Some(Player::X), Some(Player::X)) => return Some(Result::X),
            (Some(Player::O), Some(Player::O), Some(Player::O)) => return Some(Result::O),
            _ => (),
        }

        match (&self.repl[2], &self.repl[4], &self.repl[6]) {
            (Some(Player::X), Some(Player::X), Some(Player::X)) => return Some(Result::X),
            (Some(Player::O), Some(Player::O), Some(Player::O)) => return Some(Result::O),
            _ => (),
        }

        for i in 0..9 {
            match &self.repl[i] {
                Some(_) => continue,
                None => return None,
            }
        }

        Some(Result::Draw)
    }
}
