mod arithmetic;
mod number_literal;
mod variable;

use arithmetic::compile_arithmetic;
use number_literal::compile_number_literal;
use variable::{compile_variable, compile_variable_reference};

use crate::parser::{Expression, Node};

#[derive(Debug, Clone)]
pub enum Opcode {
	ADC,
	AND,
	ASL,
	BBR,
	BBS,
	BCC,
	BCS,
	BEQ,
	BIT,
	BMI,
	BME,
	BPL,
	BRA,
	BRK,
	BVC,
	BVS,
	CLC,
	CLD,
	CLI,
	CLV,
	CMP,
	CPX,
	CPY,
	DEC,
	DEX,
	DEY,
	EOR,
	INC,
	INX,
	INY,
	JMP,
	JSR,
	LDA,
	LDX,
	LDY,
	LSR,
	NOP,
	ORA,
	PHA,
	PHP,
	PHX,
	PHY,
	PLA,
	PLP,
	PLX,
	PLY,
	RMB,
	ROL,
	ROR,
	RTI,
	RTS,
	SBC,
	SEC,
	SED,
	SEI,
	SMB,
	STA,
	STP,
	STX,
	STY,
	STZ,
	TAX,
	TAY,
	TRB,
	TSB,
	TSX,
	TXS,
	TYA,
	WAI
}

use std::{collections::HashMap, fmt::{Display, Formatter, Result}};
impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
	pub opcode: Opcode,
	pub operand: Option<String>,
	pub comment: Option<String>
}

impl Display for Instruction {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut output = format!("{}", self.opcode);
		if let Some(operand) = &self.operand {
			output.push_str(&format!(" {}", operand));
		}
		if let Some(comment) = &self.comment {
			// output.push_str(&format!(" ; {}", comment));
		}

		write!(f, "{}", output)
	}
}

impl Instruction {
	pub fn new(opcode: Opcode, operand: Option<String>, comment: Option<String>) -> Self {
		Self {
			opcode,
			operand,
			comment
		}
	}
	pub fn new_simple(opcode: Opcode) -> Self {
		Self {
			opcode,
			operand: None,
			comment: None
		}
	}
}

#[derive(Debug)]
pub struct CompilerContext {
	pub nodes: Vec<Node>,
	pub assembly: Vec<Instruction>,
	pub addresses: HashMap<String, u8>,
	pub address: u8
}

impl Default for CompilerContext {
	fn default() -> Self {
		Self {
			nodes: Vec::new(),
			assembly: Vec::new(),
			addresses: HashMap::new(),
			address: 0
		}
	}
}

pub fn compile(nodes: &[Node], context: &mut CompilerContext) -> Vec<Instruction> {
	for node in nodes {
		let new_node = compile_raw(node.clone(), context);
		context.assembly.extend(new_node);
	}

	return context.assembly.clone();
}

pub fn compile_raw(node: Node, context: &mut CompilerContext) -> Vec<Instruction> {
	return match node {
		Node::None => vec![Instruction::new_simple(Opcode::NOP)],
		Node::NumberLiteral(_) => compile_number_literal(node.clone(), context),
		Node::Expression(expr) => {
			match expr {
				Expression::Arithmetic {..} => compile_arithmetic(expr.clone(), context),
				Expression::Comparison {..} => todo!()
			}
		}
		Node::Variable{..} => compile_variable(node.clone(), context),
		_ => {
			panic!("Unexpected node: {:?}", node)
		}
	}
}