use std::process::Command;
use std::io;

pub fn clear() -> () {
	if cfg!(windows) {
		let _ = Command::new("cmd.exe").arg("/c").arg("cls").status();
	} else {
	    let _ = Command::new("sh").arg("-c").arg("clear").status();
	}
}

pub fn read() -> String {
	let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    return name
}

pub fn parse_error_msg(arg: String) -> () {
	println!("\n\n-------------------------------");
	println!("[ERROR! {} must be a unsigned integer.]", arg);
	println!("Play again!");
	println!("-------------------------------\n\n");
}

pub fn position_occupied_error_msg(row: usize, col: usize) -> () {
	println!("\n\n-------------------------------");
	println!("[ERROR position [{}][{}] is already occupied.]", row, col);
	println!("Play again!");
	println!("-------------------------------\n\n");
}

pub fn index_out_of_bounds_error_msg(arg: String) -> () {
	println!("\n\n-------------------------------");
	println!("[ERROR index {} out of bounds. {} must be between [0, 2]]", arg, arg);
	println!("Play again!");
	println!("-------------------------------\n\n");
}
