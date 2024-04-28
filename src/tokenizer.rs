#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	None,

	Char(char),
	String(String),
	Number(String),

	Identifier(String),
	Operator(char),
	Separator(char),
}

impl ToString for Token {
	fn to_string(&self) -> String {
		match self {
			Token::None => "".to_string(),
			
			Token::Char(value) => value.to_string(),
			Token::String(value) => value.to_string(),
			Token::Number(value) => value.to_string(),

			Token::Identifier(value) => value.to_string(),
			Token::Operator(value) => value.to_string(),
			Token::Separator(value) => value.to_string(),
		}
	}
}

pub fn tokenize(source: &str) -> Vec<Token> {
	let mut pointer: usize = 0;
	let mut tokens: Vec<Token> = Vec::new();
	
	while pointer < source.len() {
		let char: char = source.chars().nth(pointer).unwrap();
		let next_char: char = source.chars().nth(pointer + 1).unwrap_or(' ');

		pointer += 1;
		
		match char {
			// Math operators
			'+' | '-' | '*' | '%' | '^' => {
				tokens.push(Token::Operator(char))
			}
			
			// Comparison operators
			'=' | '!' | '<' | '>' | '~' => {
				tokens.push(Token::Operator(char))
			}

			// Brackets
			'(' | ')' | '{' | '}' | '[' | ']' => {
				tokens.push(Token::Separator(char))
			}

			// Other symbols
			',' | ';' | ':' | '.' => {
				tokens.push(Token::Separator(char))
			}

			'/' => { // Comments
				if next_char == '/' { // Single line comment
					while pointer < source.len() {
						let character:char = source.chars().nth(pointer).unwrap();
						pointer += 1;

						if &character == &'\n' {
							break;
						}
					}
				} else if next_char == '*' { // Multi line comment
					while pointer < source.len() {
						let character:char = source.chars().nth(pointer).unwrap();
						let next_char:char = if pointer == source.len() - 1 {
							' '
						} else {
							source.chars().nth(pointer + 1).unwrap_or(' ')
						};

						pointer += 1;

						if character == '*' && next_char == '/' {
							pointer += 1; // Wrongly adds a MathOperator(Divide) without this
							break;
						}
					}
				} else { // Divide
					tokens.push(Token::Operator('/'))
				}
			}
			'0'..='9' => { // Integers
				let mut num = String::new();
				num.push(char);  // dont forget to push the first character

				while pointer < source.len() {
					let character:char = source.chars().nth(pointer).unwrap();
					if character.is_numeric() {
						num.push(character);
						pointer += 1;
					} else {
						break;
					}
				}

				tokens.push(Token::Number(num));
			}
			'"' => { // Strings
				let mut string = String::new();
				let mut ended = false;
				
				while pointer < source.len() {
					let character:char = source.chars().nth(pointer).unwrap();
					if character == '"' {
						pointer += 1;
						ended = true;
						break;
					} else {
						string.push(character);
						pointer += 1;
					}
				}

				if !ended {
					panic!("String literal must be closed with a \"");
				}

				tokens.push(Token::String(string));
			}
			'\'' => { // Characters
				let mut string = String::new();
				let mut ended = false;

				while pointer < source.len() {
					let character:char = source.chars().nth(pointer).unwrap();
					if character == '\'' {
						pointer += 1;
						ended = true;
						break;
					} else {
						string.push(character);
						pointer += 1;
					}
				}

				if !ended {
					panic!("Character literal must be closed with a \'");
				}

				if string.len() > 1 {
					panic!("Character literal must be a single character");
				}

				tokens.push(Token::Char(string.chars().next().unwrap()));
			}
			_ => {
				if char.is_alphabetic() {
					let mut string = String::new();
					string.push(char);

					while pointer < source.len() {
						let character:char = source.chars().nth(pointer).unwrap();
						if character.is_alphabetic() {
							string.push(character);
							pointer += 1;
						} else {
							break;
						}
					}

					tokens.push(Token::Identifier(string))
				}
			}
		}
	}

	return tokens;
}
