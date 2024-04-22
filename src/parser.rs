use crate::tokenizer::Token;
use std::collections::{HashMap, HashSet};

const KEYWORDS: [&str; 26] = [
	"auto",
	"break",
	"case",
	"char",
	"const",
	"continue",
	"default",
	"do",
	// "double",
	"else",
	"enum",
	"extern",
	// "float",   
	"for",
	"goto",
	"if",
	"int",
	// "long",
	"register",
	"return",
	// "short",
	// "signed",
	"sizeof",
	"static",
	"struct",
	"switch",
	"typedef",
	"union",
	// "unsigned",
	"void",
	"volatile",
	"while"
];

const SYMBOLS: [&str; 33] = [
	"+",
	"-",
	"*",
	"/",
	"%",
	"^",
	"#",
	"&",
	"~",
	"|",
	"<<",
	">>",
	"//",
	"==",
	"~=",
	"<=",
	">=",
	"<",
	">",
	"=",
	"(",
	")",
	"{",
	"}",
	"[",
	"]",
	"::",
	";",
	":",
	",",
	".",
	"..",
	"..."
];

#[derive(Debug)]
pub enum Type {
	Void,
	Int,
	Char
}

#[derive(Debug)]
pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulo,

	Equals,
	NotEquals,
	LessThan,
	GreaterThan,
	LessThanOrEqual,
	GreaterThanOrEqual,

	And,
	Or,
	Not,
	Xor

}

#[derive(Debug)]
pub enum Node {
	None,

	NumberLiteral(i32),
	CharLiteral(char),
	StringLiteral(String),

	Arithmetic {
		operator: Operator,
		left: Box<Node>,
		right: Box<Node>
	},

	Comparison {
		operator: Operator,
		left: Box<Node>,
		right: Box<Node>
	},

	Variable {
		name: String,
		constant: bool,
		var_type: Type,
		value: Option<Vec<Node>>
	},

	VariableAssignment {
		name: String,
		value: Box<Node>
	},

	Struct {
		name: String,
		fields: HashMap<String, Type>
	},

	Enum {
		name: String,
		values: HashSet<String>
	},

	Function {
		name: String,
		return_type: Type,
		args: HashMap<String, Type>,
		body: Vec<Node>
	},

	FunctionCall {
		name: String,
		args: Vec<Node>
	},

	If {
		condition: Box<Node>,
		body: Vec<Node>,
		else_body: Vec<Node>
	},

	For {
		initializer: Box<Node>,
		condition: Box<Node>,
		iterator: Box<Node>,
		body: Vec<Node>
	},

	While {
		condition: Box<Node>,
		body: Vec<Node>
	},

	DoWhile {
		condition: Box<Node>,
		body: Vec<Node>
	},

	Return {
		value: Box<Node>
	},

	Break,

	Continue
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
	let mut pointer: usize = 0;
	let mut tree: Vec<Node> = Vec::new();

	while pointer < tokens.len() {
		let token: &Token = &tokens[pointer];
		let next_token: Option<&Token> = tokens.get(pointer + 1);
		let last_token: Option<&Token> = if pointer > 0 { tokens.get(pointer - 1) } else { None };

		pointer += 1;

		match token {
			Token::None => { tree.push(Node::None) },

			Token::Identifier(name) => {
				match name.as_str() {
					"int" | "void" | "char" => {
						parse_variable(name, next_token, last_token, &tokens, &mut pointer, &mut tree);

						// parse_function()
					},

					"if" => {

					},

					"for" => {

					},

					"while" => {

					},

					"do" => {

					},

					"return" => {

					},
					
					"break" => {

					},

					"continue" => {

					},
					

					_ => {
						
					}
				}
			},

			_ => {
				// Number literal
				if let Token::Number(value) = token {
					match next_token {
						// Arithmetic operation
						Some(Token::Operator(op)) => {
							parse_arithmetic(*op, value.to_string(), &tokens, &mut pointer);
						}

						// Just a number
						_ => {
							tree.push(Node::NumberLiteral(value.parse().expect("Invalid number literal")));
						}
					}
				}

				// Char literal
				if let Token::Char(value) = token {
					tree.push(Node::CharLiteral(*value));
				}

				// String literal
				if let Token::String(value) = token {
					tree.push(Node::StringLiteral(value.to_string()));
				}
			}
		}
	}
	
	return tree;
}

fn parse_arithmetic(op: char, value: String, tokens: &Vec<Token>, pointer: &mut usize) {
	// let operator: Operator = match op {
	// 	'+' => Operator::Add,
	// 	'-' => Operator::Subtract,
	// 	'*' => Operator::Multiply,
	// 	'/' => Operator::Divide,
	// 	'%' => Operator::Modulo,
           
	// 	_ => panic!("Invalid operator")
	// };


	// let mut left: Box<Node> = Box::new(Node::NumberLiteral(value.parse::<i32>().expect("Invalid number literal")));
	// let mut right: Box<Node> = Box::new(Node::None);
1
	// *pointer += 1;

	// while *pointer < tokens.len() {
	// 	let token = &tokens[*pointer];
	// 	*pointer += 1;

	// 	if let Token::Number(value) = token {
	// 		right = Box::new(Node::NumberLiteral(value.parse::<i32>().expect("Invalid number literal")));
	// 	}

	// 	// This compiler will only support arithmetic operations that end with a semicolon
	// 	// What that basically means is:
	// 	// int a = 1 + 2; will compile
	// 	// printf(1 + 2); will not compile
	// 	if let Token::Separator(';') = token {
	// 		break;
	// 	}
	// }
}

fn parse_comparison() {

}

fn parse_variable(name: &String, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &Vec<Token>, pointer: &mut usize, tree: &mut Vec<Node>) {
	let variable_type: Type = match name.as_str() {
		"int" => Type::Int,
		"void" => Type::Void,
		"char" => Type::Char,
		_ => panic!("Invalid type")
	};

	let variable_name: &Token = next_token.expect("Unknown variable name");

	let mut variable_tokens: Vec<Token> = Vec::new();

	if *&tokens.get(*pointer + 1) == Some(&Token::Separator(';'))  {
		// Uninitialized variable
	} else {
		// Initialized variable
		*pointer += 2;

		while *pointer < tokens.len() {
			let token = &tokens[*pointer];
			*pointer += 1;
	
			if *token == Token::Separator(';') {
				break;
			} else {
				variable_tokens.push(token.clone());
			}
		}
	}

	let parsed_variable_tokens: Vec<Node> = parse(variable_tokens);

	tree.push(Node::Variable {
		name: variable_name.to_string(),
		constant: last_token == Some(&Token::Identifier("const".to_string())),
		var_type: variable_type,
		value: if parsed_variable_tokens.len() > 0 { Some(parsed_variable_tokens) } else { None }
	});
}

fn parse_function() {

}

fn parse_if() {

}

fn parse_for() {

}

fn parse_while() {

}

fn parse_do_while() {

}

fn parse_return() {

}

fn parse_break() {

}

fn parse_continue() {

}