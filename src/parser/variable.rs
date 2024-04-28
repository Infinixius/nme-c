use crate::parser::{parse, Context, Node, Type, Token};

pub fn parse_variable(identifier: &str, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	let constant: bool = last_token == Some(&Token::Identifier("const".to_string()));
	let variable_type: Type = match identifier {
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

			debugln!("parse_variable token: {:?}", token);
	
			if *token == Token::Separator(';') {
				break;
			} else {
				variable_tokens.push(token.clone());
			}
		}
	}

	let parsed_variable_tokens: Vec<Node> = parse(variable_tokens, Context::VariableDeclaration);

	let node = Node::Variable {
		name: variable_name.to_string(),
		constant,
		var_type: variable_type,
		value: if parsed_variable_tokens.len() > 0 { Some(parsed_variable_tokens) } else { None }
	};

	debugln!("parse_variable new node: {:?}", node);

	tree.push(node);
}