use super::op;
use super::op::Op;
use super::val_type::ValType;
use super::val_type::ValType::*;

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
    b: (ValType, Option<u16>),
    a: (ValType, Option<u16>),
}

pub fn assemble(source: &str) -> Result<Vec<u16>,Vec<String>> {
    use super::instruction::make_instruction_bits;

    let mut program: Vec<u16> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let lines = source.split("\n");
    // TODO: Use this for labels.
    let mut word_index = 0;
    for (line_num, line) in lines.enumerate() {
        match process_line(line_num, line, &mut errors) {
            Some(lr) => {
                let op_code = lr.op.op_code();
                let b_code = lr.b.0.val_code();
                let a_code = lr.a.0.val_code();
                let instruction= make_instruction_bits(a_code, b_code, op_code);
                program.push(instruction);
                word_index += 1;

                // If a literal in the `b` or `a` instructions is too large, then we push
                // a separate word after the instruction to store it.  It's important
                // that we push for `b` before `a`, because `b` will be evaluated first
                // when the program is ran.
                if lr.b.1.is_some() {
                    program.push(lr.b.1.unwrap());
                    word_index += 1;
                }
                if lr.a.1.is_some() {
                    program.push(lr.a.1.unwrap());
                    word_index += 1;
                }
            },
            None => ()
        }
    }

    if errors.len() == 0 {
        Ok(program)
    } else {
        Err(errors)
    }
}

fn process_line(line_num: usize, line: &str, errors: &mut Vec<String>)
    -> Option<LineResult> {
    use self::AssemblerState::*;

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
    for token in tokens {
        match state {
            Start => {
                if token.starts_with(":") {
                    // It's a label.
                    // TODO: Process labels in a pre-pass.
                    state = OpName;
                } else {
                    match op::try_from(token) {
                        Some(op) => result_op = Some(op),
                        None => {
                            errors.push(
                                format!("Line {}: Expected an op name; got \"{}\"",
                                        line_num, token));
                            return None;
                        },
                    }
                    state = OperandB
                }
            },
            OpName => {
                // TODO: No code duping.
                match op::try_from(token) {
                    Some(op) => result_op = Some(op),
                    None => {
                        errors.push(
                            format!("Line {}: Expected an op name; got \"{}\"",
                                    line_num, token));
                        return None;
                    },
                }
                state = OperandB
            },
            OperandB => {
                let mut base_token = token;
                if token.ends_with(",") {
                    base_token = &token[..token.len()-1];
                    state = OperandA;
                } else {
                    state = Comma;
                }

                match get_val_type(base_token, line_num) {
                    Ok(vt) => result_b = Some(vt),
                    Err(e) => {
                        errors.push(e);
                        return None;
                    },
                }
            },
            Comma => {
                // TODO: Allow "SET B ,A".
                if token != "," {
                    errors.push(format!("Line {}: Expected comma; got \"{}\"",
                                        line_num, token));
                    return None;
                }
            }
            OperandA => {
                match get_val_type(token, line_num) {
                    Ok(vt) => result_a = Some(vt),
                    Err(e) => {
                        errors.push(e);
                        return None;
                    },
                }
            },
            End => {
                errors.push(format!("Line {}: Expected line to end; got \"{}\"",
                                    line_num, token));
                return None;
            },
        }
    }

    if state == End && result_op.is_some() && result_b.is_some() && result_a.is_some() {
        Some(LineResult {
            op: result_op.unwrap(),
            b: result_b.unwrap(),
            a: result_a.unwrap(),
        })
    } else {
        errors.push(format!("Line {}: Incomplete line", line_num));
        None
    }
}

// TODO: Look up Rust doc conventions.
/// If not an error, returns a tuple containing the extracted value type, and a value if
/// it's one of the "next word" variants, since none of those variants can store the value
/// in themselves.
fn get_val_type(token: &str, line_num: usize) -> Result<(ValType,Option<u16>),String> {
    let deref = token.starts_with("[") && token.ends_with("]");
    // Strip off the brackets once we know it's a dereference.
    let token = if deref { &token[1..token.len()-1] } else { token };

    let err_str;
    match get_base_type(token, line_num) {
        Ok(vt) => {
            if deref {
                match vt {
                    Register(r) => return Ok((RegisterDeref(r), None)),
                    // Literal values in the range [-1,30] will generally be too small
                    // and uncommon enough, so we don't implement literal derefs as a
                    // value type.
                    Literal(v) => return Ok((NextWordDeref, Some(v))),
                    _ => return Err(format!("Line {}: Can't dereference \"{}\"",
                                            line_num, token)),
                };
            } else {
                match vt {
                    Register(r) => return Ok((Register(r), None)),
                    Push => return Ok((Push, None)),
                    Pop => return Ok((Pop, None)),
                    Peek => return Ok((Peek, None)),
                    Pick => return Ok((Pick, None)),
                    StackPointer => return Ok((StackPointer, None)),
                    ProgramCounter => return Ok((ProgramCounter, None)),
                    Extra => return Ok((Extra, None)),
                    Literal(v) => {
                        let vi = v as i16;
                        if vi >= -1 && vi <= 30 {
                            return Ok((Literal(v), None));
                        } else {
                            return Ok((NextWord, Some(v)));
                        }
                    },
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
            return Err(
                format!("Line {}: Only one \"+\" allowed inside brackets", line_num));
        }

        if !deref {
            return Err(format!("Line {}: Must dereference addition expressions",
                               line_num));
        }

        let lhs = match get_base_type(tokens[0], line_num) {
            Ok(vt) => vt,
            Err(e) => return Err(e),
        };
        let rhs = match get_base_type(tokens[1], line_num) {
            Ok(vt) => vt,
            Err(e) => return Err(e),
        };

        match (lhs, rhs) {
            (Literal(v), Register(r)) => {
                Ok((RegisterNextWordDeref(r), Some(v)))
            },
            _ => Err(format!(
                "Line {}: Addition expressions must be of the form \"[literal+register]\"",
                line_num)),
        }
    } else {
        Err(err_str)
    }
}

/// For getting the value type of a token without any nested structure (e.g., not "a+b" or
/// "[a]").
fn get_base_type(token: &str, line_num: usize) -> Result<ValType,String> {
    use super::literal;
    use super::register;

    // TODO: Program counter.
    if let Some(reg) = register::try_from(token) {
        Ok(Register(reg as u16))
    } else if let Ok(v) = literal::try_from(token, line_num) {
        Ok(Literal(v))
    } else {
        Err(format!("Line {}: Invalid syntax \"{}\"", line_num, token))
    }
}
