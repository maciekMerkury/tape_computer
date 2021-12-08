#[test]
fn hello() {
    assert!(true);
}

/*
#[test]
fn tests() {
    let test = vec!("dupa12", "this is #, say hello", "\t hello");

        let a = super::lines_to_tokens(test.into_iter().map(|x| String::from(x)).collect::<Vec<String>>().as_slice());
    assert!(false);
}
*/

#[test]
fn check_and_parse_constant() {
    use super::check_and_parse_constant as check;
    use super::{Constant, Len::*};
    let a = check("#cell A 1".split_whitespace().collect::<Vec<&str>>().as_slice(), 2).unwrap();
    assert!(a == Constant {identifier: "A".into(), content: Cell(1)});

    let a = check("# ptr qwerty 2137".split_whitespace().collect::<Vec<&str>>().as_slice(), 20).unwrap();
    assert!(a == Constant {identifier: "qwerty".into(), content: Pointer(2137)});

    assert!(check("#no qwerty 2137".split_whitespace().collect::<Vec<&str>>().as_slice(), 2).is_err());
    assert!(check("# cell A 2137".split_whitespace().collect::<Vec<&str>>().as_slice(), 2).is_err());
}

#[test]
fn check_and_parse_label() {
    use super::check_and_parse_label as check;
    use super::Label;
    let input = ".DUPA";
    let l = Label {identifier: "DUPA".into(), location: 1};
    let r = check(input, 1, 1).unwrap();
    println!("{:?}", r);
    assert!(l == r);

    let r = check(". no", 0, 0);
    println!("{:?}", r);
    assert!(r.is_err());
}

#[test]
fn check_and_parse_instruction() {
    use super::check_and_parse_instruction as check;
    use crate::hardware::Instruction;
    let short = Instruction::Increment;
    let medium = Instruction::Return(42);
    let long = Instruction::Add(2137);

    let t_short     = "inc".split_whitespace().collect::<Vec<&str>>();
    let t_medium    = "ret 42".split_whitespace().collect::<Vec<&str>>();;
    let t_long      = "add 2137".split_whitespace().collect::<Vec<&str>>();;

    let r_short = check(&t_short, 0, &vec![], &vec![]).unwrap();
    let r_medium = check(&t_medium, 0, &vec![], &vec![]).unwrap();
    let r_long = check(&t_long, 0, &vec![], &vec![]).unwrap();

    assert_eq!(1, r_short.0);
    assert_eq!(2, r_medium.0);
    assert_eq!(5, r_long.0);

    assert_eq!(short, r_short.1);
    assert_eq!(medium, r_medium.1);
    assert_eq!(long, r_long.1);
}

#[test]
fn lines_to_instructions() {
    use super::lines_to_instructions as func;
    use super::{Token, Constant, Label};
    use crate::hardware::Instruction;

    let dupa12 = 
        "# ptr A 2137 
        # cell Dupa 42
        inc
        inc
        .TEST
        mtp A
        mpc TEST
        ret Dupa
        ; this is a pog comment
        ";

    let toks = func(&dupa12.lines().collect::<Vec<&str>>()).unwrap();

    let target = vec![
        Instruction::Increment,
        Instruction::Increment,
        Instruction::MoveTapePointer(2137),
        Instruction::MovePC(2),
        Instruction::Return(42),
    ];

    assert_eq!(toks, target);
}

