pub struct Board {
	pub board: [[String; 3]; 3]
}

impl Board {
	pub fn new() -> Board {
		Board {
			board: [
				[String::from(" "), String::from(" "), String::from(" ")],
				[String::from(" "), String::from(" "), String::from(" ")],
				[String::from(" "), String::from(" "), String::from(" ")]
			]
		}
	}
}