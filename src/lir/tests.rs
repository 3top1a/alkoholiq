mod tests {
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::Instruction;

    #[test]
    fn basics() {
        let code = vec![
            Instruction::Set("asdf".to_string(), 10),
            Instruction::Dec("another".to_string()),
            Instruction::Inc("asdf".to_string()),
            Instruction::Copy {
                a: "asdf".to_string(),
                b: "newasdf".to_string(),
            },
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(
            bf,
            ">>[-]++++++++++>-<+>>[-]<<[-<<+>+>]<<[->>+<<]>[->>>+<<<]>>>"
        );

        let code = vec![
            Instruction::Read("a".to_string()),
            Instruction::Inc("a".to_string()),
            Instruction::Print("a".to_string()),
            Instruction::Set("b".to_string(), b'Z'),
            Instruction::Print("b".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>,+.>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.");
    }
}
