#[cfg(test)]
mod tests {
    use crate::bf;
    use crate::bf::interpreter::Interpreter;
    use crate::lir::codegen::Codegen;
    use crate::lir::instruction::Instruction;
    use crate::lir::instruction::Instruction::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    // TODO Split into "equals generated code" and "prints out string" tests

    #[test]
    fn test_equal_brainfuck() {
        // #[track_caller] makes it report the line of the caller not the func def
        #[track_caller]
        fn assert_eq_bf(code: Vec<Instruction>, expected: &str) {
            let bf = Codegen::new_test(code).codegen().unwrap();

            // Interpret the code and check that at every instruction separator, all temp variables are zero
            for input in ["", "A", "b", "11", "12", "21"] {
                let mut stdin = input.as_bytes();
                let mut stdout = Vec::new();
                let interpret = Interpreter::new();
                interpret.run(&bf, &mut stdin, &mut stdout);
            }

            let bf = bf::optim::remove_non_brainfuck(bf); // To remove instruction separators
            let bf = bf::optim::optimize_no_effect(bf);
            assert_eq!(bf, expected);
        }

        // The end result should be a tape with 11, 250, 11
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
        assert_eq_bf(code, "[-]++++++++++>-<+>----->[-]<<[-<+>>>+<<]<[->+<]>");

        let code = vec![
            Read("a".to_string()),
            Inc("a".to_string()),
            Print("a".to_string()),
            // Instruction::Set("b".to_string(), b'Z'), // Set should be equivalent except for a [-]
            IncBy("b".to_string(), b'Z'),
            Print("b".to_string()),
        ];
        assert_eq_bf(code, "[-],+.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.<");

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
        assert_eq_bf(
            code,
            "[-],>[-]++++++++++++++++++++++++++++++++[-<<+>+>]<<[->>+<<]>.",
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
        assert_eq_bf(
            code,
            "[-],>[-]++++++++++++++++++++++++++++++++[-<<+>->]<<[->>+<<]>.",
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
        assert_eq_bf(code, "[-],[->+<.]>.<");

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
        assert_eq_bf(
            code,
           "[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[-<<+>->]<<[->>+<<]>[>[-<<+>+>]<<[->>+<<]>-.>[-<<+>->]<<[->>+<<]>]>[-<<+>+>]<<[->>+<<]>"
        );

        // If input is A, set it to B and print twice, else print it once
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
        assert_eq_bf(
            code,
            "[-],>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<<<<[-]+>>>>[-<<+>->]<<[->>+<<]<<<[-]>>>>[-<+<<<+>>>>]<[->+<]<<<[>[-]<[-]]>>>>>[-<<+>+>]<<[->>+<<]<<[[-]>>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.<<<[-]]>>>.",
        );

        let code = vec![PrintS("Hello!".to_string())];
        assert_eq_bf(
            code,
            "<[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.------------------------------------------------------------------------------.[-]>",
        );

        // If input is A, print BB, else print input
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
        assert_eq_bf(
            code,
            "[-],<<<[-]+<[-]>>>>-----------------------------------------------------------------[<<<[-]<[-]>>>>[-<<<<+>>>>]]<<<<[->>>>+<<<<]>>>>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<<<[[-]>>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.<<<[-]]>>>.",
        );

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
            PrintS("=".to_string()),
            End,
            IfEqualConst {
                a: "res".to_string(),
                b: 1,
            },
            PrintS("<".to_string()),
            End,
            IfEqualConst {
                a: "res".to_string(),
                b: 2,
            },
            PrintS(">".to_string()),
            End,
        ];
        assert_eq_bf(
            code,
         "[-],>[-],>[-]<[-<<+>->]<<[->>+<<]<<[-]>>>[-<+<<+>>>]<[->+<]<<[[-]>>>>[-<<+>+>]<<[->>+<<]<[-]>[-]+[<+>>>-<-<<<[-]+<[-]>>>>[<<<[-]<[-]>>>>[-<<<<+>>>>]]<<<<[->>>>+<<<<]>[[-]>>>>>[-]+<<<[-]<<[-]][-]+<[-]>>>>>[<<<<[-]<[-]>>>>>[-<<<<<+>>>>>]]<<<<<[->>>>>+<<<<<]>[[-]>>>>>[-]++<<<[-]<<[-]]>>]<[->>+>+<<<]>>>[-<<+>->]<<[->>+<<]<<]>>>>[-<<+>+>]<<[->>+<<]<<[-]+<[-]>>>>>>[<<<<<[-]<[-]>>>>>>[-<<<<<<+>>>>>>]]<<<<<<[->>>>>>+<<<<<<]>[[-]>>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<[-]][-]+<[-]>>>>>>-[<<<<<[-]<[-]>>>>>>[-<<<<<<+>>>>>>]]<<<<<<[->>>>>>+<<<<<<]>>>>>>+<<<<<[[-]>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<[-]][-]+<[-]>>>>>>--[<<<<<[-]<[-]>>>>>>[-<<<<<<+>>>>>>]]<<<<<<[->>>>>>+<<<<<<]>>>>>>++<<<<<[[-]>>[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<[-]]>>>"
        );

        let code = vec![
            // 9 2 0
            Set("a".to_string(), 9),
            Set("b".to_string(), 2),
            Set("c".to_string(), 0),
            Push("a".to_string()),
            Push("b".to_string()),
            Pop("c".to_string()),
            Pop("b".to_string()),
            // Mem should look like: 9 9 2
        ];
        assert_eq_bf(code, "[-]+++++++++>[-]++>[-]<<<<[-]>>[-<+<+>>]<[->+<]<[->>>>>>>[>>]>+<<<[<<]<<<<<]>>>>>>>[>>]+<<[<<]<<<<<[-]>>>[-<<+<+>>>]<<[->>+<<]<[->>>>>>>[>>]>+<<<[<<]<<<<<]>>>>>>>[>>]+<<[<<]<[-]>>>[>>]<[-<[<<]<+>>>[>>]<]<[-]<<[<<]<<[-]>>>>[>>]<[-<[<<]<<+>>>>[>>]<]<[-]<<[<<]<<<");

        let code = vec![
            Set("a".to_string(), 1),
            Match("a".to_string(), vec![2, 1]),
            Case(), // b
            Case(), // a
            End,
        ];
        assert_eq_bf(code, "[-]+<<[-]>>[-<+<+>>]<[->+<][-]+<-[-[[-]>-<]>[-]<]>[-]>");

        let code = vec![PushConst(1), Pop("a".to_string()), Print("a".to_string())];
        assert_eq_bf(code, ">>>[>>]+>+<<<[<<]<[-]>>>[>>]<[-<[<<]<+>>>[>>]<]<[-]<<[<<]<.");
    }

    #[test]
    fn test_correct_printout() {
        #[track_caller]
        fn test_bf(code: &Vec<Instruction>, input: &str, output: &str) {
            let bf = Codegen::new_test(code.clone()).codegen().unwrap();

            // Interpret the code and check that at every instruction separator, all temp variables are zero
            let mut stdin = input.as_bytes();
            let mut stdout = Vec::new();
            let interpret = Interpreter::new();
            interpret.run(&bf, &mut stdin, &mut stdout);

            assert_eq!(output.to_string(), String::from_utf8(stdout).unwrap());
        }

        // If input is not A, print it twice
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
        test_bf(&code, "", "\0\0");
        test_bf(&code, "A", "A");
        test_bf(&code, "AA", "A");
        test_bf(&code, "B", "BB");
        test_bf(&code, "Q", "QQ");


        // Repeat stack pushes and pops
        let code = vec![
            PushConst(65),
            Pop("a".to_string()),
            PushConst(65),
            Pop("a".to_string()),
            PushConst(65),
            Pop("a".to_string()),
            PushConst(65),
            PushConst(65),
            PushConst(65),
            Print("a".to_string()),
            Pop("a".to_string()),
            Pop("a".to_string()),
            Pop("a".to_string()),
            PushConst(b'F'),
            Pop("a".to_string()),
            Print("a".to_string()),
        ];
        test_bf(&code, "", "AF");


        // Prints BAB with stack ops in between to check for pointer consistency
        let code = vec![
            Set("b".to_string(), 66),
            PushConst(65),
            Print("b".to_string()),
            Pop("a".to_string()),
            Print("a".to_string()),
            Print("b".to_string()),
        ];
        test_bf(&code, "", "BAB");

        // Tests match cases and default
        let code = vec![
            Read("a".to_string()),
            Set("b".to_string(), b'-'),
            Match("a".to_string(), vec![b'b', b'a']),
            PrintS("C".to_string()), // default
            Case(),                  // b
            PrintS("B".to_string()),
            Case(), // a
            PrintS("A".to_string()),
            End,
            Print("b".to_string()), // Here to check the pointer is in the correct place
        ];
        test_bf(&code, "", "C-");
        test_bf(&code, "a", "A-");
        test_bf(&code, "b", "B-");

        // This should reverse the two entered values using the stack
        let code = vec![
            Read("a".to_string()),
            Read("b".to_string()),
            Push("a".to_string()),
            Push("b".to_string()),
            Pop("a".to_string()),
            Pop("b".to_string()),
            Print("a".to_string()),
            Print("b".to_string()),
        ];
        test_bf(&code, "AC", "CA");
        test_bf(&code, "A", "\0A");

        // Should print the division result (0x04) and remainder (0x01)
        let code = vec![
            Set("a".to_string(), 9),
            Set("b".to_string(), 2),
            Div {
                a: "a".to_string(),
                b: "b".to_string(),
                remainder: "r".to_string(),
                quotient: "q".to_string(),
            },
            Print("q".to_string()),
            Print("r".to_string()),
        ];
        test_bf(&code, "", "\x04\x01");

        // Should print D
        let code = vec![
            Set("a".to_string(), 17),
            Set("b".to_string(), 4),
            Mul {
                a: "a".to_string(),
                b: "b".to_string(),
            },
            Print("a".to_string()),
        ];
        test_bf(&code, "", "D");
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
                    println!("Testing {}", path.display());
                    let code = std::fs::read_to_string(&path).expect("Failed to read file");
                    let parsed = crate::lir::parser::parse(&code).expect("Failed to parse LIR");
                    let bf = Codegen::new(parsed)
                        .codegen()
                        .expect("Failed to generate BF");
                    println!("{}", bf);

                    // Run pre-optimization - as instruction separators are stripped out, this
                    // allows the interpreter to catch temp-variables-not-zeroed-type errors
                    let interpreter = Interpreter::new();
                    let mut stdin = "".as_bytes();
                    let mut stdout = Box::new(Vec::new());
                    interpreter.run(&bf, &mut stdin, &mut stdout);

                    // Run with optimization
                    let bf = bf::optimize(bf);
                    let interpreter = Interpreter::new();
                    let mut stdin = "".as_bytes();
                    let mut stdout = Box::new(Vec::new());
                    interpreter.run(&bf, &mut stdin, &mut stdout);
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
        let bf = Codegen::new_test(parsed)
            .codegen()
            .expect("Failed to generate BF");
        let bf_optimized = bf::optimize(bf.clone());

        // Same test for optimized and unoptimized versions
        for code in [bf, bf_optimized] {
            let mut stdin = "".as_bytes();
            let mut stdout = Box::new(Vec::new());
            let interpret = bf::interpreter::Interpreter::new();
            interpret.run(&code, &mut stdin, &mut stdout);

            assert_eq!(
                stdout.as_slice(),
                [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233]
            );
        }

        let code = "set f_n-1 1
set f_n-2 1
set n 13

printc f_n-1
prints ' '
printc f_n-2
prints ' '

dec_by n 2

while_nz n
    copy f_n-1 f_n
    add f_n f_n-2

    // Printc will pretty print numbers
    printc f_n

    if_neq n 1
        prints ' '
    end

    copy f_n-2 f_n-1
    copy f_n f_n-2

    dec n
end
prints '\\n'";

        let parsed = crate::lir::parser::parse(code).expect("Failed to parse LIR");
        let bf = Codegen::new_test(parsed)
            .codegen()
            .expect("Failed to generate BF");
        let bf_optimized = bf::optimize(bf.clone());

        // Same test for optimized and unoptimized versions
        for code in [bf, bf_optimized] {
            let mut stdin = "".as_bytes();
            let mut stdout = Box::new(Vec::new());
            let interpret = bf::interpreter::Interpreter::new();
            interpret.run(&code, &mut stdin, &mut stdout);

            assert_eq!(
                String::from_utf8(*stdout.clone()).unwrap(),
                "1 1 2 3 5 8 13 21 34 55 89 144 233\n"
            );
        }
    }

    #[test]
    fn test_rot13() {
        let code = "\
        // Set some variables
set A 64
set Z 91
set a 96
set z 123

read char
while_nz char
    // Between A and Z
    compare char A res
    if_eq res 2
        compare char Z res
        if_eq res 1
            // Add 13
            dec_by char 13

            // Check for underflow
            compare char A res
            if_neq res 2
                // Wrap around
                inc_by char 26
            end
        end
    end


    // Between a and z
    compare char a res
    if_eq res 2
        compare char z res
        if_eq res 1
            // Add 13
            dec_by char 13

            // Check for underflow
            compare char a res
            if_neq res 2
                // Wrap around
                inc_by char 26
            end
        end
    end

    print char

    read char
end";

        let parsed = crate::lir::parser::parse(code).expect("Failed to parse LIR");
        let bf = Codegen::new_test(parsed)
            .codegen()
            .expect("Failed to generate BF");
        let bf_optimized = bf::optimize(bf.clone());

        // Same test for optimized and unoptimized versions
        for code in [bf, bf_optimized] {
            let mut stdin =
                "0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_abcdefghijklmnopqrstuvwxyz"
                    .as_bytes();
            let mut stdout = Box::new(Vec::new());
            let interpret = bf::interpreter::Interpreter::new();
            interpret.run(&code, &mut stdin, &mut stdout);

            assert_eq!(
                String::from_utf8(*stdout.clone()).unwrap(),
                "0123456789:;<=>?@NOPQRSTUVWXYZABCDEFGHIJKLM[]^_nopqrstuvwxyzabcdefghijklm"
            );
        }
    }

    #[test]
    fn test_string_reverse() {
        // Read from examples/lir
        let mut read_code = String::new();
        File::open("examples/lir/string_reverse.lir")
            .expect("Failed to open file")
            .read_to_string(&mut read_code)
            .unwrap();

        let parsed = crate::lir::parser::parse(&*read_code).expect("Failed to parse LIR");
        let bf = Codegen::new_test(parsed)
            .codegen()
            .expect("Failed to generate BF");
        let bf_optimized = bf::optimize(bf.clone());

        // Same test for optimized and unoptimized versions
        for code in [bf, bf_optimized] {
            let mut stdin = "Hello World!".as_bytes();
            let mut stdout = Box::new(Vec::new());
            let interpret = bf::interpreter::Interpreter::new();
            interpret.run(&code, &mut stdin, &mut stdout);

            assert_eq!(String::from_utf8(*stdout.clone()).unwrap(), "!dlroW olleH");
        }
    }
}
