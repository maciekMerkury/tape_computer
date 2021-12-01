// TODO: think about whether the instruction should be counted from 1 or from 0
// the advantage of counting from 1 is that when empty memory is encountered, we know that it's
// empty
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    /// Increments the value pointed to by TP
    Increment,
    /// Decrements the value pointed to by TP
    Decrement,
    /// Adds the values pointed to by TP and u32, stores the result in TP
    Add(u32),

    /// Sets TP to u32
    MoveTapePointer(u32),
    /// Adds u32 to the TP
    ShiftTPForwards(u32),
    /// Substracts u32 from the TP
    ShiftTPBackwards(u32),

    /// Sets PC to u32
    MovePC(u32),
    /// Sets PC to u32 if value pointed to by TP is zero
    MovePCIfZero(u32),

    /// Return the u8
    Return(u8),
    /// Returns the value pointed to by TP
    ReturnCell,

    /// Sets the value pointed to by TP to u8
    SetCellValue(u8),
    /// Copies the value pointed to by TP to u32
    CopyCellValue(u32),

    /// Negates the value pointed to by TP, stores the results in TP
    Negate,
    /// Ors the values pointed to by TP and u32, stores the results in TP
    Or(u32),
    /// Ands the values pointed to by TP and u32, stores the result in TP
    And(u32),
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
                3 => Ok(Add(u32::from_le_bytes(arr))),
                4 => Ok(MoveTapePointer(u32::from_le_bytes(arr))),
                5 => Ok(ShiftTPForwards(u32::from_le_bytes(arr))),
                6 => Ok(ShiftTPBackwards(u32::from_le_bytes(arr))),
                7 => Ok(MovePC(u32::from_le_bytes(arr))),
                8 => Ok(MovePCIfZero(u32::from_le_bytes(arr))),
                11 => Ok(CopyCellValue(u32::from_le_bytes(arr))),
                14 => Ok(Or(u32::from_le_bytes(arr))),
                15 => Ok(And(u32::from_le_bytes(arr))),
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
