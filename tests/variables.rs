use nme_c::{
	compiler::compile,
	parser::{parse, Node, Type},
	tokenizer::{tokenize, Token},
};

#[test]
fn integer() {
	const SOURCE: &str = "int x = 1;";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("int".into()),
		Token::Identifier("x".into()),
		Token::Operator('='),
		Token::Number("1".into()),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Variable {
			name: "x".into(),
			constant: false,
			var_type: Type::Int,
			value: Some(Box::new(Node::NumberLiteral(1)))
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn char() {
	const SOURCE: &str = "char c = 'a';";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("char".into()),
		Token::Identifier("c".into()),
		Token::Operator('='),
		Token::Char('a'),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Variable {
			name: "c".into(),
			constant: false,
			var_type: Type::Char,
			value: Some(Box::new(Node::CharLiteral('a')))
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn boolean() {
	const SOURCE: &str = "bool b = true;";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("bool".into()),
		Token::Identifier("b".into()),
		Token::Operator('='),
		Token::Identifier("true".into()),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Variable {
			name: "b".into(),
			constant: false,
			var_type: Type::Boolean,
			value: Some(Box::new(Node::BooleanLiteral(true)))
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn uninitialized_int() {
	const SOURCE: &str = "int ui;";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("int".into()),
		Token::Identifier("ui".into()),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Variable {
			name: "ui".into(),
			constant: false,
			var_type: Type::Int,
			value: None
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn uninitialized_char() {
	const SOURCE: &str = "char uc;";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("char".into()),
		Token::Identifier("uc".into()),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Variable {
			name: "uc".into(),
			constant: false,
			var_type: Type::Char,
			value: None
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn uninitialized_boolean() {
	const SOURCE: &str = "bool ub;";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("bool".into()),
		Token::Identifier("ub".into()),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Variable {
			name: "ub".into(),
			constant: false,
			var_type: Type::Boolean,
			value: None
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn constant() {
	const SOURCE: &str = "const int c = 1;";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("const".into()),
		Token::Identifier("int".into()),
		Token::Identifier("c".into()),
		Token::Operator('='),
		Token::Number("1".into()),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::Variable {
			name: "c".into(),
			constant: true,
			var_type: Type::Int,
			value: Some(Box::new(Node::NumberLiteral(1)))
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn variable_assignment() {
	const SOURCE: &str = "x = 1;";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("x".into()),
		Token::Operator('='),
		Token::Number("1".into()),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::VariableAssignment {
			name: "x".into(),
			value: vec![Node::NumberLiteral(1)]
		},
	];

	assert_eq!(tree, ideal_tree);
}

#[test]
fn char_type_missing() {
	const SOURCE: &str = "x = 'a';";

	let tokens = tokenize(SOURCE);

	let ideal_tokens = vec![
		Token::Identifier("x".into()),
		Token::Operator('='),
		Token::Char('a'),
		Token::Separator(';'),
	];

	assert_eq!(tokens, ideal_tokens);

	let tree = parse(tokens, nme_c::parser::Context::Program);

	let ideal_tree = vec![
		Node::VariableAssignment {
			name: "x".into(),
			value: vec![Node::CharLiteral('a')]
		},
	];

	assert_eq!(tree, ideal_tree);
}