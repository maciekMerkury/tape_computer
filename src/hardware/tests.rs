#[test]
fn tape_tests() {
    use crate::hardware::tape::*;

    let size: u32 = 2137;
    let mut t = Tape::new(size);

    assert!(t.len() == size);

    assert!(t[0] == 0);
    t[1] = 23;
    assert!(t[1] == 23);
}

#[test]
fn byte_and_instruction_convertion() {
    use super::instruction::*;
    let i1: Instruction = Instruction::Increment;
    let b1: ByteInstruction = i1.into();
    assert!(i1 == b1.try_into().unwrap());
    let i2: Instruction = Instruction::MoveTapePointer(2137);
    let b2: ByteInstruction = i2.into();
    assert!(i2 == b2.try_into().unwrap());
    let i3: Instruction = Instruction::MovePCIfZero(42);
    let b3: ByteInstruction = i3.into();
    assert!(i3 == b3.try_into().unwrap());
}

#[test]
fn insert_instructions() {
    use super::instruction::Instruction::*;

    let mut tape = super::tape::Tape::new(10);
    let mut start_index = 0u32;

    start_index = tape
        .insert_instruction(Increment.into(), start_index)
        .unwrap();
    assert!(tape[0] == 1);

    let insts = vec![Increment, Increment, MoveTapePointer(11), Return(255)];

    for inst in insts {
        start_index = tape.insert_instruction(inst.into(), start_index).unwrap();
    }
    assert!(tape[0] == 1);
    assert!(tape[1] == 1);
    assert!(tape[2] == 1);
    // MovePointer
    assert!(tape[3] == 4);
    // u32 = 10
    assert!(tape[4] == 11);
    assert!(tape[5] == 0);
    assert!(tape[6] == 0);
    assert!(tape[7] == 0);
    // return
    assert!(tape[8] == 9);
    assert!(tape[9] == 255);
    println!("{:?}", tape.tape);
}

#[test]
fn tape_run() {
    //return;
    use super::*;
    use instruction::Instruction::*;

    let mut tape = tape::Tape::new(128);
    let end = tape.len() - 1;
    let instructions = vec![
        MoveTapePointer(end - 2),
        Increment,
        Increment,
        Increment,
        MoveTapePointer(end - 1),
        Increment,
        Increment,
        MoveTapePointer(end - 2),
        Add(end - 1),
        Negate,
        ReturnCell,
    ];
    let mut start_index: u32 = 0;

    for inst in instructions.into_iter() {
        start_index = tape.insert_instruction(inst.into(), start_index).unwrap();
    }

    let result = tape.run().unwrap();

    println!("{}", result);
    assert!(result == 250);
}
