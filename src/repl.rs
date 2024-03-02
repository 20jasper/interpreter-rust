use crate::lexer::Lexer;

use std::io::{self, Write};

pub fn run() -> io::Result<()> {
	loop {
		print!(">> ");
		io::stdout().flush()?;

		let mut input = String::default();
		io::stdin().read_line(&mut input)?;

		for token in Lexer::new(&input) {
			println!("{:?}", token);
		}
	}
}
