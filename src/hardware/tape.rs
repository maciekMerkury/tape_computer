#[derive(Debug, Clone, Default)]
pub struct Tape {
    pub(super) tape: Vec<u8>,
    pub(crate) pointer: u32,
    pub(crate) program_counter: u32,
}

impl Tape {
    pub fn new(size: u32) -> Self {
        Self {
            tape: vec![0; size as usize],
            pointer: 0,
            program_counter: 0,
        }
    }

    pub fn len(&self) -> u32 {
        return self.tape.len() as u32;
    }

    pub fn pointed_value(&self) -> u8 {
        return self[self.pointer];
    }

    // this is so increadibly shit
    // I have no fucking idea why this is not working
    // This should absolutely work without the copy
    pub fn pointed_value_mut(&mut self) -> &mut u8 {
        // why
        let p = self.pointer;
        return &mut self[p];
    }

    pub fn pointed_program(&self) -> u8 {
        return self[self.program_counter];
    }

    pub fn insert_instruction(
        &mut self,
        inst: super::instruction::ByteInstruction,
        start: u32,
    ) -> Result<u32, super::HardwareError> {
        use super::instruction::ByteInstruction::*;
        use super::HardwareError;
        println!("instruction: {:?}", inst);
        let max = self.len() - 1;
        match inst {
            Big(ins, arr) => {
                if !(start + 4 <= max) {
                    return Err(HardwareError::ByteInstructionToLong(inst, start));
                }

                self[start] = ins;
                for (i, a) in arr.into_iter().enumerate() {
                    self[start + i as u32 + 1] = a;
                }

                return Ok(start + 5);
            },
            Medium(ins, val) => {
                if !(start + 1 <= max) {
                    return Err(HardwareError::ByteInstructionToLong(inst, start));
                }

                self[start] = ins;
                self[start + 1] = val;

                return Ok(start + 2);
            },
            Small(ins) => {
                if !(start <= max) {
                    return Err(HardwareError::ByteInstructionToLong(inst, start));
                }

                self[start] = ins;

                return Ok(start + 1);
            },
        };
    }

    // please never read this function
    // it is seriously ugly
    fn execute_instruction(
        &mut self,
        inst: super::instruction::Instruction,
    ) -> Result<Option<u8>, super::HardwareError> {
        use super::instruction::Instruction::*;
        use super::HardwareError;

        if self.pointer >= self.len() {
            return Err(HardwareError::PointerOutOfBounds(self.pointer));
        }
        if self.program_counter >= self.len() {
            return Err(HardwareError::PCOutOfBound(self.program_counter));
        }

        match inst {
            Increment => {
                if self.pointed_value() == 255 {
                    return Err(HardwareError::InvalidMathsOperation(
                        self.pointer,
                        self.pointed_value(),
                        1,
                    ));
                }
                *self.pointed_value_mut() += 1;
            },
            Decrement => {
                if self.pointed_value() == 0 {
                    return Err(HardwareError::InvalidMathsOperation(
                        self.pointer,
                        self.pointed_value(),
                        -1,
                    ));
                }
                *self.pointed_value_mut() -= 1;
            },
            Negate => {
                *self.pointed_value_mut() != self.pointed_value();
            },
            Add(ptr) => {
                if !(ptr < self.len()) {
                    return Err(HardwareError::PointerOutOfBounds(ptr));
                }
                *self.pointed_value_mut() += self[ptr];
            },
            MovePointer(ptr) => {
                if !(ptr < self.len()) {
                    return Err(HardwareError::PointerOutOfBounds(ptr));
                }
                self.pointer = ptr;
            },
            JumpPC(ptr) => {
                if !(ptr < self.len()) {
                    return Err(HardwareError::PCOutOfBound(ptr));
                }
                self.program_counter = ptr;
            },
            JumpPCIfZero(ptr) => {
                if !(ptr < self.len()) {
                    return Err(HardwareError::PCOutOfBound(ptr));
                }
                if self.pointed_value() == 0 {
                    self.program_counter = ptr;
                }
            },
            Return(v) => return Ok(Some(v)),
            ReturnCell => return Ok(Some(self.pointed_value())),
            SetCellValue(v) => *self.pointed_value_mut() = v,
        }

        // peak rust
        self.program_counter += match inst {
            JumpPC(_) | JumpPCIfZero(_) => 0,
            Increment | Decrement | Negate => 1,
            SetCellValue(_) => 2,
            MovePointer(_) | Add(_) => 5,
            Return(_) | ReturnCell => unreachable!(),
        };
        println!("execute PC: {}", self.program_counter);

        return Ok(None);
    }

    pub fn run(&mut self) -> Result<u8, super::HardwareError> {
        //unimplemented!("run for tape");
        use super::instruction::Instruction::{self, *};
        use super::HardwareError;

        let mut pc: u32;
        // TODO: do actuall checks and
        'runtime: loop {
            pc = self.program_counter;

            match self.execute_instruction(match self.pointed_program() {
                1 => Increment,
                2 => Decrement,
                3 => Negate,
                4 => Add(u32::from_le_bytes(
                    self[(pc as usize + 1)..(pc as usize + 5)].try_into()?,
                )),
                5 => MovePointer(u32::from_le_bytes(
                    self[(pc as usize + 1)..(pc as usize + 5)].try_into()?,
                )),
                6 => JumpPC(u32::from_le_bytes(
                    self[(pc as usize + 1)..(pc as usize + 5)].try_into()?,
                )),
                7 => JumpPCIfZero(u32::from_le_bytes(
                    self[(pc as usize + 1)..(pc as usize + 5)].try_into()?,
                )),
                8 => Return(self[pc + 1]),
                9 => ReturnCell,
                10 => SetCellValue(self[pc + 1]),
                _ => {
                    return Err(HardwareError::InvalidInstruction(
                        pc,
                        self.pointed_program(),
                    ))
                },
            })? {
                None => {},
                Some(ret) => return Ok(ret),
            }
        }
        unreachable!();
    }
}

impl std::ops::Index<u32> for Tape {
    type Output = u8;
    fn index(&self, i: u32) -> &Self::Output {
        return &self.tape[i as usize];
    }
}
impl std::ops::IndexMut<u32> for Tape {
    fn index_mut(&mut self, i: u32) -> &mut Self::Output {
        return &mut self.tape[i as usize];
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Tape {
    type Output = [u8];
    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        return &self.tape[range];
    }
}