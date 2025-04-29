#[cfg(test)]
mod tests {
    use crate::bf;
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::Instruction;
    use crate::lir::lir::Instruction::*;
    use std::io;
    use std::path::Path;

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
        let code = vec![Read("1".to_string())];
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

        // Should print D
        let code = vec![
            Set("a".to_string(), 17),
            Set("b".to_string(), 4),
            Mul {a: "a".to_string(), b: "b".to_string()},
            Print("a".to_string()),
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        assert_eq!(bf, "[-]+++++++++++++++++>[-]++++<<<<[-]>>>>[-<<+<+>>>]<<[->>+<<]<[-<+>]<[->>>[-<+<+>>]<[->+<]<[-<<+>>]<<>]>>>[-]<<<<[->>>>+<<<<]>>>>.");

        let code = vec![
            Set("a".to_string(), 9),
            Set("b".to_string(), 2),
            // Div {a: "a".to_string(), b: "b".to_string(), r: "r".to_string(), q: "q".to_string()},
        ];
        let bf = Codegen::new(code).codegen().unwrap();
        // assert_eq!(bf, "");
    }

    #[test]
    fn parse_examples() {
        // Note: This only parses the examples and codegens them, it does not run them.
        // Correct behaviour is not guaranteed.
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

    #[test]
    fn run_fibonacci_sequence() {
        let code = "set f_n-1 1\n
set f_n-2 1\n
set n 13\n
print f_n-1\n
print f_n-2\n
dec_by n 2\n
while_nz n\n
    copy f_n-1 f_n\n
    add f_n f_n-2\n
    print f_n\n
    copy f_n-2 f_n-1\n
    copy f_n f_n-2\n
    dec n\n
end";

        let parsed = crate::lir::parser::parse(code).expect("Failed to parse LIR");
        let bf = Codegen::new(parsed)
            .codegen()
            .expect("Failed to generate BF");
        let bf_optim = bf::optimize(bf.clone());

        // Same test for optimized and unoptimized versions
        for code in [bf, bf_optim] {
            // Because the interpreter complains of underflow, shift all pointers a bit
            // TODO Custom interpreter
            let code = ">>>>>>>>>".to_owned() + &*code;

            let mut stdin = io::stdin();
            let mut stdout = Vec::new();
            let program = brainfuck::program::Program::parse(&*code).unwrap();
            let mut interp = brainfuck::Interpreter::<brainfuck::tape::VecTape>::new(
                program,
                &mut stdin,
                &mut stdout,
            );
            let _ = interp.run().unwrap();
            assert_eq!(stdout, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233]);
        }
    }
}
