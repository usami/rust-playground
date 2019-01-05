extern crate rand;

use std::io;
use rand::Rng;

mod game;
use game::{Board, Result, Player};

fn main() {
    let mut board = Board::new();

    println!("Let's play tic-tac-toe!");

    board.print();

    loop {
        if let Some(x) = board.game_end() {
            match x {
                Result::Draw => println!("Draw!"),
                _ => println!("{:?} Wins!", x),
            }
            break;
        }

        match board.current_player {
            Player::O => {
                println!("Please input your choice from [0-8].");

                let mut choice = String::new();

                io::stdin().read_line(&mut choice)
                    .expect("Failed to read line");

                let choice: usize = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if choice > 8 {
                    println!("Please input valid number!");
                    continue
                }

                match board.get(choice) {
                    None => board.put(choice),
                    Some(_) => {
                        println!("The place is already taken!");
                        continue
                    },
                };
            },
            Player::X => {
                let pick = rand::thread_rng().gen_range(0, 9);

                match board.get(pick) {
                    None => board.put(pick),
                    Some(_) => continue,
                };
            },
        }
        board.print();
    }
}
