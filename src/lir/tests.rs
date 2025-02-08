mod tests {
    use crate::bf::optim::remove_nonbf;
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::{Instruction, Location, Value};
    use crate::lir::lir::Instruction::*;

    fn eq(code: String, b: &str) {
        println!("{}", code);
        assert_eq!(remove_nonbf(code), b);
    }

    fn gen(instructions: Vec<Instruction>) -> String {
        Codegen::new().codegen(instructions.into())
    }

    #[test]
    fn test() {
        let c = gen(vec![
            Push(4),
            Push(9),
            Copy {
                from: Value::Location(Location::Stack),
                to: Location::Variable(0),
            },
        ]);
        eq(c, ">++++>+++++++++[-<<+>>]<");
        // The stack should be [9] [4]
    }
}
