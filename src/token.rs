#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
	// Eof,
	Identifier(&'a str),
	Illegal(String),
	Int(i32),
	Assign,
	Eq,
	NotEq,
	Plus,
	Minus,
	Bang,
	Asterisk,
	Slash,
	LessThan,
	GreaterThan,
	Comma,
	Semicolon,
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Function,
	Let,
	True,
	False,
	If,
	Else,
	Return,
}

impl<'a> Token<'a> {
	pub fn try_from_char(s: char) -> Option<Token<'a>> {
		use Token as T;
		match s {
			'=' => Some(T::Assign),
			'+' => Some(T::Plus),
			'-' => Some(T::Minus),
			'!' => Some(T::Bang),
			'*' => Some(T::Asterisk),
			'/' => Some(T::Slash),
			'<' => Some(T::LessThan),
			'>' => Some(T::GreaterThan),
			',' => Some(T::Comma),
			';' => Some(T::Semicolon),
			'(' => Some(T::LeftParen),
			')' => Some(T::RightParen),
			'{' => Some(T::LeftBrace),
			'}' => Some(T::RightBrace),
			_ => None,
		}
	}
}

impl Token<'_> {
	pub fn from_string(s: &str) -> Token<'_> {
		use Token as T;
		match s {
			"let" => T::Let,
			"fn" => T::Function,
			"true" => T::True,
			"false" => T::False,
			"if" => T::If,
			"else" => T::Else,
			"return" => T::Return,
			_ => T::Identifier(s),
		}
	}
}
