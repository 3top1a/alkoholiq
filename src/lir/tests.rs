mod tests {
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::Instruction;
    use crate::lir::lir::Instruction::*;

    #[test]
    fn basics() {
        let code = vec![
            Set("asdf".to_string(), 10),
            Dec("another".to_string()),
            Inc("asdf".to_string()),
            DecBy("another".to_string(), 5),
            Copy {
                a: "asdf".to_string(),
                b: "newasdf".to_string(),
            },
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(
            bf,
            ">>>[-]++++++++++>-<+>----->[-]<<[-<<<+>+>>]<<<[->>>+<<<]>[->>>>+<<<<]>>>>"
        );

        let code = vec![
            Read("a".to_string()),
            Inc("a".to_string()),
            Print("a".to_string()),
            // Instruction::Set("b".to_string(), b'Z'), // Set should be equivalent except for a [-]
            IncBy("b".to_string(), b'Z'),
            Print("b".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>[-],+.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.");

        // This should error
        let code = vec![Read("a1".to_string())];
        assert!(Codegen::new(code).codegen().is_err());

        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), 32),
            Add {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(
            bf,
            ">>>[-],>[-]++++++++++++++++++++++++++++++++[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>."
        );

        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), 32),
            Sub {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(
            bf,
            ">>>[-],>[-]++++++++++++++++++++++++++++++++[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>>."
        );

        // This should error
        let code = vec![End];
        assert!(Codegen::new(code).codegen().is_err());

        // This should error
        let code = vec![
            IfEqual {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            End,
        ];
        assert!(Codegen::new(code).codegen().is_err());

        let code = vec![
            Read("a".to_string()),
            WhileNotZero("a".to_string()),
            Dec("a".to_string()),
            Print("a".to_string()),
            Raw("ASDF".to_string()),
            End,
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>[-],[-.ASDF]");

        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), b'A'),
            IfNotEqual {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Print("a".to_string()),
            End,
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>>[>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>.>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>><<<]>>>>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>");

        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), b'A'),
            UntilEqual {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Dec("a".to_string()),
            Print("a".to_string()),
            End,
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>>[>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>-.>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>>]>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>");

        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), b'A'),
            IfEqual {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Print("a".to_string()),
            End,
            Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<<[-]+>>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>>[<[-]>>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>]<[>>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>.>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>-<<]>><[-]]>>[-<<<<+>+>>>]<<<<[->>>>+<<<<]>[->>+<<]>>.");
    }

    #[test]
    fn small_programs() {
        // A small cat implementation
        let code = vec![
            Read("a".to_string()),
            WhileNotZero("a".to_string()),
            Print("a".to_string()),
            Read("a".to_string()),
            End,
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, ">>>[-],[.[-],]");
    }
}
