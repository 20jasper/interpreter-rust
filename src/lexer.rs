use core::{iter::Peekable, str::Chars};

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
	chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &str) -> Lexer {
		Lexer {
			chars: input.chars().peekable(),
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self
			.chars
			.find(|&c| !c.is_whitespace())?;

		Token::from_char(next).or_else(|| {
			let mut s = next.to_string();

			while let Some(c) = self.chars.peek() {
				if !c.is_alphabetic() {
					break;
				}
				s.push(self.chars.next().unwrap());
			}

			Some(Token::Identifier(s))
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tokens_should_not_contain_white_space() {
		let s = "( = ;";

		let tokens = Lexer::new(s).collect::<Vec<Token>>();
		let expected = vec![Token::LeftParen, Token::Assign, Token::Semicolon];

		assert_eq!(tokens, expected);
	}

	#[test]
	fn should_parse_single_char_tokens() {
		let s = "=+-!*/<,>(){};";

		let tokens = Lexer::new(s).collect::<Vec<Token>>();
		let expected = vec![
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

		assert_eq!(tokens, expected);
	}

	#[test]
	fn should_parse_identifiers() {
		let s = "(  five hello  what;";

		let tokens = Lexer::new(s).collect::<Vec<Token>>();
		let expected = vec![
			Token::LeftParen,
			Token::Identifier("five".to_string()),
			Token::Identifier("hello".to_string()),
			Token::Identifier("what".to_string()),
			Token::Semicolon,
		];

		assert_eq!(tokens, expected);
	}
}
