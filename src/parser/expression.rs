use crate::parser::{parse, Context, Node, Operator, Token};

pub fn parse_expression(op: char, value: Token, tokens: &[Token], pointer: &mut usize, tree: &mut Vec<Node>) {
	println!("op: {}, value: {:?}, token: {:?}", op, value, &tokens.get(*pointer));
}