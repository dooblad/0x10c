pub mod assembler;
pub mod instruction;
pub mod op;
pub mod val_type;

use std::fmt;

use self::instruction::Instruction;

const NUM_REGISTERS: u16 = 8;
const RAM_SIZE: u32 = 0x10000;  // words
const STACK_START: u16 = 0xffff;
//const WORD_SIZE: u16 = 16;  // bits
//const MAX_NUM_DEVICES: u16 = 65535;

pub mod register {
    #[derive(Debug)]
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
    pub fn try_from(token: &str) -> Option<u16> {
        // TODO: Find a reasonable way to propagate up the commented-out error messages.

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
                    Err(_) => None,
                    //Err(_) => return Err(
                    //    format!("Line {}: Failed to parse literal from \"{}\"",
                    //            line_num, token))
                };
            }
        }

        match result {
            Some(v) => {
                if v <= u16::max_value() {
                    Some(v)
                } else {
                    //Err(format!("Line {}: Literal \"{}\" too large", line_num, v))
                    None
                }
            },
            None => None,
            //None => Err(format!("Line {}: Failed to parse literal from \"{}\"",
            //                    line_num, token)),
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
    // TODO: The DCPU should never "finish" as long as it's powered on.
    last_pc: u16,
    finished: bool,
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
            // Sentinel value
            last_pc: u16::max_value(),
            finished: false,
            sp: STACK_START,
            ex: 0,
            ia: 0,
        }
    }

    /// Copies a program into the DCPU's memory.
    ///
    /// Programs are loaded starting from memory address 0x0.
    ///
    /// # Arguments
    ///
    /// * `program` - A vector of words containing instructions
    pub fn load_program(&mut self, program: &Vec<u16>) {
        for (i, word) in program.iter().enumerate() {
            self.set_mem(i as u16, *word as u16);
        }
        self.set_pc(0);
    }

    /// Runs a single cycle of the DCPU.
    pub fn tick(&mut self) {
        use self::op::OpResult::*;
        if !self.finished {
            // TODO: Wait for C cycles before actually executing fetched instruction.
            let curr_instr = Instruction::new(self.mem(self.pc()));
            self.incr_pc();
            match curr_instr.eval(self) {
                NextInstr => (),
                SkipNextInstr => {
                    let curr_pc = self.pc();
                    let instr_size = Instruction::new(self.mem(self.pc())).num_words();
                    self.set_pc(curr_pc + instr_size);
                },
            }

            if self.last_pc == self.pc() {
                self.finished = true;
            }

            self.last_pc = self.pc();
        }
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
        self.sp -= 1;
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

    pub fn finished(&self) -> bool {
        self.finished
    }
}

impl fmt::Display for Dcpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
====================
 A: {:#x}
 B: {:#x}
 C: {:#x}
 X: {:#x}
 Y: {:#x}
 Z: {:#x}
 I: {:#x}
 J: {:#x}
--------------------
 PC: {:#x}
 SP: {:#x}
 EX: {:#x}
 IA: {:#x}
====================
        ",
               self.reg(0), self.reg(1), self.reg(2), self.reg(3),
               self.reg(4), self.reg(5), self.reg(6), self.reg(7),
               self.pc(), self.sp(), self.ex(), self.ia()
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_finish() {
        let program_src = "\
SET A, 0x0
SET B, 0x1
SET C, 0x2
:finish SET PC, finish
        ";
        run_program(program_src);
    }

    #[test]
    fn simple() {
        let program_src = "\
; Test that comments work.
SET A, 0x30 ; Test that comments work on the same line as code.
SET [0x1000], 0x20
SUB A, [0x1000]
IFN A, 0x10
SET A, 0xff

:finish SET PC, finish
        ";
        run_program(program_src);
    }

    #[test]
    fn label_loop() {
        let program_src = "\
SET A, 0x10
:loop
SUB A, 1
IFN A, 0
SET PC, loop

:finish SET PC, finish
        ";
        run_program(program_src);
    }

    /// Assembles and runs the program from source and gives diagnostic information.
    fn run_program(program_src: &str) {
        let program = assembler::assemble(program_src);
        if !program.is_ok() {
            let errors = program.err().unwrap();
            println!();
            println!("[ERRORS]");
            for error in errors {
                println!("  {}", error);
            }
            println!();
            panic!();
        }
        let program = program.unwrap();
        println!("[Words]");
        print_program_words(&program);
        println!();
        println!("[Syntax]");
        print_program_syntax(&program);
        println!();

        // Now, run it.
        println!("[Execution]");
        let mut dcpu = Dcpu::new();
        dcpu.load_program(&program);
        while !dcpu.finished() {
            let instr_word = dcpu.mem(dcpu.pc());
            println!("Hex: {:#x}", instr_word);
            println!("Instruction: {}", Instruction::new(instr_word));
            println!();
            dcpu.tick();
            println!("{}", dcpu);
        }
    }

    fn print_instruction_components(instruction: u16) {
        println!("A: {}, B: {}, Op: {}",
                 format!("{:#x}", instruction >> 10),
                 format!("{:#x}", (instruction >> 5) & 0b11111),
                 format!("{:#x}", instruction & 0b11111),
        );
    }

    fn print_program_words(program: &Vec<u16>) {
        for word in program {
            println!("0x{:01$x}", word, 4);
        }
    }

    fn print_program_syntax(program: &Vec<u16>) {
        let mut iter = program.iter();
        while let Some(word) = iter.next() {
            let instr = Instruction::new(*word);
            println!("{}", instr);
            let to_skip = instr.num_words() - 1;
            for _ in 0..to_skip {
                iter.next().unwrap();
            }
        }
    }

}
