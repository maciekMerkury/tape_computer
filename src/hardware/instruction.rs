// TODO: think about whether the instruction should be counted from 1 or from 0
// the advantage of counting from 1 is that when empty memory is encountered, we know that it's
// empty
use crate::Word;
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    /// Increments the value pointed to by TP
    Increment,
    /// Decrements the value pointed to by TP
    Decrement,
    /// Adds the values pointed to by TP and Word, stores the result in TP
    Add(Word),

    /// Sets TP to Word
    MoveTapePointer(Word),
    /// Adds Word to the TP
    ShiftTPForwards(Word),
    /// Substracts Word from the TP
    ShiftTPBackwards(Word),

    /// Sets PC to Word
    MovePC(Word),
    /// Sets PC to Word if value pointed to by TP is zero
    MovePCIfZero(Word),

    /// Return the u8
    Return(u8),
    /// Returns the value pointed to by TP
    ReturnCell,

    /// Sets the value pointed to by TP to u8
    SetCellValue(u8),
    /// Copies the value pointed to by TP to Word
    CopyCellValue(Word),

    /// Negates the value pointed to by TP, stores the results in TP
    Negate,
    /// Ors the values pointed to by TP and Word, stores the results in TP
    Or(Word),
    /// Ands the values pointed to by TP and Word, stores the result in TP
    And(Word),
}

impl Instruction {
    pub(crate) fn to_byte_instruction(&self) -> ByteInstruction {
        return self.into();
    }
}

impl std::convert::Into<u8> for Instruction {
    fn into(self) -> u8 {
        use Instruction::*;
        return match self {
            Increment => 1,
            Decrement => 2,
            Add(_) => 3,

            MoveTapePointer(_) => 4,
            ShiftTPForwards(_) => 5,
            ShiftTPBackwards(_) => 6,

            MovePC(_) => 7,
            MovePCIfZero(_) => 8,

            Return(_) => 9,
            ReturnCell => 10,

            SetCellValue(_) => 11,
            CopyCellValue(_) => 12,

            Negate => 13,
            Or(_) => 14,
            And(_) => 15,
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
    Big(u8, [u8; crate::WORD_SIZE]),
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
                10 => Ok(ReturnCell),
                13 => Ok(Negate),
                _ => Err(InstructionError::UnknownInstruction(inst)),
            },
            Medium(inst, u) => match inst {
                9 => Ok(Return(u)),
                11 => Ok(SetCellValue(u)),
                _ => Err(InstructionError::UnknownInstruction(inst)),
            },
            Big(inst, arr) => match inst {
                3 => Ok(Add(Word::from_le_bytes(arr))),
                4 => Ok(MoveTapePointer(Word::from_le_bytes(arr))),
                5 => Ok(ShiftTPForwards(Word::from_le_bytes(arr))),
                6 => Ok(ShiftTPBackwards(Word::from_le_bytes(arr))),
                7 => Ok(MovePC(Word::from_le_bytes(arr))),
                8 => Ok(MovePCIfZero(Word::from_le_bytes(arr))),
                11 => Ok(CopyCellValue(Word::from_le_bytes(arr))),
                14 => Ok(Or(Word::from_le_bytes(arr))),
                15 => Ok(And(Word::from_le_bytes(arr))),
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
            Increment | Decrement | ReturnCell | Negate => Small(self.into()),
            Return(u) | SetCellValue(u) => Medium(self.into(), u),
            Add(v) | MoveTapePointer(v) | ShiftTPForwards(v) | ShiftTPBackwards(v) | MovePC(v)
            | MovePCIfZero(v) | CopyCellValue(v) | Or(v) | And(v) => {
                Big(self.into(), v.to_le_bytes())
            },
        };
    }
}

impl std::convert::Into<ByteInstruction> for &Instruction {
    fn into(self) -> ByteInstruction {
        use ByteInstruction::*;
        use Instruction::*;
        return match *self {
            Increment | Decrement | ReturnCell | Negate => Small((*self).into()),
            Return(u) | SetCellValue(u) => Medium((*self).into(), u),
            Add(v) | MoveTapePointer(v) | ShiftTPForwards(v) | ShiftTPBackwards(v) | MovePC(v)
            | MovePCIfZero(v) | CopyCellValue(v) | Or(v) | And(v) => {
                Big((*self).into(), v.to_le_bytes())
            },
        };
    }
}
