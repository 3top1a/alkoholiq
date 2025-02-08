mod tests {
    use crate::lir::lir::LIR;

    #[test]
    fn test_stack_machine_add_sub() {
        use crate::lir::codegen::Codegen;
        use crate::lir::lir::LIR::*;
        let mut c = Codegen::new();

        c.generate(vec![Push(b'a'), True]);

        assert_eq!(c.stack, vec![97, 1]);
        assert_eq!(c.code, ">+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+");

        c.generate(vec![Add]);

        assert_eq!(c.stack, vec![98]);
        assert_eq!(c.code, ">+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+[-<+>]<");

        c.generate(vec![Print]);

        assert_eq!(c.stack, vec![]);
        assert_eq!(c.code, ">+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+[-<+>]<.[-]<");

        let mut c = Codegen::new();
        c.generate(vec![Push(2), Push(3), Sub]);

        assert_eq!(c.stack, vec![255]);
        assert_eq!(c.code, ">++>+++[-<->]<");

        let mut c = Codegen::new();
        c.generate(vec![Push(3), Push(2), Sub]);

        assert_eq!(c.stack, vec![1]);
        assert_eq!(c.code, ">+++>++[-<->]<");
    }

    #[test]
    fn test_input() {
        use crate::lir::codegen::Codegen;
        use crate::lir::codegen::INPUT;
        use crate::lir::lir::LIR::*;
        let mut c = Codegen::new();
        c.generate(vec![Input]);

        assert_eq!(c.stack, vec![INPUT]);
        assert_eq!(c.code, ">,");
    }

    #[test]
    fn test_stack_vars() {
        use crate::lir::codegen::Codegen;
        use crate::lir::lir::LIR::*;
        let mut c = Codegen::new();

        c.generate(vec![Push(2), True, Var(0)]);

        assert_eq!(c.stack, vec![2, 1, 2]);
        assert_eq!(c.code, ">++>+<[->>+>+<<<]>>>[-<<<+>>>]<");

        let mut c = Codegen::new();
        c.generate(vec![Push(2), Push(6), False, True, Var(1)]);

        assert_eq!(c.stack, vec![2, 6, 0, 1, 6]);
        assert_eq!(c.code, ">++>++++++>>+<<[->>>+>+<<<<]>>>>[-<<<<+>>>>]<");

        let mut c = Codegen::new();
        c.generate(vec![False, True, Push(2), Dup]);

        assert_eq!(c.stack, vec![0, 1, 2, 2]);
        assert_eq!(c.code, ">>+>++[->+>+<<]>>[-<<+>>]<");

        c.generate(vec![Pop]);

        assert_eq!(c.stack, vec![0, 1, 2]);
        assert_eq!(c.code, ">>+>++[->+>+<<]>>[-<<+>>]<[-]<");
    }

    #[test]
    fn test_stack_eq() {
        use crate::lir::codegen::Codegen;
        use crate::lir::lir::LIR::*;
        let mut c = Codegen::new();

        c.generate(vec![Push(2), Push(2), Eq]);

        assert_eq!(c.stack, vec![1]);
        assert_eq!(c.code, ">++>++[-<->]+<[->[-]<]>[-<+>]<");

        let mut c = Codegen::new();

        c.generate(vec![Push(2), Push(3), Eq]);

        assert_eq!(c.stack, vec![0]);
        assert_eq!(c.code, ">++>+++[-<->]+<[->[-]<]>[-<+>]<");
    }

    #[test]
    fn test_if() {
        use crate::lir::codegen::Codegen;
        use crate::lir::lir::LIR::*;
        use crate::bf::optim::remove_comments;
        let mut c = Codegen::new();
        c.generate(vec![Push(2), Push(3), True, StartIf, Push(5), StartElse, Push(6), EndIf]);

        println!("{}", c.code);
        // assert_eq!(c.stack, vec![0, 2]);
        assert_eq!(remove_comments(c.code), ">[>++]");
        
        let mut c = Codegen::new();
        c.generate(vec![True, StartIf, Push(2), StartElse, EndIf]);
        
        assert_eq!(c.stack, vec![1, 2]);
        // assert_eq!(c.code, ">+[-<+>]<");
    }
}
