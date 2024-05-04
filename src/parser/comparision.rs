use crate::parser::{parse, Context, Expression, Node, Operator, Token};

pub fn parse_comparision(op: char, value: Token, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	let operator: Operator = match op {
		'<' => Operator::LessThan,
		'>' => Operator::GreaterThan,
		'=' => Operator::Equals,
		_ => panic!("Invalid operator")
	};

	let left = parse(vec![value], Context::Comparision);
	let right = parse(vec![tokens[*pointer + 2].clone()], Context::Comparision);

	*pointer += 3;

	if tokens.get(*pointer) != None {
		panic!("Only two operands are allowed in an comparision operation")
	}

	let node = Expression::Arithmetic {
		operator: operator,
		left: Box::new(left[0].clone()),
		right: Box::new(right[0].clone())
	};

	debugln!("parse_arithmetic new node: {:?}", node);

	tree.push(Node::Expression(node));
}