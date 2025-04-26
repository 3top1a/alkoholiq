mod tests {
    use crate::lir::codegen::Codegen;
    use crate::lir::lir::Instruction;

    #[test]
    fn basic_vars() {
        let code = vec![
            Instruction::Set("asdf".to_string(), 10),
            Instruction::Dec("another".to_string()),
            Instruction::Inc("asdf".to_string()),
        ];

        let bf = Codegen::new(code).codegen().unwrap();

        assert_eq!(bf, ">>[-]++++++++++>-<+");
    }
}