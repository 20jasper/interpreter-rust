use interpreter_rust::lexer::Lexer;

fn main() {
	let x = Lexer::new("let five = 5;");
	for c in x {
		dbg!(c);
	}
}
