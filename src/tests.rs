#[test]
fn bruh() {
    println!("{}", std::mem::size_of::<crate::hardware::Instruction>());
    println!("{}", std::mem::size_of::<crate::Word>() + 1);
    //assert!(false);
}


#[test]
fn parsing_and_execution_test() {
    use crate::asm_parser::lines_to_instructions as parse;
    use crate::hardware::{Instruction::*, Tape};

    println!("{}", std::mem::size_of::<crate::Word>());

    let asm = "
        #ptr arg1 0
        #ptr arg2 1
        mtp arg1
        scv 10
        mtp arg2
        scv 20
        mtp 3
        ccv arg1
        add arg2
        mpc END
        inc
        inc
        inc
        inc
        .END
        retc
        ";
    let insts = parse(&asm.lines().collect::<Vec<&str>>()).unwrap();
    let target_insts = vec![
        MoveTapePointer(0),
        SetCellValue(10),
        MoveTapePointer(1),
        SetCellValue(20),
        MoveTapePointer(3),
        CopyCellValue(0),
        Add(1),
        MovePC(26),
        Increment,
        Increment,
        Increment,
        Increment,
        ReturnCell,
    ];

    println!("{:?}", insts);
    assert_eq!(insts, target_insts);

    let bytes = crate::hardware::instructions_to_bytes(&insts);

    println!("bytes: {:?}", bytes);
    println!("error?: {}", bytes[15]);

    println!("{:?}", bytes.len());

    let offset = 10_usize;

    let mut tape: Vec<u8> = bytes.clone();
    tape.append(&mut vec![0u8; 32]);

    let mut tape = crate::hardware::Tape::from_vector(tape);
    //println!("{:?}", tape);

    let result = tape.run().unwrap();

    assert!(result == 30);
}

#[test]
fn overall_test() {
    use crate::*;

    let instructions = "
        #ptr arg1 0
        #ptr arg2 1
        # ptr arg3 2
        mtp arg1
        scv 2
        inc
        dec
        ".lines().collect::<Vec<&str>>();

    assert!(false);
}
