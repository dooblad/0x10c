use std::fmt;

use super::Dcpu;
use super::op::{Op, OpResult};
use super::val_type::ValType;

pub enum ValSize {
    A,  // 6 bits
    B,  // 5 bits
}

pub struct Instruction {
    op: Op,
    a: ValType,
    b: ValType,
}

impl Instruction {
    pub fn new(word: u16) -> Instruction {
        let op_code = word & 0b11111;
        let b_val = (word >> 5) & 0b11111;
        let a_val = (word >> 10) & 0b111111;
        Instruction {
            op: Op::new(op_code),
            a: ValType::new(a_val, ValSize::A),
            b: ValType::new(b_val, ValSize::B),
        }
    }

    pub fn eval(&self, dcpu: &mut Dcpu) -> OpResult {
        // Always evaluate `b` before `a`.
        let b_val = self.b.eval(dcpu);
        let a_val = self.a.eval(dcpu);
        self.op.eval(b_val, a_val, dcpu)
    }

    pub fn num_words(&self) -> u16 {
        use self::ValType::*;

        // Must be at least 1 word long.
        let mut result = 1;
        result += match self.a {
            RegisterNextWordDeref(_) | NextWordDeref | NextWord => 1,
            _ => 0,
        };
        result += match self.b {
            RegisterNextWordDeref(_) | NextWordDeref | NextWord => 1,
            _ => 0,
        };
        result
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}, {}", self.op, self.b, self.a)
    }
}

pub fn make_instruction_bits(a_code: u16, b_code: u16, op_code: u16) -> u16 {
    (a_code << 10) | (b_code << 5) | op_code
}
