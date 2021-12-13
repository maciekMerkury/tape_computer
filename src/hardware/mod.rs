use crate::Word;

#[cfg(test)]
mod tests;

// private mods, public imports to decrease the number of modules that the library exposes and
// decrease the verbosity
pub(crate) mod instruction;
pub(crate) mod tape;

pub use instruction::{Instruction, InstructionError};
pub use tape::Tape;

#[derive(Debug, Clone, PartialEq)]
pub enum HardwareError {
    ByteInstructionToLong(instruction::ByteInstruction, Word),
    PointerOutOfBounds(Word),
    PCOutOfBound(Word),
    InvalidMathsOperation(Word, u8, i8),
    InvalidInstruction(Word, u8),
    TryFromSliceErrorWrapper,
}

impl std::fmt::Display for HardwareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Ok(write!(f, "Hardware Error: {}", self))?
        use HardwareError::*;
        let string: String = match self {
            ByteInstructionToLong(inst, ptr) => format!(
                "ByteInstructionToLong; instruction: {:?}, at: {}",
                inst, ptr
            ),
            PointerOutOfBounds(ptr) => format!("PointerOutOfBounds; pointer: {}", ptr),
            PCOutOfBound(ptr) => format!("PCOutOfBound; pointer: {}", ptr),
            InvalidMathsOperation(ptr, value, operation) => format!(
                "InvalidMathsOperation; pointer: {}, value: {}, operation: {}",
                ptr, value, operation
            ),
            InvalidInstruction(ptr, inst) => format!(
                "InvalidInstruction; pointer: {}, instruction: {}",
                ptr, inst
            ),
            TryFromSliceErrorWrapper => "A try from slice bruh moment".into(),
        };

        Ok(write!(f, "Hardware Error: {}", string))?
    }
}
impl std::error::Error for HardwareError {}

impl std::convert::From<std::array::TryFromSliceError> for HardwareError {
    fn from(other: std::array::TryFromSliceError) -> Self {
        return Self::TryFromSliceErrorWrapper;
    }
}

pub(crate) fn instructions_to_bytes(instructions: &[Instruction]) -> Vec<u8> {
    use instruction::ByteInstruction::*;
    // the vec will be at least instructions.len() long
    let mut output = Vec::<u8>::with_capacity(instructions.len());
    for inst in instructions
        .into_iter()
        .map(|ins| ins.to_byte_instruction())
    {
        match inst {
            Small(u) => output.push(u),
            Medium(u1, u2) => output.extend([u1, u2].iter()),
            Big(u, arr) => output.extend([u, arr[0], arr[1]].iter()),
        }
    }

    return output;
}
