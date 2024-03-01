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

	fn build_string_while<F>(&mut self, first: impl Into<String>, condition: F) -> String
	where
		F: Fn(char) -> bool,
	{
		let mut s = first.into();

		while self
			.chars
			.peek()
			.is_some_and(|c| condition(*c))
		{
			s.push(self.chars.next().unwrap());
		}

		s
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self
			.chars
			.find(|&c| !c.is_whitespace())?;

		Token::from_char(next).or_else(|| {
			if next.is_alphabetic() {
				let s = self.build_string_while(next, |c| c.is_alphabetic());

				Some(Token::from_string(s))
			} else if next.is_numeric() {
				let s = self.build_string_while(next, |c| c.is_numeric());

				Some(Token::Int(s.parse().unwrap()))
			} else {
				None
			}
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

	#[test]
	fn should_parse_keywords() {
		let s = "let return if else  fn true+false;";
		let tokens = Lexer::new(s).collect::<Vec<Token>>();

		let expected = vec![
			Token::Let,
			Token::Return,
			Token::If,
			Token::Else,
			Token::Function,
			Token::True,
			Token::Plus,
			Token::False,
			Token::Semicolon,
		];

		assert_eq!(tokens, expected);
	}

	#[test]
	fn should_parse_integers() {
		let s = "5 10 15 20;";
		let tokens = Lexer::new(s).collect::<Vec<Token>>();

		let expected = vec![
			Token::Int(5),
			Token::Int(10),
			Token::Int(15),
			Token::Int(20),
			Token::Semicolon,
		];

		assert_eq!(tokens, expected);
	}
}
