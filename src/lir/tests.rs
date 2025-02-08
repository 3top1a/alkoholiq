mod tests {
    use crate::bf::optim::remove_nonbf;
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::Instruction::*;
    use crate::lir::lir::Location::Variable;
    use crate::lir::lir::{BinaryOp, Instruction, Location, Value};

    fn eq(code: String, b: &str) {
        println!("{}", code);
        assert_eq!(remove_nonbf(code), b);
    }

    fn gen(instructions: Vec<Instruction>) -> String {
        Codegen::new().codegen(instructions.into())
    }

    fn test(instruction: Vec<Instruction>, expected: &str) {
        eq(gen(instruction), expected);
    }

    #[test]
    fn test_copy() {
        let c = gen(vec![
            Push(4),
            Push(9),
            Copy {
                from: Value::Location(Location::Stack),
                to: Variable(0),
            },
        ]);
        eq(c, ">++++>+++++++++[-<<+>>]<");
        // The stack should be [9] [4] [0]

        // Copy immidiate to variable
        let c = gen(vec![
            Push(2),
            Copy {
                from: Value::Immediate(4),
                to: Variable(0),
            },
        ]);
        eq(c, ">++<++++>");
        // The stack should be [4] [2]
    }

    #[test]
    fn test_dup() {
        test(vec![Push(4), Dup], ">++++[->+>+<<]>>[-<<+>>]<");
        // The stack should be [0] [4] [4]
    }

    #[test]
    fn test_pop() {
        test(vec![Push(4), Pop], ">++++[-]<");
        // The stack should be [0]
    }

    #[test]
    fn test_binary() {
        // Add stack to stack
        test(
            vec![
                Push(4),
                Push(9),
                Binary {
                    op: BinaryOp::Add,
                    modified: Location::Stack,
                    consumed: Value::Location(Location::Stack),
                },
            ],
            ">++++>+++++++++[-<+>]<",
        );
        // The stack should be [0] [13]

        // Subtract stack from stack
        test(
            vec![
                Push(9),
                Push(4),
                Binary {
                    op: BinaryOp::Sub,
                    modified: Location::Stack,
                    consumed: Value::Location(Location::Stack),
                },
            ],
            ">+++++++++>++++[-<->]<",
        );
        // The stack should be [0] [5]
    }

    #[test]
    fn test_read() {
        // Test read to stack
        test(vec![Read(Location::Stack)], ">,");

        // Test read to variable
        test(vec![Read(Variable(0)), Read(Variable(1))], "><,>,");
    }

    #[test]
    fn test_swap() {
        test(
            vec![Push(4), Push(9), Swap],
            ">++++>+++++++++[->+<]<[->+<]>>[-<<+>>]<",
        );
        // The stack should be [0] [9] [4]
    }

    #[test]
    fn test_print() {
        // Test print stack
        test(
            vec![Push(4), Print(Value::Location(Location::Stack))],
            ">++++.[-]<",
        );

        // Test print variable
        test(
            vec![
                Copy {
                    from: Value::Immediate(4),
                    to: Variable(0),
                },
                Print(Value::Location(Variable(0))),
            ],
            "++++.",
        );

        test(
            vec![Print(Value::Immediate(72))],
            ">++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<",
        )
    }

    #[test]
    fn test_match() {
        // test(
        //     vec![
        //         Push(1),
        //         Match(Location::Stack),
        //         Case(0),
        //         Print(Value::Immediate(b'0')),
        //         Case(1),
        //         Print(Value::Immediate(b'1')),
        //         Case(2),
        //         Print(Value::Immediate(b'2')),
        //         CaseDefault,
        //         Print(Value::Immediate(b'D')),
        //         EndMatch,
        //     ],
        //     "",
        // );



        test(
            vec![
                Push(4),
                Match(Location::Stack),
                CaseDefault,
                Case(1),
                Case(0),
                EndMatch,
            ],
            ">++++>+<[-[[-]>-default#<]>[-1#]<]>[-0#]<",
        );
    }
}
