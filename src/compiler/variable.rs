use std::collections::HashMap;

use crate::compiler::{Instruction, Opcode};
use crate::parser::Node;

pub fn compile_variable(node: Node, assembly: &mut Vec<Instruction>, addresses: &mut HashMap<String, u8>, address: &mut u8) {
	*address += 1;

	if let Node::Variable { ref name, ref constant, ref var_type, ref value } = node {
		addresses.insert(name.into(), *address);

		match value.as_ref().unwrap().as_slice() {
			[Node::NumberLiteral(ref number)] => {
				assembly.push(Instruction::new(
					Opcode::LDA,
					Some(format!("#${number:x}")), // Decimal to hexadecimal conversion
					Some(format!("{:?} {} = {:?}", var_type, name, value))
				));

				assembly.push(Instruction::new(
					Opcode::STA,
					Some(format!("${:x}", address)),
					None
				));
			},
			_ => {}
		}
	}
}