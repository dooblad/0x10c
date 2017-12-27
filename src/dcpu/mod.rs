pub mod instruction;
pub mod op;
pub mod val_type;

use self::instruction::Instruction;

const NUM_REGISTERS: u16 = 8;

//const WORD_SIZE: u16 = 16;  // bits
const RAM_SIZE: u32 = 0x10000;  // words

const STACK_START: u16 = 0xFFFF;

//const MAX_NUM_DEVICES: u16 = 65535;

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
        // Literal 28
        let a_val = 0x3D;
        // Register A
        let b_val = 0x0;
        // SET operation
        let op_code = 0x01;

        Instruction::new(make_instruction_bits(a_val, b_val, op_code));
    }

    fn make_instruction_bits(a_val: u16, b_val: u16, op_code: u16) -> u16 {
        (a_val << 10) | (b_val << 5) | op_code
    }
}
