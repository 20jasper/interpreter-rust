use crate::lexer::Lexer;

use std::io::{self, Write};

pub fn run(stdin: &mut io::Stdin, stdout: &mut io::Stdout) -> io::Result<()> {
	loop {
		print!(">> ");
		stdout.flush()?;

		let mut input = String::default();
		stdin.read_line(&mut input)?;

		for token in Lexer::new(&input) {
			println!("{token:?}");
		}
	}
}
