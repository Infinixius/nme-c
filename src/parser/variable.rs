use crate::parser::{parse, Context, Expression, Node, Token, Type};

pub fn parse_variable(identifier: &str, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	let constant: bool = last_token == Some(&Token::Identifier("const".to_string()));
	let variable_type: Type = match identifier {
		"void" => Type::Void,
		"int" => Type::Int,
		"int*" => Type::IntPointer,
		"char" => Type::Char,
		"char*" => Type::CharPointer,
		"bool" => Type::Boolean,
		_ => panic!("Invalid type")
	};

	let variable_name: &Token = next_token.expect("Unknown variable name");

	let mut variable_tokens: Vec<Token> = Vec::new();

	if *&tokens.get(*pointer + 2) == Some(&Token::Separator(';'))  {
		// Uninitialized variable
		*pointer += 2;
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

	if variable_type == Type::Int && parsed_variable_tokens.len() > 0 {
		match &parsed_variable_tokens[0] {
			Node::NumberLiteral(_) => {},
			Node::VariableReference(_) => {},
			Node::Expression (exp) => match exp {
				Expression::Arithmetic { .. } => {},
				_ => panic!("Invalid value for int variable")
			}
			_ => panic!("Invalid value for int variable")
		}
	} else if variable_type == Type::Char && parsed_variable_tokens.len() > 0 {
		match &parsed_variable_tokens[0] {
			Node::CharLiteral(_) => {},
			_ => panic!("Invalid value for char variable")
		}
	} else if variable_type == Type::Void {
		panic!("Invalid type for variable")
	} else if parsed_variable_tokens.len() > 1 {
		panic!("Variable can only have one value: {:?}", parsed_variable_tokens)
	}

	let node = Node::Variable {
		name: variable_name.to_string(),
		constant,
		var_type: variable_type,
		value: if parsed_variable_tokens.len() > 0 { Some(Box::new(parsed_variable_tokens.get(0).unwrap().clone())) } else { None }
	};

	debugln!("parse_variable new node: {:?}", node);

	tree.push(node);
}

pub fn parse_variable_assignment(identifier: &str, next_token: Option<&Token>, last_token: Option<&Token>, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	let mut variable_tokens: Vec<Token> = Vec::new();
	
	*pointer += 1;

	while *pointer < tokens.len() {
		*pointer += 1;
		let token = &tokens[*pointer];

		debugln!("parse_variable_assignment token: {:?}", token);

		if *token == Token::Separator(';') {
			break;
		} else {
			variable_tokens.push(token.clone());
		}
	}

	let parsed_variable_tokens: Vec<Node> = parse(variable_tokens, Context::VariableDeclaration);

	let node = Node::VariableAssignment {
		name: identifier.into(),
		value: parsed_variable_tokens
	};

	debugln!("parse_variable new node: {:?}", node);

	tree.push(node);
}