mod board;
mod player;
mod symbol;
mod utils;
mod game;
 
use crate::board::Board;
use crate::player::Player;
use crate::symbol::Symbol;
use crate::game::Game;

extern crate reader;

use reader::{input, int};

const PLAYER1_TURN: bool = true;
const PLAYER2_TURN: bool = false; 
const STRING: i64 = 1000;

fn main() {

	utils::clear();

	let player1_name = input("Enter the Player 1 name: ");
	let player2_name = input("Enter the Player 2 name: ");

	utils::clear();

	let player1 = Player::new(player1_name, Symbol::O);
	let player2 = Player::new(player2_name, Symbol::X);

	let board = Board::new().board;
	let mut game = Game{board: board};

	let mut finish: bool = false;
	let mut turn: bool = PLAYER1_TURN;

	while !finish {
		if turn { // turn = true: player 1, turn = false : player 2
			println!("IT'S YOUR TURN {}\n\n", player1.name.to_uppercase());
			let row = int(input("Row: ")).unwrap_or(STRING) as usize;
			if row != STRING as usize {
				if row >= 0 && row <= 2 {
					let col = int(input("Col: ")).unwrap_or(STRING) as usize;
					if col != STRING as usize {
						if col >= 0 && col <= 2 {
							if game.position_occupied(row, col) {
								utils::position_occupied_error_msg(row, col);
							} else {
								game.play(&player1, row, col);
								game.display();
								let win = game.check_win(&player1);
								if win {
									println!("WINNER: {}", player1.name.to_uppercase());
									println!("CONGRATULATIONS!!!");
									finish = true;
								} else {
									turn = PLAYER2_TURN;
								}
							}
						} else {
							utils::index_out_of_bounds_error_msg(String::from("Col"));
						}
						
					} else {
						utils::parse_error_msg(String::from("Col"));
					}
				} else {
					utils::index_out_of_bounds_error_msg(String::from("Row"));
				}
				
			} else {
				utils::parse_error_msg(String::from("Row"));
			}
		} else {
			println!("IT'S YOUR TURN {}\n\n", player2.name.to_uppercase());
			let row = int(input("Row: ")).unwrap_or(STRING) as usize;
			if row != STRING as usize {
				if row >= 0 && row <= 2 {
					let col = int(input("Col: ")).unwrap_or(STRING) as usize;
					if col != STRING as usize{
						if col >= 0 && col <= 2 {
							if game.position_occupied(row, col) {
								utils::position_occupied_error_msg(row, col);
							} else {
								game.play(&player2, row, col);
								game.display();
								let win = game.check_win(&player2);
								if win {
									println!("WINNER: {}", player2.name.to_uppercase());
									println!("CONGRATULATIONS!!!");
									finish = true;
								} else {
									turn = PLAYER1_TURN;
								}
							}
						} else {
							utils::index_out_of_bounds_error_msg(String::from("Col"));
						}
						
					} else {
						utils::parse_error_msg(String::from("Col"));
					}
				} else {
					utils::index_out_of_bounds_error_msg(String::from("Row"));
				}
				
			} else {
				utils::parse_error_msg(String::from("Row"));
			}
		}
	}
}
