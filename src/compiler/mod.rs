mod variable;

use variable::compile_variable;

use crate::parser::Node;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Instruction {
	pub opcode: Opcode,
	pub operand: Option<String>,
	pub comment: Option<String>
}

impl Display for Instruction {
	fn fmt(&self, f: &mut Formatter) -> Result {
		todo!()
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

pub fn compile(nodes: Vec<Node>) -> Vec<Instruction> {
	let mut assembly: Vec<Instruction> = Vec::new();
	let mut addresses: HashMap<String, u8> = HashMap::new();
	let mut address: u8 = 0;

	for node in nodes {
		match node {
			Node::None => assembly.push(Instruction::new_simple(Opcode::NOP)),
			Node::Variable { ref name, ref constant, ref var_type, ref value } => {
				compile_variable(node, &mut assembly, &mut addresses, &mut address)
			},
			_ => {}
		}
	}

	return assembly;
}