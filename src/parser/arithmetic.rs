use crate::parser::{Node, Operator, Token};

pub fn parse_arithmetic(op: char, value: String, tokens: &Vec<Token>, pointer: &mut usize, tree: &mut Vec<Node>) {
	let operator: Operator = match op {
		'+' => Operator::Add,
		'-' => Operator::Subtract,
		'*' => Operator::Multiply,
		'/' => Operator::Divide,
		'%' => Operator::Modulo,
           
		_ => panic!("Invalid operator")
	};

	let left = Node::NumberLiteral(value.parse().expect("Invalid number literal"));
	let right = Node::NumberLiteral(tokens.get(*pointer + 2).unwrap().to_string().parse().expect("Invalid number literal"));

	*pointer += 3;

	if tokens.get(*pointer) != None {
		panic!("Only two numbers are allowed in an arithmetic operation")
	}

	tree.push(Node::Arithmetic {
		operator: operator,
		left: Box::new(left),
		right: Box::new(right)
	});
}