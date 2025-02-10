mod tests {
    use crate::bf::optim::remove_nonbf;
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::{BinaryOp, Instruction, Location, Value};
    use crate::lir::lir::Instruction::{
        Binary, Copy, Dup, Match, Pop, Print, Push, Read, Swap, While,
    };
    use crate::lir::lir::Location::{Stack, Variable};
    use crate::lir::lir::Value::Immediate;

    fn eq(code: String, b: &str) {
        println!("{}", code);
        assert_eq!(remove_nonbf(code), remove_nonbf(b.to_string()));
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
                from: Value::Location(Stack),
                to: Variable(0),
            },
        ]);
        eq(c, ">++++>+++++++++[-<<+>>]<");
        // The stack should be [9] [4] [0]

        // Copy immidiate to variable
        let c = gen(vec![
            Push(2),
            Copy {
                from: Immediate(4),
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
                    modified: Stack,
                    consumed: Value::Location(Stack),
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
                    modified: Stack,
                    consumed: Value::Location(Stack),
                },
            ],
            ">+++++++++>++++[-<->]<",
        );
        // The stack should be [0] [5]
    }

    #[test]
    fn test_read() {
        // Test read to stack
        test(vec![Read(Stack)], ">,");

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
        test(vec![Push(4), Print(Value::Location(Stack))], ">++++.[-]<");

        // Test print variable
        test(
            vec![
                Copy {
                    from: Immediate(4),
                    to: Variable(0),
                },
                Print(Value::Location(Variable(0))),
            ],
            "++++.",
        );

        test(
            vec![Print(Immediate(72))],
            ">++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<",
        )
    }

    #[test]
    fn test_match() {
        test(
            vec![
                Push(4),
                Match {
                    source: Stack,
                    cases: vec![(0, vec![].into()), (1, vec![].into())],
                    default: vec![].into(),
                },
            ],
            ">++++>+<[-[[-]>-default#<]>[-1#]<]>[-0#]<",
        );

        test(
            vec![
                Push(1),
                Match {
                    source: Stack,
                    cases: vec![
                        (0, vec![Print(Immediate(b'0'))].into()),
                        (1, vec![Print(Immediate(b'1'))].into()),
                        (2, vec![Print(Immediate(b'2'))].into()),
                        (5, vec![Print(Immediate(b'5'))].into()),
                    ],
                    default: vec![Print(Immediate(b'E'))].into(),
                }
            ],
            "Push(1) >+Match >+<[-[-[---[[-]>-Print Immediate(69) >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<]>[-Print Immediate(53) >+++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<]<]>[-Print Immediate(50) >++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<]<]>[-Print Immediate(49) >+++++++++++++++++++++++++++++++++++++++++++++++++.[-]<]<]>[-Print Immediate(48) >++++++++++++++++++++++++++++++++++++++++++++++++.[-]<]<",
        );

        // Test that variables and pointer math works
        test(
            vec![
                Copy {from: Immediate(b'0'), to: Variable(0)},
                Read(Stack),
                Match {
                    source: Stack,
                    default: vec![Print(Immediate(b'E'))].into(),
                    cases: vec![
                        (b'A', vec![Print(Value::Location(Variable(0)))].into()),
                    ]
                },
                Copy {from: Immediate(b'x'), to: Variable(1)},
                Print(Value::Location(Variable(1)))
            ],
            "Variables: 2 >
Copy Immediate(48) to Variable(0) <++++++++++++++++++++++++++++++++++++++++++++++++>
Read Stack >,
Match >+<-----------------------------------------------------------------[[-]>-Print Immediate(69) >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<
<]>[-Print Location(Variable(0)) <<<.>>>
]<
Copy Immediate(120) to Variable(1) <++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>
Print Location(Variable(1)) <.>"
        )
    }

    #[test]
    fn test_while() {
        // Simple decrement and print loop
        test(
            vec![
                Copy {from: Immediate(b'z'), to: Variable(0)},
                While {
                    source: Variable(0),
                    body: vec![
                        Print(Value::Location(Variable(0))),
                        Binary {
                            op: BinaryOp::Sub,
                            modified: Variable(0),
                            consumed: Immediate(1),
                        },
                    ].into()
                }
            ],
            "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[.-]"
        );
    }
}
