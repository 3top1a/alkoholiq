mod tests {
    use brainfuck::Interpreter;
    use brainfuck::program::Program;
    use brainfuck::tape::ArrayTape;
    use crate::bf::optim::remove_nonbf;
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::Instruction::{
        Binary, Dup, Match, Move, Pop, Print, Push, Read, Swap, While,
    };
    use crate::lir::lir::Location::{Stack, Variable};
    use crate::lir::lir::Value::Immediate;
    use crate::lir::lir::{BinaryOp, Instruction, Location, Value};

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
    fn test_move() {
        let c = gen(vec![
            Push(4),
            Push(9),
            Move {
                from: Value::Location(Stack),
                to: Variable(0),
            },
        ]);
        eq(c, ">++++>+++++++++[-<<+>>]<");
        // The stack should be [9] [4] [0]

        // Move immediate to variable
        let c = gen(vec![
            Push(2),
            Move {
                from: Immediate(4),
                to: Variable(0),
            },
        ]);
        eq(c, ">++<++++>");
        // The stack should be [4] [2]

        // Test reassignment
        // It should empty it out first
        test(
            vec![
                Move {
                    from: Immediate(b'E'),
                    to: Variable(0),
                },
                Move {
                    from: Immediate(b'A'),
                    to: Variable(0),
                },
                Print(Value::Location(Variable(0))),
            ],
            "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.",
        )
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
                Move {
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
            ">++++>+<[-[[-]>-<<>><]>[-]<]>[-]<",
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
            "Variables: 0
(Push 1) >+
(Match Stack) >+<[-[-[---[[-]>-<<(Print Immediate(69)) >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<
>><]>[-(Print Immediate(53)) >+++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<
]<]>[-(Print Immediate(50)) >++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<
]<]>[-(Print Immediate(49)) >+++++++++++++++++++++++++++++++++++++++++++++++++.[-]<
]<]>[-(Print Immediate(48)) >++++++++++++++++++++++++++++++++++++++++++++++++.[-]<
]<"
        );

        // Test that variables and pointer math works
        test(
            vec![
                Move {from: Immediate(b'0'), to: Variable(0)},
                Read(Stack),
                Match {
                    source: Stack,
                    default: vec![Print(Immediate(b'E'))].into(),
                    cases: vec![
                        (b'A', vec![Print(Value::Location(Variable(0)))].into()),
                    ]
                },
                Move {from: Immediate(b'x'), to: Variable(1)},
                Print(Value::Location(Variable(1)))
            ],
            "Variables: 2 >
(Move Immediate(48) Variable(0)) <++++++++++++++++++++++++++++++++++++++++++++++++>
(Read Stack) >,
(Match Stack) >+<-----------------------------------------------------------------[[-]>-<<(Print Immediate(69)) >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]<
>><]>[-(Print Location(Variable(0))) <<<.>>>
]<
(Move Immediate(120) Variable(1)) <++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>
(Print Location(Variable(1))) <.>"
        )
    }

    #[test]
    fn test_while() {
        // Simple decrement and print loop
        test(
            vec![
                Move {from: Immediate(b'z'), to: Variable(0)},
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

    #[test]
    fn test_interpreted_cat() {
        // Worst `cat(1)` implementation ever
        let c = gen(vec![
            Move {
                from: Immediate(1),
                to: Variable(0),
            },
            While {
                source: Variable(0),
                body: vec![
                    Read(Stack),
                    Dup,
                    Match {
                        source: Stack,
                        cases: vec![(
                            0,
                            vec![Move {
                                from: Immediate(0),
                                to: Variable(0),
                            }]
                            .into(),
                        )],
                        default: vec![Print(Value::Location(Stack))].into(),
                    },
                ]
                .into(),
            },
        ]);

        let mut stdin = "Hello, World!".as_bytes();
        let mut stdout = Vec::new();
        let program = Program::parse(&c).unwrap();
        let mut interp = Interpreter::<ArrayTape>::new(program, &mut stdin, &mut stdout);
        interp.run().unwrap();
        assert_eq!(stdout, "Hello, World!".as_bytes());
    }
}
