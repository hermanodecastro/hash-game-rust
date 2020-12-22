#[allow(unused_imports)]
use super::*;

#[test]
fn player_win_first_row() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 0, 0);
	game.play(&player, 0, 1);
	game.play(&player, 0, 2);
	assert!(game.check_win(&player), true);
}

#[test]
fn player_win_second_row() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 1, 0);
	game.play(&player, 1, 1);
	game.play(&player, 1, 2);
	assert!(game.check_win(&player), true);
}

#[test]
fn player_win_third_row() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 2, 0);
	game.play(&player, 2, 1);
	game.play(&player, 2, 2);
	assert!(game.check_win(&player), true);
}

#[test]
fn player_win_first_col() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 0, 0);
	game.play(&player, 1, 0);
	game.play(&player, 2, 0);
	assert!(game.check_win(&player), true);
}

#[test]
fn player_win_second_col() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 0, 1);
	game.play(&player, 1, 1);
	game.play(&player, 2, 1);
	assert!(game.check_win(&player), true);
}

#[test]
fn player_win_third_col() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 0, 2);
	game.play(&player, 1, 2);
	game.play(&player, 2, 2);
	assert!(game.check_win(&player), true);
}

#[test]
fn player_win_main_diagonal() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 0, 0);
	game.play(&player, 1, 1);
	game.play(&player, 2, 2);
	assert!(game.check_win(&player), true);
}

#[test]
fn player_win_secondary_diagonal() {
	let player = Player {
		name: String::from("Player"),
		symbol: Symbol::X
	};
	let board = Board::new().board;
	let mut game = Game{board};
	game.play(&player, 0, 2);
	game.play(&player, 1, 1);
	game.play(&player, 2, 0);
	assert!(game.check_win(&player), true);
}



