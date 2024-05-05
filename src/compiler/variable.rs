use std::collections::HashMap;

use crate::compiler::{compile_raw, CompilerContext, Instruction, Opcode};
use crate::parser::Node;

pub fn compile_variable(node: Node, context: &mut CompilerContext) -> Vec<Instruction> {
	let mut result: Vec<Instruction> = Vec::new();
	context.address += 1;

	if let Node::Variable { ref name, ref constant, ref var_type, ref value } = node {
		context.addresses.insert(name.into(), context.address);

		// Ignore uninitialized variables
		if let Some(value) = value {
			let unboxed = value.as_ref();
			result.extend(compile_raw(unboxed.clone(), context));

			result.push(Instruction::new(
				Opcode::STA,
				Some(format!("${:x}", context.address)),
				Some(format!("{:?} = {:?}", var_type, value))
			));
		}
	} else {
		panic!("Expected Variable, found {:?}", node);
	}

	return result
}

pub fn compile_variable_reference(node: Node, context: &mut CompilerContext) -> Vec<Instruction> {
	if let Node::VariableReference ( ref name ) = node {
		let address = context.addresses.get(name).expect(format!("Variable {} not found", name).as_str());
		return vec![Instruction::new(
			Opcode::LDA,
			Some(format!("${:x}", address)),
			Some(format!("&{}", name))
		)];
	} else {
		panic!("Expected VariableReference, found {:?}", node);
	}
}