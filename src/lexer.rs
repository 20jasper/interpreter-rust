use core::{iter::Peekable, str::CharIndices};

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
	input: &'a str,
	chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Lexer<'a> {
		Lexer {
			input,
			chars: input.char_indices().peekable(),
		}
	}

	fn build_str_while<F>(&mut self, start: usize, first: char, condition: F) -> &str
	where
		F: Fn(char) -> bool,
	{
		let mut end = start + first.len_utf8();

		while let Some((i, c)) = self
			.chars
			.next_if(|(_, c)| condition(*c))
		{
			end = i + c.len_utf8();
		}

		&self.input[start..end]
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let (i, next) = self
			.chars
			.find(|(_, c)| !c.is_whitespace())?;

		if self
			.chars
			.next_if(|(_, c)| c == &'=')
			.is_some()
		{
			if next == '=' {
				return Some(Token::Eq);
			}
			return Some(Token::NotEq);
		}

		Token::try_from_char(next).or_else(|| {
			if next.is_alphabetic() {
				let s = self.build_str_while(i, next, char::is_alphabetic);

				Some(Token::from_string(s))
			} else if next.is_numeric() {
				let s = self.build_str_while(i, next, char::is_numeric);

				match s.parse() {
					Ok(num) => Some(Token::Int(num)),
					Err(e) => Some(Token::Illegal(e.to_string())),
				}
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

	#[test]
	fn should_parse_multi_character_non_alphanumeric_tokens() {
		let s = "!= == ";
		let tokens = Lexer::new(s).collect::<Vec<Token>>();

		let expected = vec![Token::NotEq, Token::Eq];

		assert_eq!(tokens, expected);
	}
}
