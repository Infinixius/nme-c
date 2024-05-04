use crate::parser::{parse, Context, Node, Operator, Token};

pub fn parse_comparision(op: char, value: Token, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	let operator: Operator = match op {
		
		_ => panic!("Invalid operator")
	};

	let left = parse(vec![value], Context::Arithmetic);
	let right = parse(vec![tokens[*pointer + 2].clone()], Context::Arithmetic);

	*pointer += 3;

	if tokens.get(*pointer) != None {
		panic!("Only two operands are allowed in an arithmetic operation")
	}

	let node = Node::Arithmetic {
		operator: operator,
		left: Box::new(left[0].clone()),
		right: Box::new(right[0].clone())
	};

	debugln!("parse_arithmetic new node: {:?}", node);

	tree.push(node);
}