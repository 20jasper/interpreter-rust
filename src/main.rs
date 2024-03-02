use std::io;

use interpreter_rust::repl;

fn main() -> io::Result<()> {
	println!("Hello! This is the Monkey programming language!");
	println!("Please type a commmand:");

	repl::run()
}
