use dcpu::Dcpu;
use dcpu::instruction::ValSize;

pub enum ValType {
    Register(u16),
    RegisterDeref(u16),
    RegisterNextWordDeref(u16),
    Push,
    Pop,
    Peek,
    Pick,
    StackPointer,
    ProgramCounter,
    Extra,
    NextWordDeref,
    NextWord,
    Literal(u16),
}

pub enum ValKind {
    Literal(u16),
    Register(u16),
    Deref(u16),
}

impl ValType {
    pub fn new(val_code: u16, val_size: ValSize) -> ValType {
        use self::ValType::*;
        match val_code {
            0x00...0x07 => Register(val_code),
            0x08...0x0F => RegisterDeref(val_code - 0x8),
            0x10...0x17 => RegisterNextWordDeref(val_code - 0x10),
            0x18 => match val_size {
                ValSize::A => Pop,
                ValSize::B => Push,
            },
            0x19 => Peek,
            0x1A => Pick,
            0x1B => StackPointer,
            0x1C => ProgramCounter,
            0x1D => Extra,
            0x1E => NextWordDeref,
            0x1F => NextWord,
            0x20...0x3F => Literal(val_code - 0x21),  // Map to range [-1, 30].
            _ => panic!("Invalid value specifier \"{}\"", format!("{:#X}", val_code)),
        }
    }

    pub fn val_code(&self) -> u16 {
        use self::ValType::*;
        match *self {
            Register(r) => r,
            RegisterDeref(r) => r + 0x8,
            RegisterNextWordDeref(r) => r + 0x10,
            Push | Pop => 0x18,
            Peek => 0x19,
            Pick => 0x1A,
            StackPointer => 0x1B,
            ProgramCounter => 0x1C,
            Extra => 0x1D,
            NextWordDeref => 0x1E,
            NextWord => 0x1F,
            Literal(v) => v + 0x21,
        }
    }

    pub fn eval(&self, dcpu: &mut Dcpu) -> ValKind {
        use self::ValType::*;
        use self::ValKind;
        match *self {
            Register(r) => ValKind::Register(r),
            RegisterDeref(r) => ValKind::Deref(dcpu.reg(r)),
            RegisterNextWordDeref(r) => {
                let mem_index = dcpu.reg(r) + dcpu.mem(dcpu.pc());
                dcpu.incr_pc();
                ValKind::Deref(mem_index)
            },
            Push => {
                dcpu.decr_sp();
                ValKind::Deref(dcpu.sp())
            },
            Pop => {
                let mem_index = dcpu.sp();
                dcpu.incr_sp();
                ValKind::Deref(mem_index)
            },
            Peek => ValKind::Deref(dcpu.sp()),
            Pick => {
                let mem_index = dcpu.sp() + dcpu.mem(dcpu.pc());
                dcpu.incr_pc();
                ValKind::Deref(mem_index)
            },
            StackPointer => ValKind::Literal(dcpu.sp()),
            ProgramCounter => ValKind::Literal(dcpu.pc()),
            Extra => ValKind::Literal(dcpu.ex()),
            NextWordDeref => {
                let mem_index = dcpu.mem(dcpu.pc());
                dcpu.incr_pc();
                ValKind::Deref(mem_index)
            },
            NextWord => {
                let mem_index = dcpu.pc();
                dcpu.incr_pc();
                ValKind::Deref(mem_index)
            },
            Literal(val) => ValKind::Literal(val),
        }
    }


    pub fn num_cycles(&self) -> u32 {
        use self::ValType::*;
        match *self {
            Register(_) => 0,
            RegisterDeref(_) => 0,
            RegisterNextWordDeref(_) => 1,
            Push => 0,
            Pop => 0,
            Peek => 0,
            Pick => 1,
            StackPointer => 0,
            ProgramCounter => 0,
            Extra => 0,
            NextWordDeref => 1,
            NextWord => 1,
            Literal(_) => 0,
        }
    }
}
