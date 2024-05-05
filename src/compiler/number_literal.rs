use std::collections::HashMap;

use crate::compiler::{CompilerContext, Instruction, Opcode};
use crate::parser::Node;

pub fn compile_number_literal(node: Node, context: &mut CompilerContext) -> Vec<Instruction> {
	let CompilerContext { nodes, assembly, addresses, address } = context;

	if let Node::NumberLiteral(num) = node {
		return vec![Instruction::new(
			Opcode::LDA,
			Some(format!("#${:x}", num)),
			Some(format!("NumberLiteral(i32) = {}", num))
		)];
	} else {
		panic!("Expected NumberLiteral, found {:?}", node);
	}
}