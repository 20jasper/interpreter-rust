use core::str::Chars;

use crate::token::Token;

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
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self
			.chars
			.find(|&c| !c.is_whitespace())?;

		Token::build(next)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tokens_should_not_contain_white_space() {
		let s = "( = ;";
		let tokens = vec![Token::LeftParen, Token::Assign, Token::Semicolon];

		assert_eq!(Lexer::new(s).collect::<Vec<Token>>(), tokens);
	}

	#[test]
	fn should_parse_single_char_tokens() {
		let s = "=+-!*/<,>(){};";
		let tokens = vec![
			Token::Assign,
			Token::Plus,
			Token::Minus,
			Token::Bang,
			Token::Asterisk,
			Token::Slash,
			Token::LessThan,
			Token::Comma,
			Token::GreaterThan,
			Token::LeftParen,
			Token::RightParen,
			Token::LeftBrace,
			Token::RightBrace,
			Token::Semicolon,
		];

		assert_eq!(Lexer::new(s).collect::<Vec<Token>>(), tokens);
	}
}
