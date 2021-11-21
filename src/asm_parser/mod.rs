use crate::hardware::instruction::Instruction;
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "ParseError: {}",
            match self {
                _ => "bruh",
            }
        )?)
    }
}

impl std::error::Error for ParserError {}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Instruction(Instruction),
}


fn line_to_tokens(file: Vec<String>) -> Result<Vec<Token>, ParserError> {
    let file = file.into_iter().filter(|x| x.is_ascii() && !(x.is_empty())).collect::<Vec<String>>();
    let mut output: Vec<Token> = Vec::with_capacity(file.len());
    for line in file {
    }
    unimplemented!();

    return Ok(output.into_iter().rev().collect());
}

pub fn parse_file(file: Vec<String>) -> Result<Vec<Instruction>, ParserError> {
    unimplemented!();
}
