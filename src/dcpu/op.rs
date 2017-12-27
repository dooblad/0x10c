use dcpu::Dcpu;
use dcpu::val_type::ValKind;

/// Tells whether to skip the next instruction or not.
pub enum OpResult {
    NextInstr,
    SkipNextInstr,
}

pub enum Op {
    // Basic Ops
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
    // Special Ops
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

impl Op {
    pub fn new(instruction_bits: u16) -> Op {
        use self::Op::*;
        // Opcodes are 5 bits long.
        let op_mask = 0b11111;
        if (instruction_bits & op_mask) == 0 {
            // Special instructions have their lower 5 bits unset.
            let op_code = (instruction_bits >> 5) & 0b11111;
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
                _ => panic!("Unknown special opcode \"{}\"", format!("{:#X}", op_code)),
            }
        } else {
            // Regular Instructions
            let op_code = instruction_bits & op_mask;
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
                0x0A => AND,
                0x0B => BOR,
                0x0C => XOR,
                0x0D => SHR,
                0x0E => ASR,
                0x0F => SHL,
                0x10 => IFB,
                0x11 => IFC,
                0x12 => IFE,
                0x13 => IFN,
                0x14 => IFG,
                0x15 => IFA,
                0x16 => IFL,
                0x17 => IFU,
                0x1A => ADX,
                0x1B => SBX,
                0x1E => STI,
                0x1F => STD,
                _ => panic!("Unknown opcode \"{}\"", format!("{:#X}", op_code)),
            }
        }
    }

    fn set(lhs: ValKind, rhs: u16, dcpu: &mut Dcpu) {
        use self::ValKind::*;
        match lhs {
            // TODO: This is supposed to silently fail.
            Literal(_) => panic!("Can't assign to literal value."),
            Register(v) => dcpu.set_reg(v, rhs),
            Deref(v) => dcpu.set_mem(v, rhs),
        }
    }

    pub fn eval(&self, b_kind: ValKind, a_kind: ValKind, dcpu: &mut Dcpu) -> OpResult {
        use self::Op::*;
        use self::OpResult::*;
        use dcpu::val_type::ValKind::*;
        use dcpu::Register;

        // Once ValKind's have been extracted, evaluation order of `a` and `b` doesn't
        // matter, and they can be evaluated multiple times.
        let a_val = match a_kind {
            Literal(v) => v,
            Register(v) => dcpu.reg(v),
            Deref(v) => dcpu.mem(v),
        };
        let b_val = match b_kind {
            Literal(v) => v,
            Register(v) => dcpu.reg(v),
            Deref(v) => dcpu.mem(v),
        };

        match *self {
            SET => {
                Self::set(b_kind, a_val, dcpu);
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
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            SUB => {
                let result = b_val - a_val;
                let underflow_check = b_val as u32 - a_val as u32;
                if (result as u32) < underflow_check {
                    // There was an underflow.
                    dcpu.set_ex(0xFFFF);
                } else {
                    dcpu.set_ex(0x0);
                }
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            MUL => {
                let result = b_val * a_val;
                let ex_val = (((b_val as u32 * a_val as u32) >> 16) & 0xFFFF) as u16;
                dcpu.set_ex(ex_val);
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            MLI => {
                let result = (b_val as i16 * a_val as i16) as u16;
                let ex_val = (((b_val as i32 * a_val as i32) >> 16) & 0xFFFF) as u16;
                dcpu.set_ex(ex_val);
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            DIV => {
                if a_val == 0 {
                    dcpu.set_ex(0x0);
                    Self::set(b_kind, 0, dcpu);
                } else {
                    let result = b_val / a_val;
                    let ex_val = (((b_val as u32) << 16) / a_val as u32) & 0xFFFF;
                    let ex_val = ex_val as u16;
                    dcpu.set_ex(ex_val);
                    Self::set(b_kind, result, dcpu);
                }
                NextInstr
            },
            DVI => {
                if a_val == 0 {
                    dcpu.set_ex(0x0);
                    Self::set(b_kind, 0, dcpu);
                } else {
                    let result = (b_val as i16 / a_val as i16) as u16;
                    let ex_val = (((b_val as i32) << 16) / a_val as i32) & 0xFFFF;
                    let ex_val = ex_val as u16;
                    dcpu.set_ex(ex_val);
                    Self::set(b_kind, result, dcpu);
                }
                NextInstr
            },
            MOD => {
                if a_val == 0 {
                    Self::set(b_kind, 0, dcpu);
                } else {
                    Self::set(b_kind, b_val % a_val, dcpu);
                }
                NextInstr
            },
            MDI => {
                if a_val == 0 {
                    Self::set(b_kind, 0, dcpu);
                } else {
                    Self::set(b_kind, ((b_val as i16) % (a_val as i16)) as u16, dcpu);
                }
                NextInstr
            },
            AND => {
                Self::set(b_kind, b_val & a_val, dcpu);
                NextInstr
            },
            BOR => {
                Self::set(b_kind, b_val | a_val, dcpu);
                NextInstr
            },
            XOR => {
                Self::set(b_kind, b_val ^ a_val, dcpu);
                NextInstr
            },
            SHR => {
                let result = b_val >> a_val;
                let ex_val = ((((b_val as u32) << 16) >> (a_val as u32)) & 0xFFFF) as u16;
                dcpu.set_ex(ex_val);
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            ASR => {
                let result = ((b_val as i16) >> (a_val as i16)) as u16;
                let ex_val = ((((b_val as u32) << 16) >> (a_val as u32)) & 0xFFFF) as u16;
                dcpu.set_ex(ex_val);
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            SHL => {
                let result = b_val << a_val;
                let ex_val = ((((b_val as u32) << (a_val as u32)) >> 16) & 0xFFFF) as u16;
                dcpu.set_ex(ex_val);
                Self::set(b_kind, result, dcpu);
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
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            SBX => {
                let ex = dcpu.ex();
                let result = b_val - a_val + ex;
                let underflow_check = b_val as u32 - a_val as u32 + ex as u32;
                if (result as u32) < underflow_check {
                    // There was an underflow.
                    dcpu.set_ex(0xFFFF);
                } else {
                    dcpu.set_ex(0x0);
                }
                Self::set(b_kind, result, dcpu);
                NextInstr
            },
            STI => {
                Self::set(b_kind, a_val, dcpu);
                let i_reg = dcpu.reg(Register::I as u16);
                dcpu.set_reg(Register::I as u16, i_reg + 1);
                let j_reg = dcpu.reg(Register::J as u16);
                dcpu.set_reg(Register::J as u16, j_reg + 1);
                NextInstr
            },
            STD => {
                Self::set(b_kind, a_val, dcpu);
                let i_reg = dcpu.reg(Register::I as u16);
                dcpu.set_reg(Register::I as u16, i_reg - 1);
                let j_reg = dcpu.reg(Register::J as u16);
                dcpu.set_reg(Register::J as u16, j_reg - 1);
                NextInstr
            },
            JSR => {
                dcpu.decr_sp();
                let sp = dcpu.sp();
                let next_instr = dcpu.pc() + 1;
                dcpu.set_mem(sp, next_instr);
                dcpu.set_pc(a_val);
                NextInstr
            },
            INT => {
                // TODO
                NextInstr
            },
            IAG => {
                Self::set(a_kind, dcpu.ia(), dcpu);
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
                Self::set(a_kind, 0, dcpu);
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

    pub fn num_cycles(&self) -> u16 {
        use self::Op::*;
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

            // Conditional instructions take one cycle longer if the test fails.
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
            JSR => 3,
            INT => 4,
            IAG => 1,
            IAS => 1,
            RFI => 3,
            IAQ => 2,
            HWN => 2,
            HWQ => 4,
            // TODO: This one is labeled 4+ in the DCPU spec for some reason.
            HWI => 4,
        }
    }
}

// TODO: Use TryFrom trait.
pub fn try_from(op_name: &str) -> Option<Op> {
    use self::Op::*;
    match op_name {
        "SET" => Some(SET),
        "ADD" => Some(ADD),
        "SUB" => Some(SUB),
        "MUL" => Some(MUL),
        "MLI" => Some(MLI),
        "DIV" => Some(DIV),
        "DVI" => Some(DVI),
        "MOD" => Some(MOD),
        "MDI" => Some(MDI),
        "AND" => Some(AND),
        "BOR" => Some(BOR),
        "XOR" => Some(XOR),
        "SHR" => Some(SHR),
        "ASR" => Some(ASR),
        "SHL" => Some(SHL),
        "IFB" => Some(IFB),
        "IFC" => Some(IFC),
        "IFE" => Some(IFE),
        "IFN" => Some(IFN),
        "IFG" => Some(IFG),
        "IFA" => Some(IFA),
        "IFL" => Some(IFL),
        "IFU" => Some(IFU),
        "ADX" => Some(ADX),
        "SBX" => Some(SBX),
        "STI" => Some(STI),
        "STD" => Some(STD),
        "JSR" => Some(JSR),
        "INT" => Some(INT),
        "IAG" => Some(IAG),
        "IAS" => Some(IAS),
        "RFI" => Some(RFI),
        "IAQ" => Some(IAQ),
        "HWN" => Some(HWN),
        "HWQ" => Some(HWQ),
        "HWI" => Some(HWI),
        _ => None,
    }
}

