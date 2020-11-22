use crate::symbol::Symbol;

pub struct Player {
	pub name: String,
	pub symbol: Symbol
}

impl Player {
	pub fn new(name: String, symbol: Symbol) -> Player {
		Player {
			name,
			symbol
		}
	}
}
