use crate::parser::{parse, Context, Expression, Node, Operator, Token};

pub fn parse_arithmetic(op: char, value: Token, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	let operator: Operator = match op {
		'+' => Operator::Add,
		'-' => Operator::Subtract,
		'*' => Operator::Multiply,
		'/' => Operator::Divide,
		'%' => Operator::Modulo,
           
		_ => panic!("Invalid operator")
	};

	let left = parse(vec![value], Context::Arithmetic);
	let right = parse(vec![tokens[*pointer + 2].clone()], Context::Arithmetic);

	*pointer += 3;

	if tokens.get(*pointer) != None {
		panic!("Only two operands are allowed in an arithmetic operation")
	}

	let node = Expression::Arithmetic {
		operator: operator,
		left: Box::new(left[0].clone()),
		right: Box::new(right[0].clone())
	};

	debugln!("parse_arithmetic new node: {:?}", node);

	tree.push(Node::Expression(node));
}