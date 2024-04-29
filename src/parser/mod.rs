mod arithmetic;
mod comparision;
mod for_loop;
mod function_call;
mod function_declaration;
mod if_else;
mod return_statement;
mod variable;
mod while_loop;

use crate::parser::arithmetic::parse_arithmetic;
use crate::parser::comparision::parse_comparison;
use crate::parser::for_loop::parse_for_loop;
use crate::parser::function_call::parse_function_call;
use crate::parser::function_declaration::parse_function_declaration;
use crate::parser::if_else::parse_if_else;
use crate::parser::return_statement::parse_return_statement;
use crate::parser::variable::parse_variable;
use crate::parser::while_loop::parse_while_loop;

use crate::tokenizer::Token;
use std::collections::HashSet;
use std::path::Display;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	Void,
	Int,
	IntPointer,
	Char,
	CharPointer,
	Boolean
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
pub enum Context {
	Program,
	FunctionArguments,
	FunctionBody,

	VariableDeclaration,

	Arithmetic
}

impl std::fmt::Display for Context {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Context::Program => write!(f, "program"),
			Context::FunctionArguments => write!(f, "func_arg"),
			Context::FunctionBody => write!(f, "func_body"),

			Context::VariableDeclaration => write!(f, "var_decl"),

			Context::Arithmetic => write!(f, "arithmetic")
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
	None,

	NumberLiteral(i32),
	CharLiteral(char),
	BooleanLiteral(bool),
	StringLiteral(String),

	VariableReference(String),

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
		fields: HashSet<(String, Type)>
	},

	Enum {
		name: String,
		values: HashSet<String>
	},

	Function {
		name: String,
		return_type: Type,
		args: HashSet<(String, Type)>,
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

	// DoWhile {
	// 	condition: Box<Node>,
	// 	body: Vec<Node>
	// },

	Return {
		value: Box<Node>
	},

	Break,

	Continue
}

pub fn parse(tokens: Vec<Token>, context: Context) -> Vec<Node> {
	let mut pointer: usize = 0;
	let mut tree: Vec<Node> = Vec::new();

	while pointer < tokens.len() {
		if tokens.get(pointer) == None {
			break;
		}

		let token: &Token = &tokens.get(pointer).expect("Invalid token");
		let next_token: Option<&Token> = tokens.get(pointer + 1);
		let last_token: Option<&Token> = if pointer > 0 { tokens.get(pointer - 1) } else { None };

		debugln!("parse token_{}: {:?} (last: {:?} next: {:?}) pointer: {}", context, token, last_token, next_token, pointer);

		match token {
			Token::None => { tree.push(Node::None) },

			Token::Identifier(identifier) => {
				match identifier.as_str() {
					"bool" | "int" | "int*" | "void" | "void*" | "char" | "char*" => {
						match tokens.get(pointer + 2).expect("Invalid variable/function declaration") {
							Token::Operator('=') | Token::Separator(';') => {
								parse_variable(identifier, next_token, last_token, &tokens, &mut pointer, &mut tree);
							},
							Token::Separator('(') => {
								parse_function_declaration(identifier, next_token, last_token, &tokens, &mut pointer, &mut tree);
							},
							_ => {
								panic!("Invalid variable/function declaration: {:?}", tokens.get(pointer + 1))
							}
						}
					},

					"if" => {

					},

					"for" => {

					},

					"while" => {

					},

					"return" => {

					},
					
					"break" => {
						tree.push(Node::Break);
					},

					"continue" => {
						tree.push(Node::Continue);
					},

					"const" => {}, // Ignore, as we parse constants in the variable declaration

					"asm" => {},

					"auto" | "do" | "double" | "extern" | "float" | "long" | "register" | "short" | "signed" | "static" | "typedef" | "union" | "unsigned" => {
						panic!("The {} keyword is not supported", identifier);
					},

					"bool*" => {
						panic!("Bool pointers are not supported");
					},

					"true" | "false" => {
						tree.push(Node::BooleanLiteral(identifier == "true"));
					},

					_ => {
						// Variable reference
						match next_token {
							Some(Token::Operator(op)) => { // Arithmetic operation
								parse_arithmetic(*op, token.clone(), &tokens, &mut pointer, &mut tree);
							}
	
							_ => { // Just a variable reference?
								tree.push(Node::VariableReference(identifier.to_string()));
							}
						}
					}
				}
			},

			_ => {
				if let Token::Number(value) = token { // Number literal
					match next_token {
						Some(Token::Operator(op)) => { // Arithmetic operation
							parse_arithmetic(*op, token.clone(), &tokens, &mut pointer, &mut tree);
						}

						_ => { // Just a number
							tree.push(Node::NumberLiteral(value.parse().expect("Invalid number literal")));
						}
					}
				} else if let Token::Char(value) = token { // Char literal
					tree.push(Node::CharLiteral(*value));
				} else if let Token::String(value) = token { // String literal
					tree.push(Node::StringLiteral(value.to_string()));
				} else {
					panic!("Invalid token: {:?}", token);
				}
			}
		}

		pointer += 1;
	}
	
	return tree;
}