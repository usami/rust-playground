mod game;
use crate::game::{Game, Strategy};

fn main() {
    let mut game = Game::new(Strategy::Monkey, Strategy::Monkey);

    game.print_board();

    loop {
        if game.end() {
            // result
            break;
        }

        let player = game.current_player();

        if let Some(m) = player.select_move(&game.board) {
            game.take(&m);
            game.print_board();
        }
        game.next_turn();
    }
}
