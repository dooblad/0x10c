use std::collections::HashMap;
use std::str;

use super::op::{BasicOp, SpecialOp};
use super::val_type;


/// Represents the state of the assembler for a single instruction line.
#[derive(PartialEq)]
enum BasicInstructionParseState {
    OpName,
    OperandB,
    Comma,
    OperandA,
    End,
}

#[derive(PartialEq)]
enum SpecialInstructionParseState {
    OpName,
    OperandA,
    End,
}

/// Result from processing a single line of a program.
enum LineResult {
    BasicInstruction(BasicInstructionComponents),
    SpecialInstruction(SpecialInstructionComponents),
    Data(Vec<u16>),
}

/// Intermediate data used to create a basic instruction.
struct BasicInstructionComponents {
    op: BasicOp,
    b: ValType,
    a: ValType,
}

/// Intermediate data used to create a special instruction.
struct SpecialInstructionComponents {
    op: SpecialOp,
    a: ValType,
}

/// The assembler needs its own value types that store more information than the
/// value types used by the emulator.
enum ValType {
    Register(u16),
    RegisterDeref(u16),
    NextWordRegisterDeref(u16, u16),
    Push,
    Pop,
    Peek,
    Pick,
    StackPointer,
    ProgramCounter,
    Extra,
    NextWordDeref(u16),
    NextWord(u16),
    Literal(u16),
    Label(String),
    LabelDeref(String),
    LabelRegisterDeref(String, u16),
    LabelNextWordDeref(String, u16),
}

struct AssemblerContext {
    errors: Vec<String>,
    labels: HashMap<String,u16>,
    word_index: u16,
    program: Vec<u16>,
}

impl AssemblerContext {
    fn new() -> AssemblerContext {
        AssemblerContext {
            errors: Vec::new(),
            labels: HashMap::new(),
            word_index: 0,
            program: Vec::new(),
        }
    }

    fn finish(self) -> Result<Vec<u16>, Vec<String>> {
        if self.errors.len() == 0 {
            Ok(self.program)
        } else {
            Err(self.errors)
        }
    }

    fn append_special_instruction(&mut self, instr: SpecialInstructionComponents) -> bool {
        use super::instruction::make_instruction_bits;

        let mut data_words: Vec<u16> = Vec::new();
        let a_val = match self.process_val_type(instr.a, &mut data_words) {
            Ok(vt) => vt,
            Err(e) => {
                self.errors.push(e);
                return false;
            }
        };

        let op_code = instr.op.op_code();
        let a_code = a_val.val_code();
        // Special instructions have their lower 5 bits unset.
        // TODO: This is using `op_code` in the place of the `b_code` argument.  Hack!
        let instruction = make_instruction_bits(a_code, op_code, 0x0);
        self.program.push(instruction);

        for word in data_words {
            self.program.push(word);
        }

        true
    }

    fn append_normal_instruction(&mut self, instr: BasicInstructionComponents) -> bool {
        use super::instruction::make_instruction_bits;

        let mut data_words: Vec<u16> = Vec::new();
        let b_val = match self.process_val_type(instr.b, &mut data_words) {
            Ok(vt) => vt,
            Err(e) => {
                self.errors.push(e);
                return false;
            }
        };
        let a_val = match self.process_val_type(instr.a, &mut data_words) {
            Ok(vt) => vt,
            Err(e) => {
                self.errors.push(e);
                return false;
            }
        };

        let op_code = instr.op.op_code();
        let b_code = b_val.val_code();
        let a_code = a_val.val_code();
        let instruction = make_instruction_bits(a_code, b_code, op_code);
        self.program.push(instruction);

        for word in data_words {
            self.program.push(word);
        }

        true
    }

    /// Converts from the assembler's value types into the emulator's value types, and
    /// mutates the program to include any data held in the assembler's value types.
    ///
    /// # Arguments
    ///
    /// * `data_words` - A vector of words to push extra data into, so it can eventually be
    ///                  included in the instruction.
    fn process_val_type(&self, val_type: ValType, data_words: &mut Vec<u16>)
        -> Result<val_type::ValType,String> {
        use self::ValType::*;
        match val_type {
            Register(r) => Ok(val_type::ValType::Register(r)),
            RegisterDeref(r) => Ok(val_type::ValType::RegisterDeref(r)),
            NextWordRegisterDeref(v, r) => {
                data_words.push(v);
                Ok(val_type::ValType::RegisterNextWordDeref(r))
            },
            Push => Ok(val_type::ValType::Push),
            Pop => Ok(val_type::ValType::Pop),
            Peek => Ok(val_type::ValType::Peek),
            Pick => Ok(val_type::ValType::Pick),
            StackPointer => Ok(val_type::ValType::StackPointer),
            ProgramCounter => Ok(val_type::ValType::ProgramCounter),
            Extra => Ok(val_type::ValType::Extra),
            NextWordDeref(v) => {
                data_words.push(v);
                Ok(val_type::ValType::NextWordDeref)
            },
            NextWord(v) => {
                data_words.push(v);
                Ok(val_type::ValType::NextWord)
            },
            Literal(v) => Ok(val_type::ValType::Literal(v)),
            // TODO: Get rid of label lookup duplication.
            Label(s) => {
                match self.labels.get(&s) {
                    Some(v) => {
                        data_words.push(*v);
                        Ok(val_type::ValType::NextWord)
                    },
                    None => {
                        Err(format!("No label \"{}\" found", s))
                    }
                }
            },
            LabelDeref(s) => {
                match self.labels.get(&s) {
                    Some(v) => {
                        data_words.push(*v);
                        Ok(val_type::ValType::NextWordDeref)
                    },
                    None => {
                        Err(format!("No label \"{}\" found", s))
                    }
                }
            },
            LabelRegisterDeref(s, r) => {
                match self.labels.get(&s) {
                    Some(v) => {
                        data_words.push(*v);
                        Ok(val_type::ValType::RegisterNextWordDeref(r))
                    },
                    None => {
                        Err(format!("No label \"{}\" found", s))
                    }
                }
            },
            LabelNextWordDeref(s, v) => {
                match self.labels.get(&s) {
                    Some(x) => {
                        data_words.push((*x) + v);
                        Ok(val_type::ValType::NextWordDeref)
                    },
                    None => {
                        Err(format!("No label \"{}\" found", s))
                    }
                }
            }
        }
    }

    fn process_line(&mut self, line_num: usize, line: &str) -> Option<LineResult> {
        // Discard everything after a comment.
        let line = line.split(";").next().unwrap();

        let tokens: Vec<&str> = tokenize(line);
        if tokens.len() == 0 {
            // Empty line
            return None;
        }

        // Check if the line begins with a label.
        let tokens = if tokens[0].starts_with(":") {
            // If it does, process it and remove it from `tokens`.
            let label = &tokens[0][1..];
            if self.labels.contains_key(label) {
                self.errors.push(fmt_error(
                    &format!("Label \"{}\" already exists", label),
                    line_num));
                return None;
            } else {
                let word_index = self.word_index;
                self.labels.insert(String::from(label), word_index);
            }
            &tokens[1..]
        } else {
            &tokens[..]
        };

        if tokens.len() == 0 {
            // Empty line
            return None;
        }

        if tokens[0] == "DAT" {
            // Data lines have different syntax than other instructions.
            match self.process_data_line(tokens, line_num) {
                Some(d) => Some(LineResult::Data(d)),
                None => None,
            }
        } else if tokens[0] == "JSR" {
            // TODO: Support *all* special instructions.
            match self.process_special_line(tokens, line_num) {
                Some(ic) => Some(LineResult::SpecialInstruction(ic)),
                None => None,
            }
        } else {
            // Line contains a basic instruction.
            match self.process_basic_line(tokens, line_num) {
                Some(ic) => Some(LineResult::BasicInstruction(ic)),
                None => None,
            }
        }
    }

    fn process_data_line(&mut self, tokens: &[&str], line_num: usize)
        -> Option<Vec<u16>> {
        use super::literal;

        let line_rest = &tokens[1..].join("");
        let data_tokens: Vec<&str> = line_rest.split_terminator(',').collect();
        let mut data: Vec<u16> = Vec::new();
        for token in data_tokens {
            if let Some(v) = literal::try_from(token) {
                data.push(v);
                self.word_index += 1
            } else if token.starts_with('"') && token.ends_with('"'){
                // Strip off quotes.
                let token = &token[1..token.len()-1];
                let bytes = token.as_bytes();
                for &byte in bytes {
                    if byte < 0x20 || byte > 0x7e {
                        self.errors.push(fmt_error(
                            &format!("Byte {} out of range for string literal", byte),
                            line_num));
                        return None;
                    }
                    data.push(byte as u16);
                    self.word_index += 1
                }
            } else {
                self.errors.push(fmt_error(
                    &format!("Unrecognized data format \"{}\"", token),
                    line_num));
                return None;
            }
        }
        Some(data)
    }

    fn process_special_line(&mut self, tokens: &[&str], line_num: usize)
                          -> Option<SpecialInstructionComponents> {
        use self::SpecialInstructionParseState::*;

        let mut state = OpName;
        let mut result_op = None;
        let mut result_a = None;
        for &token in tokens {
            match state {
                OpName => {
                    match SpecialOp::try_from(token) {
                        Some(op) => result_op = Some(op),
                        None => {
                            self.errors.push(fmt_error(
                                &format!("Expected an op name; got \"{}\"", token),
                                line_num));
                            return None;
                        },
                    }
                    state = OperandA;
                },
                OperandA => {
                    match get_val_type(token) {
                        Ok(vt) => result_a = Some(vt),
                        Err(e) => {
                            self.errors.push(fmt_error(&e, line_num));
                            return None;
                        },
                    }

                    state = End;
                },
                End => {
                    self.errors.push(fmt_error(
                        &format!("Expected line to end; got \"{}\"", token), line_num));
                    return None;
                },
            }
        }

        if state == End && result_op.is_some() && result_a.is_some() {
            self.word_index += 1;
            let a = result_a.unwrap();

            self.word_index += a.num_words();

            Some(SpecialInstructionComponents {
                op: result_op.unwrap(),
                a,
            })
        } else {
            // If we're at the `OpName` state, then we've only seen a label on this line,
            // which is fine.  Otherwise, the line is incomplete.
            if state != OpName {
                self.errors.push(fmt_error("Incomplete line", line_num));
            }
            None
        }
    }

    fn process_basic_line(&mut self, tokens: &[&str], line_num: usize)
                          -> Option<BasicInstructionComponents> {
        use self::BasicInstructionParseState::*;
        use self::ValType::*;

        let mut state = OpName;
        let mut result_op = None;
        let mut result_b = None;
        let mut result_a = None;
        for &token in tokens {
            match state {
                OpName => {
                    match BasicOp::try_from(token) {
                        Some(op) => result_op = Some(op),
                        None => {
                            self.errors.push(fmt_error(
                                &format!("Expected an op name; got \"{}\"", token),
                                line_num));
                            return None;
                        },
                    }
                    state = OperandB;
                },
                OperandB => {
                    match get_val_type(token) {
                        Ok(vt) => match vt {
                            Label(_) => {
                                self.errors.push(fmt_error("Can't have labels as lvalues",
                                                           line_num));
                            }
                            _ => result_b = Some(vt),
                        } ,
                        Err(e) => {
                            self.errors.push(fmt_error(&e, line_num));
                            return None;
                        },
                    }
                    state = Comma;
                },
                Comma => {
                    if token == "," {
                        state = OperandA;
                    } else {
                        self.errors.push(fmt_error(
                            &format!("Expected comma; got \"{}\"", token), line_num));
                        return None;
                    }
                }
                OperandA => {
                    match get_val_type(token) {
                        Ok(vt) => result_a = Some(vt),
                        Err(e) => {
                            self.errors.push(fmt_error(&e, line_num));
                            return None;
                        },
                    }

                    state = End;
                },
                End => {
                    self.errors.push(fmt_error(
                        &format!("Expected line to end; got \"{}\"", token), line_num));
                    return None;
                },
            }
        }

        if state == End && result_op.is_some() && result_b.is_some() &&
            result_a.is_some() {
            self.word_index += 1;
            let b = result_b.unwrap();
            let a = result_a.unwrap();
            self.word_index += b.num_words();
            self.word_index += a.num_words();

            Some(BasicInstructionComponents {
                op: result_op.unwrap(),
                b,
                a,
            })
        } else {
            // If we're at the `OpName` state, then we've only seen a label on this line,
            // which is fine.  Otherwise, the line is incomplete.
            if state != OpName {
                self.errors.push(fmt_error("Incomplete line", line_num));
            }
            None
        }
    }
}

impl ValType {
    /// How many words does this value type extend beyond the first word.
    pub fn num_words(&self) -> u16 {
        use self::ValType::*;
        match *self {
            NextWordRegisterDeref(_, _) | NextWordDeref(_) | NextWord(_) |
            Label(_) | LabelDeref(_) | LabelRegisterDeref(_, _) |
            LabelNextWordDeref(_, _) => 1,
            _ => 0,
        }
    }
}


/// If not an error, returns a tuple containing the extracted value type, and a value if
/// it's one of the "next word" variants, since none of those variants can store the value
/// in themselves.
fn get_val_type(token: &str) -> Result<ValType,String> {
    use self::ValType::*;

    let deref = token.starts_with("[") && token.ends_with("]");
    // Strip off the brackets once we know it's a dereference.
    let token = if deref { &token[1..token.len()-1] } else { token };

    let err_str;
    match get_base_type(token) {
        Ok(vt) => {
            if deref {
                match vt {
                    Register(r) => return Ok(RegisterDeref(r)),
                    // Literal values in the range [-1,30] will generally be too small
                    // and uncommon enough, so we don't implement literal derefs as a
                    // value type.
                    Literal(v) => return Ok(NextWordDeref(v)),
                    Label(s) => return Ok(LabelDeref(s)),
                    _ => return Err(format!("Can't dereference \"{}\"", token)),
                };
            } else {
                match vt {
                    Register(r) => return Ok(Register(r)),
                    Push => return Ok(Push),
                    Pop => return Ok(Pop),
                    Peek => return Ok(Peek),
                    Pick => return Ok(Pick),
                    StackPointer => return Ok(StackPointer),
                    ProgramCounter => return Ok(ProgramCounter),
                    Extra => return Ok(Extra),
                    Literal(v) => {
                        let vi = v as i16;
                        if vi >= -1 && vi <= 30 {
                            return Ok(Literal(v));
                        } else {
                            return Ok(NextWord(v));
                        }
                    },
                    Label(s) => return Ok(Label(s)),
                    _ => panic!("(Supposedly) impossible branch reached"),
                };
            }
        },
        Err(e) => {
            err_str = e;
        },
    };

    // If it didn't parse correctly as a base type, then it must be an addition
    // expression (assuming it's well-formed).
    if token.contains("+") {
        let tokens: Vec<&str> = token.split("+").collect();
        if tokens.len() > 2 {
            return Err(String::from(
                "Only one \"+\" allowed inside brackets"));
        }

        if !deref {
            return Err(String::from(
                "Must dereference addition expressions"));
        }

        let lhs = match get_base_type(tokens[0]) {
            Ok(vt) => vt,
            Err(e) => return Err(e),
        };
        let rhs = match get_base_type(tokens[1]) {
            Ok(vt) => vt,
            Err(e) => return Err(e),
        };

        match (lhs, rhs) {
            (Literal(v), Register(r)) | (Register(r), Literal(v)) =>
                Ok(NextWordRegisterDeref(v, r)),
            (Label(s), Register(r)) | (Register(r), Label(s)) =>
                Ok(LabelRegisterDeref(s, r)),
            (Label(s), Literal(v)) | (Literal(v), Label(s)) =>
                Ok(LabelNextWordDeref(s, v)),
            _ => Err(format!("Improperly formatted addition expression \"{}\"", token)),
        }
    } else {
        Err(err_str)
    }
}


pub fn assemble(source: &str) -> Result<Vec<u16>, Vec<String>> {
    // First, we do a pre-pass to collect intermediate values, and wait until we have
    // all label names.
    let mut line_results: Vec<LineResult> = Vec::new();
    let mut context = AssemblerContext::new();
    let lines = source.split("\n");
    for (line_num, line) in lines.enumerate() {
        match context.process_line(line_num, line) {
            Some(lr) => {
                line_results.push(lr);
            },
            None => ()
        }
    }

    if context.errors.len() != 0 {
        return Err(context.errors);
    }

    // Now, we generate the program words.
    for lr in line_results {
        use self::LineResult::*;
        match lr {
            BasicInstruction(instr) => {
                if !context.append_normal_instruction(instr) {
                    return context.finish();
                };
            },
            SpecialInstruction(instr) => {
                if !context.append_special_instruction(instr) {
                    return context.finish();
                };
            },
            Data(d) => {
                for word in d {
                    context.program.push(word);
                }
            },
        };
    }

    context.finish()
}


/// Returns the value type of a token without any nested structure (e.g., not "a+b" or
/// "[a]").
fn get_base_type(token: &str) -> Result<ValType,String> {
    use super::literal;
    use super::register;
    use self::ValType::*;

    if let Some(reg) = register::try_from(token) {
        Ok(Register(reg as u16))
    } else if let Some(v) = literal::try_from(token) {
        Ok(Literal(v))
    } else if token == "PC" {
        Ok(ProgramCounter)
    } else if token == "SP" {
        Ok(StackPointer)
    } else if token == "EX" {
        Ok(Extra)
    } else if token == "PUSH" {
        Ok(Push)
    } else if token == "POP" {
        Ok(Pop)
    } else if token == "PEEK" {
        Ok(Peek)
    } else if token == "PICK" {
        Ok(Pick)
    } else if let None = token.find("+") {
        Ok(Label(String::from(token)))
    } else {
        Err(format!("Invalid syntax \"{}\"", token))
    }
}


/// Decomposes `line` into tokens for the assembler.
fn tokenize(line: &str) -> Vec<&str> {
    // TODO: Escape quotes.
    let mut tokens: Vec<&str> = Vec::new();
    let mut in_quotes = false;
    let mut token_start: Option<usize> = None;
    let s = line.as_bytes();
    for i in 0..s.len() {
        match token_start {
            Some(j) => {
                if in_quotes {
                    if s[i] == '"' as u8 {
                        in_quotes = false;
                        tokens.push(str::from_utf8(&s[j..i+1]).unwrap());
                        token_start = None;
                    }
                } else {
                    if s[i] == ' ' as u8 {
                        tokens.push(str::from_utf8(&s[j..i]).unwrap());
                        token_start = None;
                    } else if s[i] == ',' as u8 {
                        // Unquoted commas represent a single token.
                        tokens.push(str::from_utf8(&s[j..i]).unwrap());
                        tokens.push(str::from_utf8(&s[i..i+1]).unwrap());
                        token_start = None;
                    }
                }
            },
            None => {
                if s[i] == ',' as u8 {
                    // Unquoted commas represent a single token.
                    tokens.push(str::from_utf8(&s[i..i+1]).unwrap());
                } else if s[i] != ' ' as u8 {
                    token_start = Some(i);
                    if s[i] == '"' as u8 {
                        in_quotes = true;
                    }
                }
            }
        }
    }

    match token_start {
        Some(j) => {
            tokens.push(str::from_utf8(&s[j..s.len()]).unwrap());
        },
        None => (),
    }
    tokens
}


fn fmt_error(error: &str, line_num: usize) -> String {
    format!("Line {}: {}", line_num, error)
}
