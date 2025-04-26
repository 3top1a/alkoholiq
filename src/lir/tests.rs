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
            ">>>[-]++++++++++>-<+>>[-]<<[-<<<+>+>>]<<<[->>>+<<<]>[->>>>+<<<<]>>>>"
        );

        let code = vec![
            Instruction::Read("a".to_string()),
            Instruction::Inc("a".to_string()),
            Instruction::Print("a".to_string()),
            // Instruction::Set("b".to_string(), b'Z'), // Set should be equivalent except for a [-]
            Instruction::IncBy("b".to_string(), b'Z'),
            Instruction::Print("b".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>,+.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.");

        // This should error
        let code = vec![Instruction::Read("a1".to_string())];
        assert!(Codegen::new(code).codegen().is_err());

        let code = vec![
            Instruction::Read("a".to_string()),
            Instruction::Set("b".to_string(), 32),
            Instruction::Add {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Instruction::Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(
            bf,
            ">>>,>[-]++++++++++++++++++++++++++++++++[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>."
        );

        let code = vec![
            Instruction::Read("a".to_string()),
            Instruction::Set("b".to_string(), 32),
            Instruction::Sub {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Instruction::Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(
            bf,
            ">>>,>[-]++++++++++++++++++++++++++++++++[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>>."
        );

        // This should error
        let code = vec![Instruction::End];
        assert!(Codegen::new(code).codegen().is_err());

        let code = vec![
            Instruction::Read("a".to_string()),
            Instruction::WhileNotZero("a".to_string()),
            Instruction::Dec("a".to_string()),
            Instruction::Print("a".to_string()),
            Instruction::End,
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>,[-.]");

        let code = vec![
            Instruction::Read("a".to_string()),
            Instruction::Set("b".to_string(), b'A'),
            Instruction::IfNotEqual {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Instruction::Print("a".to_string()),
            Instruction::End,
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>,>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>>[>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>.>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>><<<]>>>>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>");
    }
}
