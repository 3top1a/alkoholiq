use anyhow::Result;
use argh::FromArgs;
use std::io::{stdin, Read};
use std::path::PathBuf;

mod bf;
mod lir;

#[derive(FromArgs, Debug)]
/// Compile and/or interpret Alkoholiq
struct CliArgs {
    /// dump generated brainfuck instead of interpreting
    #[argh(switch, short = 'b')]
    brainfuck: bool,

    /// optimize the generated brainfuck
    // TODO This is quite confusing, make it level based like gcc
    #[argh(option, short = 'o', default = "true")]
    optimize: bool,

    /// input file
    #[argh(positional)]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args: CliArgs = argh::from_env();

    let mut input_cursor: Box<dyn Read>;

    if args.input.is_none() {
        input_cursor = Box::new(stdin());
    } else {
        let path = args.input.unwrap();
        if path.as_os_str() == "-" {
            input_cursor = Box::new(stdin());
        } else {
            input_cursor = Box::new(std::fs::File::open(path)?);
        }
    }
    let mut input = String::new();
    input_cursor.read_to_string(&mut input)?;

    let parsed = lir::parser::parse(&input)?;

    let codegen = lir::codegen::Codegen::new(parsed);
    let mut code = codegen.codegen()?;

    if args.optimize {
        code = bf::optimize(code);
    }

    if args.brainfuck {
        println!();
        println!("{code}");
        return Ok(());
    }

    let interpreter = bf::interpreter::Interpreter::new();
    interpreter.run(&code, &mut stdin(), &mut std::io::stdout());

    Ok(())
}
