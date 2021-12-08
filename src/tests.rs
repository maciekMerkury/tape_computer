
#[test]
fn bruh() {
    println!("{}", std::mem::size_of::<crate::hardware::Instruction>());
    //assert!(false);
}


#[test]
fn parsing_and_execution_test() {
    use crate::hardware::{Tape, Instruction::*};
    use crate::asm_parser::lines_to_instructions as parse;

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
        ReturnCell,
    ];

    println!("{:?}", insts);
    assert_eq!(insts, target_insts);

    let bytes = crate::hardware::instructions_to_bytes(&insts);

    println!("{:?}", bytes.len());

    let offset = 10_usize;

    let tape: Vec<u8> = {
        let mut temp = vec![0u8; 128];
        bytes.iter().enumerate().for_each(|(i, b)| temp[i] = *b);
        temp
    };

    let mut tape = crate::hardware::Tape::from_vector(tape);

    let result = tape.run().unwrap();

    assert!(result == 30);
}
