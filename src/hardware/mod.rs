#[cfg(test)]
mod tests;

pub mod instruction;
pub mod tape;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HardwareError {
    ByteInstructionToLong(instruction::ByteInstruction, u32),
    PointerOutOfBounds(u32),
    PCOutOfBound(u32),
    InvalidMathsOperation(u32, u8, i8),
    InvalidInstruction(u32, u8),
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
