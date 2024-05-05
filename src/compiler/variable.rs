use std::collections::HashMap;

use crate::compiler::{compile, CompilerContext, Instruction, Opcode};
use crate::parser::Node;

pub fn compile_variable(node: Node, context: &mut CompilerContext) -> Vec<Instruction> {
	context.address += 1;

	if let Node::Variable { ref name, ref constant, ref var_type, ref value } = node {
		context.addresses.insert(name.into(), context.address);

		if let Some(value) = value {
			compile(value.as_slice(), context);
		} else {
			context.assembly.push(Instruction::new(
				Opcode::NOP,
				None,
				Some(format!("Unitialized variable: {}", name))
			));
		}

		return vec![Instruction::new(
			Opcode::STA,
			Some(format!("${:x}", context.address)),
			Some(format!("{:?} = {:?}", var_type, value))
		)]
	} else {
		panic!("Expected Variable, found {:?}", node);
	}
}

pub fn compile_variable_reference() {}