use crate::player::Player;
use crate::symbol::Symbol;

pub struct Game {
	pub board: [[String; 3]; 3]
}

impl Game {
	pub fn play(&mut self, player: &Player, row: usize, col: usize) -> () { 
		match player.symbol {
			Symbol::O => self.board[row][col] = String::from("O"),
			Symbol::X => self.board[row][col] = String::from("X")
		}
	}

	pub fn check_win(&mut self, player: &Player) -> bool { 
		let symbol = match player.symbol {
			Symbol::O => String::from("O"),
			Symbol::X => String::from("X")
		};
		let mut main_diagonal = 0;
		let mut secondary_diagonal = 0;
		for i in 0..3 {
			let mut row = 0;
			let mut col = 0;
			for j in 0..3 {
				if self.board[i][j] == symbol {
					row += 1;
				}
				if self.board[j][i] == symbol {
					col += 1;
				}
				if i == j && self.board[i][j] == symbol {
					main_diagonal += 1; 
				}
				if j == (2 - i) && self.board[i][j] == symbol {
					secondary_diagonal += 1
				}
			}
			if row == 3 || col == 3 || main_diagonal == 3 || secondary_diagonal == 3 {
				return true;
			}
		}

		return false;
	}

	pub fn position_occupied(&mut self, row: usize, col: usize) -> bool {
		self.board[row][col] != " "
	}

	pub fn display(&mut self) -> () {
		println!("\n");
		for i in 0..3 {
			for j in 0..3 {
				print!("[{}] ", self.board[i][j]);
			}
			println!("");
		}
		println!("\n");
	}
}
