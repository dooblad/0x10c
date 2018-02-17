use std::fmt;

use super::Dcpu;
use super::val_type::ValKind;
use super::val_type::ValKind::*;
use super::register;

use self::BasicOp::*;
use self::SpecialOp::*;
use self::OpResult::*;


/// Tells whether to skip the next instruction or not.
pub enum OpResult {
    NextInstr,
    SkipNextInstr,
}

#[derive(Debug)]
pub enum BasicOp {
    SET,
    ADD,
    SUB,
    MUL,
    MLI,
    DIV,
    DVI,
    MOD,
    MDI,
    AND,
    BOR,
    XOR,
    SHR,
    ASR,
    SHL,
    IFB,
    IFC,
    IFE,
    IFN,
    IFG,
    IFA,
    IFL,
    IFU,
    ADX,
    SBX,
    STI,
    STD,
}

#[derive(Debug)]
pub enum SpecialOp {
    JSR,
    INT,
    IAG,
    IAS,
    RFI,
    IAQ,
    HWN,
    HWQ,
    HWI,
}


impl BasicOp {
    pub fn new(op_code: u16) -> BasicOp {
        match op_code {
            0x01 => SET,
            0x02 => ADD,
            0x03 => SUB,
            0x04 => MUL,
            0x05 => MLI,
            0x06 => DIV,
            0x07 => DVI,
            0x08 => MOD,
            0x09 => MDI,
            0x0a => AND,
            0x0b => BOR,
            0x0c => XOR,
            0x0d => SHR,
            0x0e => ASR,
            0x0f => SHL,
            0x10 => IFB,
            0x11 => IFC,
            0x12 => IFE,
            0x13 => IFN,
            0x14 => IFG,
            0x15 => IFA,
            0x16 => IFL,
            0x17 => IFU,
            0x1a => ADX,
            0x1b => SBX,
            0x1e => STI,
            0x1f => STD,
            _ => panic!("Unknown opcode \"{}\"", format!("{:#x}", op_code)),
        }
    }

    pub fn eval(&self, b_kind: ValKind, a_kind: ValKind, dcpu: &mut Dcpu) -> OpResult {
        // Once ValKind's have been extracted, evaluation order of `a` and `b` doesn't
        // matter, and they can be evaluated multiple times.
        let a_val = match a_kind {
            Literal(v) => v,
            Register(v) => dcpu.reg(v),
            ProgramCounter => dcpu.pc(),
            Deref(v) => dcpu.mem(v),
        };
        let b_val = match b_kind {
            Literal(v) => v,
            Register(v) => dcpu.reg(v),
            ProgramCounter => dcpu.pc(),
            Deref(v) => dcpu.mem(v),
        };

        match *self {
            SET => {
                self::set(b_kind, a_val, dcpu);
                NextInstr
            },
            ADD => {
                let result = b_val + a_val;
                let overflow_check = b_val as u32 + a_val as u32;
                if (result as u32) < overflow_check {
                    // There was an overflow.
                    dcpu.set_ex(0x1);
                } else {
                    dcpu.set_ex(0x0);
                }
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            SUB => {
                let result = b_val - a_val;
                let underflow_check = b_val as u32 - a_val as u32;
                if (result as u32) < underflow_check {
                    // There was an underflow.
                    dcpu.set_ex(0xffff);
                } else {
                    dcpu.set_ex(0x0);
                }
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            MUL => {
                let result = b_val * a_val;
                let ex_val = (((b_val as u32 * a_val as u32) >> 16) & 0xffff) as u16;
                dcpu.set_ex(ex_val);
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            MLI => {
                let result = (b_val as i16 * a_val as i16) as u16;
                let ex_val = (((b_val as i32 * a_val as i32) >> 16) & 0xffff) as u16;
                dcpu.set_ex(ex_val);
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            DIV => {
                if a_val == 0 {
                    dcpu.set_ex(0x0);
                    self::set(b_kind, 0, dcpu);
                } else {
                    let result = b_val / a_val;
                    let ex_val = (((b_val as u32) << 16) / a_val as u32) & 0xffff;
                    let ex_val = ex_val as u16;
                    dcpu.set_ex(ex_val);
                    self::set(b_kind, result, dcpu);
                }
                NextInstr
            },
            DVI => {
                if a_val == 0 {
                    dcpu.set_ex(0x0);
                    self::set(b_kind, 0, dcpu);
                } else {
                    let result = (b_val as i16 / a_val as i16) as u16;
                    let ex_val = (((b_val as i32) << 16) / a_val as i32) & 0xffff;
                    let ex_val = ex_val as u16;
                    dcpu.set_ex(ex_val);
                    self::set(b_kind, result, dcpu);
                }
                NextInstr
            },
            MOD => {
                if a_val == 0 {
                    self::set(b_kind, 0, dcpu);
                } else {
                    self::set(b_kind, b_val % a_val, dcpu);
                }
                NextInstr
            },
            MDI => {
                if a_val == 0 {
                    self::set(b_kind, 0, dcpu);
                } else {
                    self::set(b_kind, ((b_val as i16) % (a_val as i16)) as u16, dcpu);
                }
                NextInstr
            },
            AND => {
                self::set(b_kind, b_val & a_val, dcpu);
                NextInstr
            },
            BOR => {
                self::set(b_kind, b_val | a_val, dcpu);
                NextInstr
            },
            XOR => {
                self::set(b_kind, b_val ^ a_val, dcpu);
                NextInstr
            },
            SHR => {
                let result = b_val >> a_val;
                let ex_val = ((((b_val as u32) << 16) >> (a_val as u32)) & 0xffff) as u16;
                dcpu.set_ex(ex_val);
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            ASR => {
                let result = ((b_val as i16) >> (a_val as i16)) as u16;
                let ex_val = ((((b_val as u32) << 16) >> (a_val as u32)) & 0xffff) as u16;
                dcpu.set_ex(ex_val);
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            SHL => {
                let result = b_val << a_val;
                let ex_val = ((((b_val as u32) << (a_val as u32)) >> 16) & 0xffff) as u16;
                dcpu.set_ex(ex_val);
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            IFB => {
                if (b_val & a_val) != 0 {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            IFC => {
                if (b_val & a_val) == 0 {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            IFE => {
                if b_val == a_val {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            IFN => {
                if b_val != a_val {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            IFG => {
                if b_val > a_val {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            IFA => {
                if (b_val as i16) > (a_val as i16) {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            IFL => {
                if b_val < a_val {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            IFU => {
                if (b_val as i16) < (a_val as i16) {
                    NextInstr
                } else {
                    SkipNextInstr
                }
            },
            ADX => {
                let ex = dcpu.ex();
                let result = b_val + a_val + ex;
                let overflow_check = b_val as u32 + a_val as u32 + ex as u32;
                if (result as u32) < overflow_check {
                    // There was an overflow.
                    dcpu.set_ex(0x1);
                } else {
                    dcpu.set_ex(0x0);
                }
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            SBX => {
                let ex = dcpu.ex();
                let result = b_val - a_val + ex;
                let underflow_check = b_val as u32 - a_val as u32 + ex as u32;
                if (result as u32) < underflow_check {
                    // There was an underflow.
                    dcpu.set_ex(0xffff);
                } else {
                    dcpu.set_ex(0x0);
                }
                self::set(b_kind, result, dcpu);
                NextInstr
            },
            STI => {
                self::set(b_kind, a_val, dcpu);
                let i_reg = dcpu.reg(register::Register::I as u16);
                dcpu.set_reg(register::Register::I as u16, i_reg + 1);
                let j_reg = dcpu.reg(register::Register::J as u16);
                dcpu.set_reg(register::Register::J as u16, j_reg + 1);
                NextInstr
            },
            STD => {
                self::set(b_kind, a_val, dcpu);
                let i_reg = dcpu.reg(register::Register::I as u16);
                dcpu.set_reg(register::Register::I as u16, i_reg - 1);
                let j_reg = dcpu.reg(register::Register::J as u16);
                dcpu.set_reg(register::Register::J as u16, j_reg - 1);
                NextInstr
            },
        }
    }

    pub fn op_code(&self) -> u16 {
        match *self {
            SET => 0x01,
            ADD => 0x02,
            SUB => 0x03,
            MUL => 0x04,
            MLI => 0x05,
            DIV => 0x06,
            DVI => 0x07,
            MOD => 0x08,
            MDI => 0x09,
            AND => 0x0a,
            BOR => 0x0b,
            XOR => 0x0c,
            SHR => 0x0d,
            ASR => 0x0e,
            SHL => 0x0f,
            IFB => 0x10,
            IFC => 0x11,
            IFE => 0x12,
            IFN => 0x13,
            IFG => 0x14,
            IFA => 0x15,
            IFL => 0x16,
            IFU => 0x17,
            ADX => 0x1a,
            SBX => 0x1b,
            STI => 0x1e,
            STD => 0x1f,
        }
    }

    pub fn num_cycles(&self) -> u16 {
        match *self {
            SET => 1,
            ADD => 2,
            SUB => 2,
            MUL => 2,
            MLI => 2,
            DIV => 3,
            DVI => 3,
            MOD => 3,
            MDI => 3,
            AND => 1,
            BOR => 1,
            XOR => 1,
            SHR => 1,
            ASR => 1,
            SHL => 1,
            // Conditional instructions should take an additional cycle if the test
            // fails.
            IFB => 2,
            IFC => 2,
            IFE => 2,
            IFN => 2,
            IFG => 2,
            IFA => 2,
            IFL => 2,
            IFU => 2,
            ADX => 3,
            SBX => 3,
            STI => 2,
            STD => 2,
        }
    }

    pub fn try_from(op_name: &str) -> Option<BasicOp> {
        match op_name {
            "set" => Some(SET),
            "add" => Some(ADD),
            "sub" => Some(SUB),
            "mul" => Some(MUL),
            "mli" => Some(MLI),
            "div" => Some(DIV),
            "dvi" => Some(DVI),
            "mod" => Some(MOD),
            "mdi" => Some(MDI),
            "and" => Some(AND),
            "bor" => Some(BOR),
            "xor" => Some(XOR),
            "shr" => Some(SHR),
            "asr" => Some(ASR),
            "shl" => Some(SHL),
            "ifb" => Some(IFB),
            "ifc" => Some(IFC),
            "ife" => Some(IFE),
            "ifn" => Some(IFN),
            "ifg" => Some(IFG),
            "ifa" => Some(IFA),
            "ifl" => Some(IFL),
            "ifu" => Some(IFU),
            "adx" => Some(ADX),
            "sbx" => Some(SBX),
            "sti" => Some(STI),
            "std" => Some(STD),
            _ => None,
        }
    }
}

impl SpecialOp {
    pub fn new(op_code: u16) -> SpecialOp {
        match op_code {
            0x01 => JSR,
            0x08 => INT,
            0x09 => IAG,
            0x0a => IAS,
            0x0b => RFI,
            0x0c => IAQ,
            0x10 => HWN,
            0x11 => HWQ,
            0x12 => HWI,
            _ => panic!("Unknown special opcode \"{}\"", format!("{:#x}", op_code)),
        }
    }

    pub fn eval(&self, a_kind: ValKind, dcpu: &mut Dcpu) -> OpResult {
        use self::SpecialOp::*;
        use self::OpResult::*;
        use super::val_type::ValKind::*;

        let a_val = match a_kind {
            Literal(v) => v,
            Register(v) => dcpu.reg(v),
            ProgramCounter => dcpu.pc(),
            Deref(v) => dcpu.mem(v),
        };

        match *self {
            JSR => {
                // TODO: Jumping one instruction too far.
                dcpu.decr_sp();
                let sp = dcpu.sp();

                // At this point, the `a` operand has already been evaluated, so the PC is
                // already pointing to the next instruction.
                let curr_pc = dcpu.pc();
                dcpu.set_mem(sp, curr_pc);
                dcpu.set_pc(a_val);
                NextInstr
            },
            INT => {
                // TODO
                NextInstr
            },
            IAG => {
                self::set(a_kind, dcpu.ia(), dcpu);
                NextInstr
            },
            IAS => {
                dcpu.set_ia(a_val);
                NextInstr
            },
            RFI => {
                // TODO
                NextInstr
            },
            IAQ => {
                // TODO
                NextInstr
            },
            HWN => {
                // TODO
                self::set(a_kind, 0, dcpu);
                NextInstr
            },
            HWQ => {
                // TODO
                NextInstr
            },
            HWI => {
                // TODO
                NextInstr
            },
        }
    }

    pub fn op_code(&self) -> u16 {
        match *self {
            JSR => 0x01,
            INT => 0x08,
            IAG => 0x09,
            IAS => 0x0a,
            RFI => 0x0b,
            IAQ => 0x0c,
            HWN => 0x10,
            HWQ => 0x11,
            HWI => 0x12,
        }
    }

    pub fn num_cycles(&self) -> u16 {
        match *self {
            JSR => 3,
            INT => 4,
            IAG => 1,
            IAS => 1,
            RFI => 3,
            IAQ => 2,
            HWN => 2,
            HWQ => 4,
            HWI => 4,
        }
    }

    pub fn try_from(op_name: &str) -> Option<SpecialOp> {
        match op_name {
            "jsr" => Some(JSR),
            "int" => Some(INT),
            "iag" => Some(IAG),
            "ias" => Some(IAS),
            "rfi" => Some(RFI),
            "iaq" => Some(IAQ),
            "hwn" => Some(HWN),
            "hwq" => Some(HWQ),
            "hwi" => Some(HWI),
            _ => None,
        }
    }
}


fn set(lhs: ValKind, rhs: u16, dcpu: &mut Dcpu) {
    use self::ValKind::*;
    match lhs {
        // Silently fail when assigning to a literal.
        Literal(_) => (),
        Register(v) => dcpu.set_reg(v, rhs),
        ProgramCounter => dcpu.set_pc(rhs),
        Deref(v) => dcpu.set_mem(v, rhs),
    }
}

impl fmt::Display for BasicOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl fmt::Display for SpecialOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

