use core::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
	chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &str) -> Lexer {
		Lexer {
			chars: input.chars(),
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		self.chars.find(|&c| !c.is_whitespace())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tokens_should_not_contain_white_space() {
		let s = "let five = 5;";
		let tokens = ['l', 'e', 't', 'f', 'i', 'v', 'e', '=', '5', ';'].into_iter();

		assert!(Lexer::new(s).eq(tokens));
	}
}
