use std::collections::HashMap;

use super::op;
use super::op::Op;
use super::val_type;

/// Represents the state of the assembler for a single line.
#[derive(PartialEq)]
enum AssemblerState {
    Start,
    OpName,
    OperandB,
    Comma,
    OperandA,
    End,
}

/// Result from processing a single line of a program.
struct LineResult {
    op: Op,
    b: ValType,
    a: ValType,
}

struct AssemblerContext {
    errors: Vec<String>,
    labels: HashMap<String,u16>,
    word_index: u16,
}

/// The assembler needs its own value types that store more information than the
/// value types used by the emulator.
enum ValType {
    Register(u16),
    RegisterDeref(u16),
    RegisterNextWordDeref(u16, u16),
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
}

// TODO: Support data sections.

pub fn assemble(source: &str) -> Result<Vec<u16>,Vec<String>> {
    use super::instruction::make_instruction_bits;

    // First, we do a pre-pass to collect intermediate values, and wait until we have
    // all label names.
    let mut line_results: Vec<LineResult> = Vec::new();
    let mut context = AssemblerContext {
        errors: Vec::new(),
        labels: HashMap::new(),
        word_index: 0,
    };
    let lines = source.split("\n");
    for (line_num, line) in lines.enumerate() {
        match process_line(line_num, line, &mut context) {
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
    let mut program: Vec<u16> = Vec::new();
    for lr in line_results {
        let mut data_words: Vec<u16> = Vec::new();
        let b_val = match process_val_type(lr.b, &context.labels, &mut data_words) {
            Ok(vt) => vt,
            Err(e) => {
                context.errors.push(e);
                return Err(context.errors);
            }
        };
        let a_val = match process_val_type(lr.a, &context.labels, &mut data_words) {
            Ok(vt) => vt,
            Err(e) => {
                context.errors.push(e);
                return Err(context.errors);
            }
        };

        let op_code = lr.op.op_code();
        let b_code = b_val.val_code();
        let a_code = a_val.val_code();
        let instruction = make_instruction_bits(a_code, b_code, op_code);
        program.push(instruction);

        for word in data_words {
            program.push(word);
        }
    }

    Ok(program)
}

/// Converts from the assembler's value types into the emulator's value types, and
/// mutates the program to include any data held in the assembler's value types.
///
/// # Arguments
///
/// * `data_words` - A vector of words to push extra data into, so it can eventually be
///                  included in the instruction.
fn process_val_type(val_type: ValType, labels: &HashMap<String,u16>,
                    data_words: &mut Vec<u16>) -> Result<val_type::ValType,String> {
    use self::ValType::*;
    match val_type {
        Register(r) => Ok(val_type::ValType::Register(r)),
        RegisterDeref(r) => Ok(val_type::ValType::RegisterDeref(r)),
        RegisterNextWordDeref(r, v) => {
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
        Label(s) => {
            match labels.get(&s) {
                Some(v) => {
                    data_words.push(*v);
                    Ok(val_type::ValType::NextWord)
                },
                None => {
                    Err(format!("No label \"{}\" found", s))
                }
            }
        },
    }
}

fn process_line(line_num: usize, line: &str, context: &mut AssemblerContext)
    -> Option<LineResult> {
    use self::AssemblerState::*;
    use self::ValType::*;

    // TODO: Support special instructions.

    // Discard everything after a comment.
    let line = line.split(";").next().unwrap();

    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 0 {
        // Empty line
        return None;
    }

    let mut state = Start;
    let mut result_op = None;
    let mut result_b = None;
    let mut result_a = None;
    let mut i = 0;
    while i < tokens.len() {
        let mut token = tokens[i];
        let mut increment = true;
        match state {
            Start => {
                if token.starts_with(":") {
                    // It's a label.
                    let label = &token[1..];
                    if context.labels.contains_key(label) {
                        context.errors.push(fmt_error(
                            &format!("Label \"{}\" already exists", label),
                            line_num));
                        return None;
                    } else {
                        let word_index = context.word_index;
                        context.labels.insert(String::from(label), word_index);
                    }
                    state = OpName;
                } else {
                    // No label, so progress the state and stay on the same token.
                    state = OpName;
                    increment = false;
                }
            },
            OpName => {
                match op::try_from(token) {
                    Some(op) => result_op = Some(op),
                    None => {
                        context.errors.push(fmt_error(
                            &format!("Expected an op name; got \"{}\"", token),
                            line_num));
                        return None;
                    },
                }
                state = OperandB;
            },
            OperandB => {
                let mut ends_in_comma = false;
                if token.ends_with(",") {
                    token = &token[..token.len()-1];
                    ends_in_comma = true;
                }

                match get_val_type(token) {
                    Ok(vt) => match vt {
                        Label(_) => {
                            context.errors.push(fmt_error("Can't have labels as lvalues",
                                                          line_num));
                        }
                        _ => result_b = Some(vt),
                    } ,
                    Err(e) => {
                        context.errors.push(fmt_error(&e, line_num));
                        return None;
                    },
                }

                if ends_in_comma {
                    state = OperandA;
                } else {
                    state = Comma;
                }
            },
            Comma => {
                if token == "," {
                    state = OperandA;
                } else if token.starts_with(",") {
                    state = OperandA;
                    increment = false;
                }

                if token != "," {
                    context.errors.push(fmt_error(
                        &format!("Expected comma; got \"{}\"", token), line_num));
                    return None;
                }
            }
            OperandA => {
                if token.starts_with(",") {
                    token = &token[1..];
                }

                match get_val_type(token) {
                    Ok(vt) => result_a = Some(vt),
                    Err(e) => {
                        context.errors.push(fmt_error(&e, line_num));
                        return None;
                    },
                }

                state = End;
            },
            End => {
                context.errors.push(fmt_error(
                    &format!("Expected line to end; got \"{}\"", token), line_num));
                return None;
            },
        }

        if increment {
            i += 1;
        }
    }

    if state == End && result_op.is_some() && result_b.is_some() && result_a.is_some() {
        context.word_index += 1;
        let b = result_b.unwrap();
        let a = result_a.unwrap();
        context.word_index += match b {
            RegisterNextWordDeref(_, _) | NextWordDeref(_) | NextWord(_) | Label(_) => 1,
            _ => 0,
        };
        context.word_index += match a {
            RegisterNextWordDeref(_, _) | NextWordDeref(_) | NextWord(_) | Label(_) => 1,
            _ => 0,
        };

        Some(LineResult {
            op: result_op.unwrap(),
            b,
            a,
        })
    } else {
        if state != OpName {
            // If we're at the `OpName` state, then we've only seen a label on this line.
            context.errors.push(fmt_error("Incomplete line", line_num));
        }
        None
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
            (Literal(v), Register(r)) => Ok(RegisterNextWordDeref(r, v)),
            _ => Err(String::from(
                "Addition expressions must be of the form \"[literal+register]\"")),
        }
    } else {
        Err(err_str)
    }
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

fn fmt_error(error: &str, line_num: usize) -> String {
    format!("Line {}: {}", line_num, error)
}
