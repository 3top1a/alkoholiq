use std::io::stdin;

mod bf;
mod lir;

fn main() {
    let input = stdin().lines();
    let input = input
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let parsed = lir::parser::parse(&input).unwrap();

    let codegen = lir::codegen::Codegen::new(parsed);
    let instructions = codegen.codegen().unwrap();
    let optimized = bf::optimize(instructions);

    println!("{}", optimized);
}
