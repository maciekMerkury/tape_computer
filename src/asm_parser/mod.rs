use crate::{Word, WORD_SIZE, asm_parser, hardware};
use crate::hardware::Instruction;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    //UnexpectedNewline(usize),
    //InvalidInstructionLenght(usize),
    InvalidInstruction(String, usize),
    InvalidInstructionArgument(usize),
    InvalidInstructionArgumentCount(usize),
    InvalidInstructionInt(usize),

    InvalidConstantDeclaration(usize),
    NonAlphabeticConstantStart(usize),
    InvalidConstantName(usize),

    CannotRedeclareConstantsAndLabels(usize),

    InvalidLabelDeclaration(usize),
    NonAlphabeticLabelStart(usize),

    ConstantParseIntError(std::num::ParseIntError, usize),
    ConstantIntToLong(std::num::TryFromIntError, usize),

    UnrecognisedSymbol(usize, String),
    InvalidCommentSymbol(usize),

    ShiftLessThan3,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: actually finish this
        return Ok(write!(
            f,
            "ParseError: {}",
            match self {
                _ => "bruh",
            }
        )?);
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, PartialEq)]
struct Constant {
    identifier: String,
    content: Len,
}

#[derive(Debug, Clone, PartialEq)]
enum Len {
    Pointer(Word),
    Cell(u8),
}

#[derive(Debug, Clone, PartialEq)]
struct Label {
    identifier: String,
    location: Word,
}


#[derive(Debug, Clone, PartialEq)]
enum Token {
    Instruction(Instruction),
}

fn check_and_parse_constant(input: &[&str], line_num: usize) -> Result<Constant, ParserError> {
    // if there is a whitespace between '#' and identifier, all checks have to be shifted by 1
    use ParserError::*;
    let i = if input[0] == "#" { 1 } else { 0 };
    if input.len() != 3 + i {
        return Err(InvalidConstantDeclaration(line_num));
    }
    if !["ptr", "#ptr", "cell", "#cell"].contains(&input[i]) {
        return Err(InvalidConstantDeclaration(line_num));
    }
    for c in input[1 + i].chars() {
        if !c.is_alphanumeric() {
            return Err(InvalidConstantName(line_num));
        }
    }

    if let Some(c) = input[1 + i].chars().next() {
        if !c.is_alphabetic() {
            return Err(NonAlphabeticLabelStart(line_num));
        }
    }

    match input[2 + i].parse::<Word>() {
        Err(e) => return Err(ConstantParseIntError(e, line_num)),
        Ok(v) => {
            if ["ptr", "#ptr"].contains(&input[i]) {
                return Ok((Constant {
                    identifier: input[1 + i].into(),
                    content: Len::Pointer(v),
                }));
            }
            match u8::try_from(v) {
                Err(e) => return Err(ConstantIntToLong(e, line_num)),
                Ok(v) => {
                    return Ok((Constant {
                        identifier: input[1 + i].into(),
                        content: Len::Cell(v),
                    }))
                },
            }
        },
    }
}

fn check_and_parse_label(
    input: &str,
    line_num: usize,
    location: usize,
) -> Result<Label, ParserError> {
    use ParserError::*;

    let mut label = String::new();

    for (i, c) in input.chars().enumerate() {
        if !c.is_whitespace() {
            label = input.chars().collect::<Vec<char>>()[i..].iter().collect();
            break;
        }
    }

    label.remove_matches('.');
    if label.len() == 0 {
        return Err(InvalidLabelDeclaration(line_num));
    }

    //println!("{}", label.chars().count());
    for c in label.chars() {
        if !c.is_alphanumeric() {
            return Err(InvalidLabelDeclaration(line_num));
        }
    }

    if let Some(c) = label.chars().next() {
        if !c.is_alphabetic() {
            //println!("c: {}", c);
            return Err(NonAlphabeticLabelStart(line_num));
        }
    }

    return Ok(Label {
        location: location as Word,
        identifier: label,
    });
}

const VALID_INSTRUCTION_IDENTIFIERS: [&str; 15] = [
    "inc", "dec", "add", "mtp", "tpf", "tpb", "mpc", "mpciz", "ret", "retc", "scv", "ccv", "neg",
    "or", "and",
];

fn instruction_index(token: &str, line_num: usize) -> Result<u8, ParserError> {
    use ParserError::*;
    if !VALID_INSTRUCTION_IDENTIFIERS.contains(&token) {
        // is an invalid instruction, just get out
        return Err(InvalidInstruction(token.to_string(), line_num));
    }

    // we can unwrap, because we know the string is a valid instruction identifier
    return Ok(VALID_INSTRUCTION_IDENTIFIERS
        .iter()
        .position(|&r| r == token)
        .unwrap() as u8
        + 1);
}

fn check_and_parse_instruction(
    input: &[&str],
    line_num: usize,
    constants: &[Constant],
    labels: &[Label],
) -> Result<(usize, Instruction), ParserError> {
    //todo!("parse instruction");
    use crate::hardware::Instruction::*;
    use ParserError::*;

    let i = instruction_index(input[0], line_num)?;
    match i {
        1 | 2 | 10 | 13 => {
            if input.len() != 1 {
                return Err(InvalidInstructionArgumentCount(line_num));
            } else {
                return Ok((
                    1,
                    match i {
                        1 => Increment,
                        2 => Decrement,
                        10 => ReturnCell,
                        13 => Negate,
                        _ => unreachable!(),
                    },
                ));
            }
        },

        9 | 11 => {
            let value: u8;
            if input.len() != 2 {
                return Err(InvalidInstructionArgumentCount(line_num));
            }
            if let Some(i) = constants.iter().position(|c| c.identifier == input[1]) {
                if let Len::Cell(u) = constants[i].content {
                    value = u;
                } else {
                    return Err(InvalidInstructionInt(line_num));
                }
            } else if let Ok(u) = input[1].parse::<u8>() {
                value = u;
            } else {
                return Err(InvalidInstructionArgument(line_num));
            }
            return Ok((
                2,
                match i {
                    9 => Return(value),
                    11 => SetCellValue(value),
                    _ => unreachable!(),
                },
            ));
        },
        3..=8 | 12 | 14 | 15 => {
            //todo!();
            if input.len() != 2 {
                return Err(InvalidInstructionArgumentCount(line_num));
            }
            let value: Word;
            if let Some(i) = constants.iter().position(|c| c.identifier == input[1]) {
                //println!("constant: {:?}", constants);
                if let Len::Pointer(v) = constants[i].content {
                    value = v;
                } else {
                    return Err(InvalidInstructionInt(line_num));
                }
            } else if let Some(i) = labels.iter().position(|l| l.identifier == input[1]) {
                value = labels[i].location;
            } else if let Ok(v) = input[1].parse::<Word>() {
                value = v;
            } else {
                return Err(InvalidInstructionArgument(line_num));
            }
            return Ok((
                5,
                match i {
                    3 => Add(value),

                    4 => MoveTapePointer(value),
                    5 => ShiftTPForwards(value),
                    6 => ShiftTPBackwards(value),

                    7 => MovePC(value),
                    8 => MovePCIfZero(value),

                    12 => CopyCellValue(value),

                    14 => Or(value),
                    15 => And(value),
                    _ => unreachable!(),
                },
            ));
        },
        _ => unreachable!(),
    };

    unreachable!();
}

pub(crate) fn lines_to_instructions(lines: &[&str]) -> Result<Vec<Instruction>, ParserError> {
    use ParserError::*;
    let mut output = Vec::<Instruction>::with_capacity(lines.len());
    let mut labels = Vec::<Label>::new();
    let mut constants = Vec::<Constant>::new();
    // the length is made up
    let mut instructions = Vec::<(usize, &str)>::with_capacity(lines.len() / 2);

    let mut ptr: usize = 0;
    // parse labels and constants
    for (line_num, &line) in lines.into_iter().enumerate() {
        let toks = line.split_whitespace().collect::<Vec<&str>>();

        if toks.len() == 0 {
            continue;
        }

        if line.contains(';') {
            if !toks[0].starts_with(';') {
                return Err(InvalidCommentSymbol(line_num));
            }
        } else if toks[0].starts_with('#') {
            // declaration of a constant
            let con = check_and_parse_constant(&toks, line_num)?;
            if constants
                .iter()
                .position(|c| c.identifier == con.identifier)
                .is_some()
                | labels
                    .iter()
                    .position(|l| l.identifier == con.identifier)
                    .is_some()
            {
                return Err(CannotRedeclareConstantsAndLabels(line_num));
            }

            constants.push(con);
        } else if toks[0].starts_with('.') {
            // label
            if toks.len() != 1 {
                return Err(InvalidLabelDeclaration(line_num));
            }
            let lab = check_and_parse_label(line, line_num, ptr)?;
            if constants
                .iter()
                .position(|c| c.identifier == lab.identifier)
                .is_some()
                | labels
                    .iter()
                    .position(|l| l.identifier == lab.identifier)
                    .is_some()
            {
                return Err(CannotRedeclareConstantsAndLabels(line_num));
            }
            labels.push(lab);
        } else {
            // is an instruction
            if !VALID_INSTRUCTION_IDENTIFIERS.contains(&toks[0]) {
                println!("{}", toks[0]);
                return Err(InvalidInstruction(toks[0].to_string(), line_num));
            }
            ptr += match instruction_index(&toks[0], line_num)? {
                1 | 2 | 10 | 13 => 1,
                9 | 11 => 2,
                3..=8 | 12 | 14 | 15 => 1 + std::mem::size_of::<Word>(),
                _ => unreachable!(),
            };
            instructions.push((line_num, line));
        }
    }

    // instructions
    for (line_num, line) in instructions.into_iter() {
        let toks = line.split_whitespace().collect::<Vec<&str>>();
        let (len, inst) = check_and_parse_instruction(&toks, line_num, &constants, &labels)?;
        output.push(inst);
    }

    output.shrink_to_fit();
    return Ok(output);
}

pub fn lines_to_bytes(lines: &[&str], instruction_start_index: Word) -> Result<Vec<u8>, asm_parser::ParserError> {
    if instruction_start_index < WORD_SIZE as Word + 1 {
        return Err(asm_parser::ParserError::ShiftLessThan3);
    }

    let instructions = asm_parser::lines_to_instructions(lines)?;

    let bytes = hardware::instructions_to_bytes(&instructions);

    let offset = {
        use crate::hardware::instruction::ByteInstruction;
        match hardware::Instruction::MoveTapePointer(instruction_start_index).to_byte_instruction() {
            ByteInstruction::Big(u, arr) => {
                [u, arr[0], arr[1]]
            },
            _ => unreachable!(),
        }
    };

    let tape: Vec<u8> = Vec::from_iter(offset.into_iter().chain(vec![0u8; (instruction_start_index - (WORD_SIZE as Word + 1)) as usize].into_iter()).chain(bytes.into_iter()));

    return Ok(tape);
}
