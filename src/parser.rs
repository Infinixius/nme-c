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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
	Void,
	Int,
	IntPointer,
	Char,
	CharPointer
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
pub enum Context {
	Program,
	FunctionArguments,
	FunctionBody,

	VariableDeclaration
}

#[derive(Debug)]
pub enum Node {
	None,

	NumberLiteral(i32),
	CharLiteral(char),
	StringLiteral(String),
	VariableLiteral(String),

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

		println!("DEBUG: parse token: {:?} (last: {:?} next: {:?}) pointer: {}", token, last_token, next_token, pointer);

		match token {
			Token::None => { tree.push(Node::None) },

			Token::Identifier(identifier) => {
				match identifier.as_str() {
					"int" | "void" | "char" | "int*" | "void*" | "char*" => {
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

					"do" => {
						panic!("Do-while loops are not supported");
					},

					"return" => {

					},
					
					"break" => {
						tree.push(Node::Break);
					},

					"continue" => {
						tree.push(Node::Continue);
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
							parse_arithmetic(*op, value.to_string(), &tokens, &mut pointer, &mut tree);
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

		pointer += 1;
	}
	
	return tree;
}

fn parse_arithmetic(op: char, value: String, tokens: &Vec<Token>, pointer: &mut usize, tree: &mut Vec<Node>) {
	let operator: Operator = match op {
		'+' => Operator::Add,
		'-' => Operator::Subtract,
		'*' => Operator::Multiply,
		'/' => Operator::Divide,
		'%' => Operator::Modulo,
           
		_ => panic!("Invalid operator")
	};

	let left = Node::NumberLiteral(value.parse().expect("Invalid number literal"));
	let right = Node::NumberLiteral(tokens[*pointer + 1].to_string().parse().expect("Invalid number literal"));

	*pointer += 2;

	if tokens.get(*pointer) != None {
		panic!("Only two numbers are allowed in an arithmetic operation")
	}

	tree.push(Node::Arithmetic {
		operator: operator,
		left: Box::new(left),
		right: Box::new(right)
	});
}

fn parse_comparison() {

}

fn parse_variable(identifier: &String, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &Vec<Token>, pointer: &mut usize, tree: &mut Vec<Node>) {
	let constant: bool = last_token == Some(&Token::Identifier("const".to_string()));
	let variable_type: Type = match identifier.as_str() {
		"void" => Type::Void,
		"int" => Type::Int,
		"int*" => Type::IntPointer,
		"char" => Type::Char,
		"char*" => Type::CharPointer,
		_ => panic!("Invalid type")
	};

	let variable_name: &Token = next_token.expect("Unknown variable name");

	let mut variable_tokens: Vec<Token> = Vec::new();

	if *&tokens.get(*pointer + 2) == Some(&Token::Separator(';'))  {
		// Uninitialized variable
	} else {
		// Initialized variable
		*pointer += 2;

		while *pointer < tokens.len() {
			*pointer += 1;
			let token = &tokens[*pointer];

			println!("DEBUG: parse_variable token: {:?}", token);
	
			if *token == Token::Separator(';') {
				break;
			} else {
				variable_tokens.push(token.clone());
			}
		}
	}

	let parsed_variable_tokens: Vec<Node> = parse(variable_tokens, Context::VariableDeclaration);

	tree.push(Node::Variable {
		name: variable_name.to_string(),
		constant,
		var_type: variable_type,
		value: if parsed_variable_tokens.len() > 0 { Some(parsed_variable_tokens) } else { None }
	});
	// *pointer += 1;
}

fn parse_function_declaration(identifier: &String, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &Vec<Token>, pointer: &mut usize, tree: &mut Vec<Node>) {
	let return_type: Type = match identifier.as_str() {
		"void" => Type::Void,
		"int" => Type::Int,
		"int*" => Type::IntPointer,
		"char" => Type::Char,
		"char*" => Type::CharPointer,
		_ => panic!("Invalid type")
	};

	let name = next_token.expect("Invalid function name");
	
	let mut arguments: HashSet<(String, Type)> = HashSet::new();

	*pointer += 2;
	
	while *pointer < tokens.len() {
		*pointer += 1;
		let token = tokens.get(*pointer);

		println!("DEBUG: parse_function_declaration_arguments token: {:?}", token);

		match token.unwrap() {
			Token::Identifier(argument_type) => {
				match argument_type.as_str() {
					"int" | "void" | "char" | "int*" | "void*" | "char*" => {
						let argument_name: String = tokens.get(*pointer + 1).expect("Invalid argument name").to_string();
						let argument_type: Type = match argument_type.as_str() {
							"void" => Type::Void,
							"int" => Type::Int,
							"int*" => Type::IntPointer,
							"char" => Type::Char,
							"char*" => Type::CharPointer,
							_ => panic!("Invalid type")
						};

						arguments.insert((argument_name, argument_type));
						*pointer += 1;
					}

					_ => {
						panic!("Invalid argument type: {}", argument_type)
					}
				}
			},

			Token::Separator(',') => {
				continue;
			},

			Token::Separator(')') => {
				break;
			},

			_ => {
				panic!("Invalid argument type: {:?}", token)
			}
		}
	}

	*pointer += 1;

	if tokens.get(*pointer) != Some(&Token::Separator('{')) {
		panic!("Invalid function declaration")
	}

	let mut function_tokens: Vec<Token> = Vec::new();

	while *pointer < tokens.len() {
		*pointer += 1;
		let token = &tokens[*pointer];

		println!("DEBUG: parse_function_declaration_body token: {:?}", token);

		if *token == Token::Separator('}') {
			break;
		} else if *token == Token::Separator('{') {
			continue;
		} else {
			function_tokens.push(token.clone());
		}
	}

	tree.push(Node::Function {
		name: name.to_string(),
		return_type: return_type,
		args: arguments.into_iter().collect(),
		body: if function_tokens.len() > 0 {
			parse(function_tokens, Context::FunctionBody)
		} else {
			Vec::new()
		}
	});
}

fn parse_function_call() {

}

fn parse_if() {

}

fn parse_for() {

}

fn parse_while() {

}

// fn parse_do_while() {

// }

fn parse_return() {

}