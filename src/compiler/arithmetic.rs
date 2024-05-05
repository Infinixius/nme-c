use std::collections::HashMap;

use crate::compiler::{compile, compile_raw, CompilerContext, Instruction, Opcode};
use crate::parser::{Expression, Node, Operator};

pub fn compile_arithmetic(expression: Expression, context: &mut CompilerContext) -> Vec<Instruction> {
	let mut result: Vec<Instruction> = Vec::new();
	context.address += 1;

	if let Expression::Arithmetic { operator, left, right } = expression {
		let compiled_left = compile_raw(*left, context);
		let compiled_right = compile_raw(*right, context);

		let compiled_right = compiled_right
			.get(0).unwrap()
			.operand.as_ref().unwrap();

		match operator {
			Operator::Add => {
				result.extend(compiled_left.clone());

				result.push(Instruction::new_simple(Opcode::CLC));

				result.push(Instruction::new(
					Opcode::ADC,
					Some(compiled_right.into()),
					Some(format!("Addition: {:?} + {:?}", compiled_left.get(0).unwrap().operand, compiled_right))
				));
			},
			_ => panic!("Unknown operator: {:?}", operator)
		}
	} else {
		panic!("Expected Arithmetic, found {:?}", expression);
	}

	return result;
}