use crate::parser::{parse, Context, Node, Operator, Token};
use crate::parser::{
	arithmetic::parse_arithmetic,
	comparision::parse_comparision
};

pub fn parse_expression(op: char, value: Token, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	match op {
		'+' | '-' | '*' | '/' | '%' => {
			parse_arithmetic(op, value, tokens, pointer, tree);
		},
		
		'<' | '>' | '=' => {
			parse_comparision(op, value, tokens, pointer, tree);
		},
		
		_ => panic!("Invalid operator")
	}
}