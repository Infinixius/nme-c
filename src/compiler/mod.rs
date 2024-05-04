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

use std::fmt::{Display, Formatter, Result};
impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Instruction {
	pub opcode: Opcode,
	pub operand: Option<i32>,
}

impl Instruction {
	pub fn new(opcode: Opcode, operand: Option<i32>) -> Instruction {
		Instruction {
			opcode,
			operand
		}
	}
}

pub fn compile(nodes: Vec<Node>) -> Vec<Instruction> {
	let mut instructions: Vec<Instruction> = Vec::new(); 

	let i = Instruction::new(Opcode::NOP, None);

	println!("{}", Opcode::NOP);

	return instructions;
}