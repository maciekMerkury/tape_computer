use crate::hardware::Instruction;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedNewline(usize),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Variable {
    start: u32,
    length: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Instruction(Instruction),
    Label(u32),
    Variable(Variable),
}


fn lines_to_token_unvalidated_stream(lines: &[String]) -> Result<Vec<Token>, ParserError> {
    let mut output = Vec::<Token>::with_capacity(lines.len());

    'lines: for (line_num, line) in lines.into_iter().enumerate() {
        'tokens: for t in line.chars() {
            if t == '\n' {
                return Err(ParserError::UnexpectedNewline(line_num));
            }

            if t == ' ' || t == '\t' {
                continue 'tokens;
            }

            if t == '#' {
                continue 'lines;
            }
        }
    }

    output.shrink_to_fit();
    unimplemented!();
    return (Ok(output));
}

fn validate_token_stream(tokens: Vec<Token>) -> Result<Vec<Token>, ParserError> {
    unimplemented!();
}

fn token_stream_to_instructions(tokens: Vec<Token>) -> Result<Vec<Instruction>, ParserError> {
    unimplemented!();
}

pub fn lines_to_instructions(lines: &[String]) -> Result<Vec<Instruction>, ParserError> {
    let tokens = lines_to_token_unvalidated_stream(lines)?;
    let tokens = validate_token_stream(tokens)?;
    return token_stream_to_instructions(tokens);
}
