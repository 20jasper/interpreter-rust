use core::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
	chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			chars: input.chars(),
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		self.chars.next()
	}
}
