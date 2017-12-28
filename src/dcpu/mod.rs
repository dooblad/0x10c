pub mod assembler;
pub mod instruction;
pub mod op;
pub mod val_type;

use self::instruction::Instruction;

const NUM_REGISTERS: u16 = 8;

//const WORD_SIZE: u16 = 16;  // bits
const RAM_SIZE: u32 = 0x10000;  // words

const STACK_START: u16 = 0xFFFF;

//const MAX_NUM_DEVICES: u16 = 65535;

pub mod register {
    pub enum Register {
        A = 0x0,
        B = 0x1,
        C = 0x2,
        X = 0x3,
        Y = 0x4,
        Z = 0x5,
        I = 0x6,
        J = 0x7,
    }

    // TODO: Use TryFrom trait.
    pub fn try_from(token: &str) -> Option<Register> {
        use self::Register::*;
        match token {
            "A" => Some(A),
            "B" => Some(B),
            "C" => Some(C),
            "X" => Some(X),
            "Y" => Some(Y),
            "Z" => Some(Z),
            "I" => Some(I),
            "J" => Some(J),
            _ => None,
        }
    }
}

pub mod literal {
    pub fn try_from(token: &str, line_num: usize) -> Result<u16,String> {
        // First try decimal.
        let mut result = match u16::from_str_radix(token, 10) {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        // Then try hex if decimal fails.
        if let None = result {
            if token.len() > 2 {
                // Strip off the "0x" prefix.
                result = match u16::from_str_radix(&token[2..], 16) {
                    Ok(v) => Some(v),
                    Err(_) => return Err(
                        format!("Line {}: Failed to parse literal from \"{}\"",
                                line_num, token))
                };
            }
        }

        match result {
            Some(v) => {
                if v <= u16::max_value() {
                    Ok(v)
                } else {
                    Err(format!("Line {}: Literal \"{}\" too large", line_num, v))
                }
            },
            None => Err(format!("Line {}: Failed to parse literal from \"{}\"",
                                line_num, token)),
        }
    }
}

pub struct Dcpu {
    /// CPU Registers
    reg: [u16; NUM_REGISTERS as usize],
    /// Main Memory
    mem: [u16; RAM_SIZE as usize],
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
    /// Extra/Excess
    ex: u16,
    /// Interrupt Address
    ia: u16,
    // TODO: Interrupt queue
}

impl Dcpu {
    pub fn new() -> Dcpu {
        Dcpu {
            reg: [0; NUM_REGISTERS as usize],
            mem: [0; RAM_SIZE as usize],
            pc: 0,
            sp: STACK_START,
            ex: 0,
            ia: 0,
        }
    }

    pub fn tick(&mut self) {
        // TODO: Wait for C cycles before actually executing fetched instruction.
        let curr_instr = Instruction::new(self.mem(self.pc()));
        self.incr_pc();
        curr_instr.eval(self);
    }

    pub fn reg(&self, i: u16) -> u16 {
        self.reg[i as usize]
    }

    pub fn set_reg(&mut self, i: u16, val: u16) {
        self.reg[i as usize] = val;
    }

    pub fn mem(&self, i: u16) -> u16 {
        self.mem[i as usize]
    }

    pub fn set_mem(&mut self, i: u16, val: u16) {
        self.mem[i as usize] = val;
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn incr_pc(&mut self) {
        self.pc += 1;
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }

    pub fn incr_sp(&mut self) {
        self.sp += 1;
    }

    pub fn decr_sp(&mut self) {
        self.sp += 1;
    }

    pub fn ex(&self) -> u16 {
        self.ex
    }

    pub fn set_ex(&mut self, ex: u16) {
        self.ex = ex;
    }

    pub fn ia(&self) -> u16 {
        self.ia
    }

    pub fn set_ia(&mut self, ia: u16) {
        self.ia = ia;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        // TODO: Load program into DCPU.
        let program_src = "\
; Test that comments work.
SET A, 0x30 ; Test that comments work on the same line as code.
        ";
        let program = assembler::assemble(program_src);
        println!("Program: {:?}", program);
    }
}
