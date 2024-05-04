use std::collections::HashSet;

use nme_c::{
	compiler::compile,
	parser::{parse, Expression, Node, Type},
	tokenizer::{tokenize, Token},
};

#[test]
fn function() {
	const SOURCE: &str = "void main() {}";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("void".to_string()),
		Token::Identifier("main".to_string()),
		Token::Separator('('),
		Token::Separator(')'),
		Token::Separator('{'),
		Token::Separator('}'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Function {
			name: "main".to_string(),
			return_type: Type::Void,
			args: HashSet::new(),
			body: Vec::new(),
		}
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn function_arguments() {
	const SOURCE: &str = "void main(int a, char b) {}";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("void".to_string()),
		Token::Identifier("main".to_string()),
		Token::Separator('('),
		Token::Identifier("int".to_string()),
		Token::Identifier("a".to_string()),
		Token::Separator(','),
		Token::Identifier("char".to_string()),
		Token::Identifier("b".to_string()),
		Token::Separator(')'),
		Token::Separator('{'),
		Token::Separator('}'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Function {
			name: "main".to_string(),
			return_type: Type::Void,
			args: vec![
				("a".to_string(), Type::Int),
				("b".to_string(), Type::Char),
			].into_iter().collect(),
			body: Vec::new(),
		}
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn function_arguments_2() {
	const SOURCE: &str = "void main(int a, char b, int c) {}";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("void".to_string()),
		Token::Identifier("main".to_string()),
		Token::Separator('('),
		Token::Identifier("int".to_string()),
		Token::Identifier("a".to_string()),
		Token::Separator(','),
		Token::Identifier("char".to_string()),
		Token::Identifier("b".to_string()),
		Token::Separator(','),
		Token::Identifier("int".to_string()),
		Token::Identifier("c".to_string()),
		Token::Separator(')'),
		Token::Separator('{'),
		Token::Separator('}'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Function {
			name: "main".to_string(),
			return_type: Type::Void,
			args: vec![
				("a".to_string(), Type::Int),
				("b".to_string(), Type::Char),
				("c".to_string(), Type::Int),
			].into_iter().collect(),
			body: Vec::new(),
		}
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn function_body() {
	const SOURCE: &str = "void main() { int a = 1; }";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("void".to_string()),
		Token::Identifier("main".to_string()),
		Token::Separator('('),
		Token::Separator(')'),
		Token::Separator('{'),
		Token::Identifier("int".to_string()),
		Token::Identifier("a".to_string()),
		Token::Operator('='),
		Token::Number("1".to_string()),
		Token::Separator(';'),
		Token::Separator('}'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Function {
			name: "main".to_string(),
			return_type: Type::Void,
			args: HashSet::new(),
			body: vec![
				Node::Variable {
					name: "a".to_string(),
					constant: false,
					var_type: Type::Int,
					value: Some(vec![Node::NumberLiteral(1)]),
				}
			],
		}
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn function_body_with_arguments() {
	const SOURCE: &str = "void add(int a, int b) { int r = a + b; }";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("void".to_string()),
		Token::Identifier("add".to_string()),
		Token::Separator('('),
		Token::Identifier("int".to_string()),
		Token::Identifier("a".to_string()),
		Token::Separator(','),
		Token::Identifier("int".to_string()),
		Token::Identifier("b".to_string()),
		Token::Separator(')'),
		Token::Separator('{'),
		Token::Identifier("int".to_string()),
		Token::Identifier("r".to_string()),
		Token::Operator('='),
		Token::Identifier("a".to_string()),
		Token::Operator('+'),
		Token::Identifier("b".to_string()),
		Token::Separator(';'),
		Token::Separator('}'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Function {
			name: "add".to_string(),
			return_type: Type::Void,
			args: vec![
				("a".to_string(), Type::Int),
				("b".to_string(), Type::Int),
			].into_iter().collect(),
			body: vec![
				Node::Variable {
					name: "r".to_string(),
					constant: false,
					var_type: Type::Int,
					value: Some(vec![
						Node::Expression(Expression::Arithmetic {
							operator: nme_c::parser::Operator::Add,
							left: Box::new(Node::VariableReference("a".to_string())),
							right: Box::new(Node::VariableReference("b".to_string())),
						})
					]),
				}
			],
		}
	];

	assert_eq!(tree, ideal_tree);
}