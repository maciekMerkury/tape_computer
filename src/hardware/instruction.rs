// TODO: think about whether the instruction should be counted from 1 or from 0
// the advantage of counting from 1 is that when empty memory is encountered, we know that it's
// empty
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    /// Increments the value at the current pointed cell
    Increment,

    /// Decrements the value at the current pointed cell
    Decrement,

    /// Negates the pointed cell
    Negate,

    /// Adds the pointed cell to the cell at position, the result is stored in pointed cell
    Add(u32),

    /// Moves the tape pointer to the cell
    MovePointer(u32),

    /// moves the program_pointer to the cell
    JumpPC(u32),

    /// JumpPCs to program_pointer if current pointed cell is zero
    JumpPCIfZero(u32),

    /// quits the entire instruction set and returns the u8 value
    Return(u8),

    /// Returns the value at current pointed cell
    ReturnCell,

    /// Sets the value of the current pointed cell to u8 value
    SetCellValue(u8),
}

impl std::convert::Into<u8> for Instruction {
    fn into(self) -> u8 {
        use Instruction::*;
        return match self {
            Increment => 1,
            Decrement => 2,
            Negate => 3,
            Add(_) => 4,
            MovePointer(_) => 5,
            JumpPC(_) => 6,
            JumpPCIfZero(_) => 7,
            Return(_) => 8,
            ReturnCell => 9,
            SetCellValue(_) => 10,
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstructionError {
    UnknownInstruction(u8),
}

impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "Instruction Error: {}", self))?
    }
}
impl std::error::Error for InstructionError {}

// i still hate this
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteInstruction {
    Big(u8, [u8; 4]),
    Medium(u8, u8),
    Small(u8),
}

impl std::convert::TryInto<Instruction> for ByteInstruction {
    type Error = InstructionError;
    fn try_into(self) -> Result<Instruction, Self::Error> {
        use ByteInstruction::*;
        use Instruction::*;
        return match self {
            Small(inst) => match inst {
                1 => Ok(Increment),
                2 => Ok(Decrement),
                3 => Ok(Negate),
                9 => Ok(ReturnCell),
                _ => Err(InstructionError::UnknownInstruction(inst)),
            },
            Medium(inst, val) => match inst {
                8 => Ok(Return(val)),
                10 => Ok(SetCellValue(val)),
                _ => Err(InstructionError::UnknownInstruction(inst)),
            },
            Big(inst, arr) => match inst {
                4 => Ok(Add(u32::from_le_bytes(arr))),
                5 => Ok(MovePointer(u32::from_le_bytes(arr))),
                6 => Ok(JumpPC(u32::from_le_bytes(arr))),
                7 => Ok(JumpPCIfZero(u32::from_le_bytes(arr))),
                _ => Err(InstructionError::UnknownInstruction(inst)),
            },
        };
    }
}

impl std::convert::Into<ByteInstruction> for Instruction {
    fn into(self) -> ByteInstruction {
        use ByteInstruction::*;
        use Instruction::*;
        return match self {
            Increment => Small(self.into()),
            Decrement => Small(self.into()),
            Negate => Small(self.into()),
            Add(v) => Big(self.into(), v.to_le_bytes()),
            MovePointer(v) => Big(self.into(), v.to_le_bytes()),
            JumpPC(v) => Big(self.into(), v.to_le_bytes()),
            JumpPCIfZero(v) => Big(self.into(), v.to_le_bytes()),
            Return(v) => Medium(self.into(), v),
            ReturnCell => Small(self.into()),
            SetCellValue(v) => Medium(self.into(), v),
        };
    }
}
