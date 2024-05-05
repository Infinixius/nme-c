use crate::parser::{parse, Context, Node, Type, Token};
use std::collections::HashSet;

pub fn parse_function_declaration(identifier: &str, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	let return_type: Type = match identifier {
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

		debugln!("parse_function_declaration_arguments token: {:?}", token);

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

		debugln!("parse_function_declaration_body token: {:?}", token);

		if *token == Token::Separator('}') {
			break;
		} else if *token == Token::Separator('{') {
			continue;
		} else {
			function_tokens.push(token.clone());
		}
	}

	let node = Node::Function {
		name: name.to_string(),
		return_type: return_type,
		args: arguments.into_iter().collect(),
		body: if function_tokens.len() > 0 {
			parse(function_tokens, Context::FunctionBody)
		} else {
			Vec::new()
		}
	};

	debugln!("parse_variable new node: {:?}", node);
	

	tree.push(node);
}

pub fn parse_function_call(identifier: &str, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	println!("parse_function_call({}, {:?}, {:?}, {:?}, {}, {:?})", identifier, next_token, last_token, tokens, pointer, tree);
}