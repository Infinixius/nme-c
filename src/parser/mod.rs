mod arithmetic;
mod comparision;
mod expression;
mod for_loop;
mod function;
mod if_else;
mod return_statement;
mod variable;
mod while_loop;

use crate::parser::expression::parse_expression;
use crate::parser::for_loop::parse_for_loop;
use crate::parser::function::{parse_function_declaration, parse_function_call};
use crate::parser::if_else::parse_if_else;
use crate::parser::return_statement::parse_return_statement;
use crate::parser::variable::{parse_variable, parse_variable_assignment};
use crate::parser::while_loop::parse_while_loop;

use crate::tokenizer::Token;
use std::collections::HashSet;
use std::path::Display;

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

	Arithmetic,
	Comparision
}

impl std::fmt::Display for Context {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Context::Program => write!(f, "program"),
			Context::FunctionArguments => write!(f, "func_arg"),
			Context::FunctionBody => write!(f, "func_body"),

			Context::VariableDeclaration => write!(f, "var_decl"),

			Context::Arithmetic => write!(f, "arithmetic"),
			Context::Comparision => write!(f, "comparision")
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
	Arithmetic {
		operator: Operator,
		left: Box<Node>,
		right: Box<Node>
	},

	Comparison {
		operator: Operator,
		left: Box<Node>,
		right: Box<Node>
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

	Expression(Expression),

	Variable {
		name: String,
		constant: bool,
		var_type: Type,
		value: Option<Box<Node>>
	},

	VariableAssignment {
		name: String,
		value: Vec<Node>
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

		debugln!("main_parse({}) : {:?} (last: {:?} next: {:?}) pointer: {}", context, token, last_token, next_token, pointer);

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
						todo!()
					},

					"for" => {
						todo!()
					},

					"while" => {
						todo!()
					},

					"do" => {
						todo!()
					},

					"return" => {
						todo!()
					},
					
					"break" => {
						tree.push(Node::Break);
					},

					"continue" => {
						tree.push(Node::Continue);
					},

					"register" => {
						todo!()
					},

					"const" => {}, // Ignore, as we parse constants in the variable declaration

					"asm" => {
						todo!()
					},

					"auto" | "double" | "extern" | "float" | "goto" | "long" | "restrict" | "short" | "sizeof" | "static" | "typedef" | "union" | "unsigned" => {
						panic!("The {} keyword is not supported", identifier);
					},

					"bool*" => {
						panic!("Bool pointers are not supported");
					},

					// Color literals for the TMS9918 graphics chip
					"TRANSPARENT" | "BLACK" | "MEDIUM_GREEN" | "LIGHT_GREEN" | "DARK_BLUE" | "LIGHT_BLUE" | "DARK_RED" | "CYAN" | "MEDIUM_RED" | "LIGHT_RED" | "DARK_YELLOW" | "LIGHT_YELLOW" | "DARK_GREEN" | "MAGENTA" | "GRAY" | "WHITE" => {
						tree.push(Node::NumberLiteral(match identifier.as_str() {
							"TRANSPARENT" => 0,
							"BLACK" => 1,
							"MEDIUM_GREEN" => 2,
							"LIGHT_GREEN" => 3,
							"DARK_BLUE" => 4,
							"LIGHT_BLUE" => 5,
							"DARK_RED" => 6,
							"CYAN" => 7,
							"MEDIUM_RED" => 8,
							"LIGHT_RED" => 9,
							"DARK_YELLOW" => 10,
							"LIGHT_YELLOW" => 11,
							"DARK_GREEN" => 12,
							"MAGENTA" => 13,
							"GRAY" => 14,
							"WHITE" => 15,
							_ => {
								panic!("Invalid color: {}", identifier);
							}
						}));
					},

					"true" | "false" => {
						tree.push(Node::BooleanLiteral(
							match identifier.as_str() {
								"true" => true,
								"false" => false,
								_ => false
							}
						));
					},

					_ => {
						// Variable reference
						match next_token {
							Some(Token::Operator(op)) => { // Arithmetic operation
								if *op == '=' {
									parse_variable_assignment(identifier, next_token, last_token, &tokens, &mut pointer, &mut tree);
								} else {
									parse_expression(*op, Token::Identifier(identifier.to_string()), &tokens, &mut pointer, &mut tree);
								}
							}

							Some(Token::Separator('(')) => { // Function call
								parse_function_call(identifier, next_token, last_token, &tokens, &mut pointer, &mut tree);
							}
	
							_ => { // Just a variable reference?
								debugln!("naive variable reference: {}", identifier);
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
							parse_expression(*op, token.clone(), &tokens, &mut pointer, &mut tree);
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