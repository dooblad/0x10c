use std::fmt;

use super::Dcpu;
use super::op::{BasicOp, SpecialOp, OpResult};
use super::val_type::ValType;

use self::Instruction::*;


pub enum ValSize {
    A,  // 6 bits
    B,  // 5 bits
}

pub enum Instruction {
    Basic(BasicInstruction),
    Special(SpecialInstruction),
}

pub struct BasicInstruction {
    op: BasicOp,
    a: ValType,
    b: ValType,
}

pub struct SpecialInstruction {
    op: SpecialOp,
    a: ValType,
}

impl Instruction {
    pub fn new(word: u16) -> Instruction {
        let lower_bits = word & 0b11111;
        if lower_bits == 0 {
            let op_code = (word >> 5) & 0b11111;
            let a_val = (word >> 10) & 0b111111;
            Special(SpecialInstruction {
                op: SpecialOp::new(op_code),
                a: ValType::new(a_val, ValSize::A),
            })
        } else {
            let op_code = lower_bits;
            let b_val = (word >> 5) & 0b11111;
            let a_val = (word >> 10) & 0b111111;
            Basic(BasicInstruction {
                op: BasicOp::new(op_code),
                a: ValType::new(a_val, ValSize::A),
                b: ValType::new(b_val, ValSize::B),
            })
        }
    }

    pub fn eval(&self, dcpu: &mut Dcpu) -> OpResult {
        match *self {
            Basic(ref i) => {
                // Always evaluate `b` before `a`.
                let b_val = i.b.eval(dcpu);
                let a_val = i.a.eval(dcpu);
                i.op.eval(b_val, a_val, dcpu)
            },
            Special(ref i) => {
                let a_val = i.a.eval(dcpu);
                i.op.eval(a_val, dcpu)
            },
        }
    }

    pub fn num_words(&self) -> u16 {
        1 + match *self {
            Basic(ref i) => {
                i.a.num_words() + i.b.num_words()
            },
            Special(ref i) => {
                i.a.num_words()
            },
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Basic(ref i) => {
                write!(f, "{} {}, {}", i.op, i.b, i.a)
            },
            Special(ref i) => {
                write!(f, "{} {}", i.op, i.a)
            },
        }
    }
}

pub fn make_instruction_bits(a_code: u16, b_code: u16, op_code: u16) -> u16 {
    (a_code << 10) | (b_code << 5) | op_code
}
