#[cfg(test)]
mod tests {
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::Instruction;
    use crate::lir::lir::Instruction::*;
    use std::path::Path;
    use crate::bf;

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
            "[-]++++++++++>-<+>----->[-]<<[-<+<+>>]<[->+<]<[->>>>+<<<<]>>>>"
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
        assert_eq!(bf, "[-],+.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.");

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
            "[-],>[-]++++++++++++++++++++++++++++++++[-<<+<+>>>]<<[->>+<<]<[->>+<<]>>."
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
            "[-],>[-]++++++++++++++++++++++++++++++++[-<<+<+>>>]<<[->>+<<]<[->>-<<]>>."
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
            Inc("b".to_string()),
            Print("a".to_string()),
            Raw("ASDF".to_string()),
            End,
            Print("b".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, "[-],[->+<.ASDF]>.");

        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), b'A'),
            IfNotEqual {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Print("a".to_string()),
            End,
            Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, "[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[-<<+<+>>>]<<[->>+<<]<[->>-<<]>><<<[-]>>>[-<+<+>>]<[->+<]<[-<+>]<[[-]>>>>[-<<+<+>>>]<<[->>+<<]<[->>+<<]>>.>[-<<+<+>>>]<<[->>+<<]<[->>-<<]>><<<]>>>>[-<<+<+>>>]<<[->>+<<]<[->>+<<]>>.");

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
        assert_eq!(bf, "[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[-<<+<+>>>]<<[->>+<<]<[->>-<<]>>[>[-<<+<+>>>]<<[->>+<<]<[->>+<<]>>-.>[-<<+<+>>>]<<[->>+<<]<[->>-<<]>>]>[-<<+<+>>>]<<[->>+<<]<[->>+<<]>>");

        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), b'A'),
            IfEqual {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Set("a".to_string(), b'B'),
            Print("a".to_string()),
            End,
            Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, "[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<<<<[-]+>>>>[-<<+<+>>>]<<[->>+<<]<[->>-<<]>><<<<[-]>>>>[-<+<+>>]<[->+<]<[-<<+>>]<<[>[-]<[-]]>>>>>[-<<+<+>>>]<<[->>+<<]<[->>+<<]>><<<[>>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.<<<[-]][-]>>>.");

        let code = vec![PrintMsg("Hello!".to_string())];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, "<[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.------------------------------------------------------------------------------.[-]");

        let code = vec![
            Read("a".to_string()),
            IfEqualConst {
                a: "a".to_string(),
                b: b'A',
            },
            Set("a".to_string(), b'B'),
            Print("a".to_string()),
            End,
            Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, "[-],<<<[-]+>>>-----------------------------------------------------------------<<<<[-]>>>>[-<+<+>>]<[->+<]<[-<<+>>]<<[>[-]<[-]]>>>>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<<<[>>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.<<<[-]][-]>>>.");

        let code = vec![
            Read("a".to_string()),
            Read("b".to_string()),
            Compare {
                a: "a".to_string(),
                b: "b".to_string(),
                res: "res".to_string(),
            },
            IfEqualConst {
                a: "res".to_string(),
                b: 0,
            },
            PrintMsg("=".to_string()),
            End,
            IfEqualConst {
                a: "res".to_string(),
                b: 1,
            },
            PrintMsg("<".to_string()),
            End,
            IfEqualConst {
                a: "res".to_string(),
                b: 2,
            },
            PrintMsg(">".to_string()),
            End,
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, "[-],>[-],>[-]<[-<<+<+>>>]<<[->>+<<]<[->>-<<]>><<<[-]>>>[-<+<+>>]<[->+<]<[-<+>]<[[-]>>>>[-<<+<+>>>]<<[->>+<<]<[->>+<<]>>>>[-]++++++++<<<<<<[-]<[-]+<[-]>[>>>>>->-<<<<<<<+>>>[-]+<[->>>+<+<<]>>>[-<<<+>>>]<[->>-<<]>><<<<[-]>>>>[-<+<+>>]<[->+<]<[-<<+>>]<<[>[-]<[-]][->>>+<+<<]>>>[-<<<+>>>]<[->>+<<]>><<<[>>>>>[-]+<<<<<<<[-]>>[-]][-][-]+<[->>>+<+<<]>>>[-<<<+>>>]<[->>>-<<<]>>><<<<<[-]>>>>>[-<<+<+>>>]<<[->>+<<]<[-<<+>>]<<[>[-]<[-]][->>>+<+<<]>>>[-<<<+>>>]<[->>>+<<<]>>><<<<[>>>>>[-]++<<<<<<<[-]>>[-]][-]<<]<[>>>>>>+>+<<<<<<<-]>>>>>>>[-<<+<+>>>]<<[->>+<<]<[->>-<<]>><<<]>>>>[-<<+<+>>>]<<[->>+<<]<[->>+<<]>><<<[-]+>>>>><<<<<<[-]>>>>>>[-<<<+<+>>>>]<<<[->>>+<<<]<[-<<+>>]<<[>[-]<[-]]>>>>>><<<<<[>>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<[-]][-][-]+>>>>>-<<<<<<[-]>>>>>>[-<<<+<+>>>>]<<<[->>>+<<<]<[-<<+>>]<<[>[-]<[-]]>>>>>>+<<<<<[>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<[-]][-][-]+>>>>>--<<<<<<[-]>>>>>>[-<<<+<+>>>>]<<<[->>>+<<<]<[-<<+>>]<<[>[-]<[-]]>>>>>>++<<<<<[>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<[-]][-]");
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
        assert_eq!(bf, "[-],[.[-],]");
    }

    #[test]
    fn run_examples() {
        let examples_dir = Path::new("examples/lir");

        let entries = std::fs::read_dir(examples_dir).expect("Failed to read examples directory");

        for entry in entries {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(".lir") {
                    let code = std::fs::read_to_string(&path).expect("Failed to read file");
                    let parsed = crate::lir::parser::parse(&code).expect("Failed to parse LIR");
                    let bf = Codegen::new(parsed)
                        .codegen()
                        .expect("Failed to generate BF");
                    let bf = bf::optimize(bf);
                    println!("{}", bf);
                }
            }
        }
    }
}
