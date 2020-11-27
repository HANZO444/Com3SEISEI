mod ast;
mod codegen;
mod parser;
mod token;
mod tokenizer;
mod translate;
mod x64;

use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = {
    let node = {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            eprintln!("./main <source-code>");
            std::process::exit(1);
        }

        tokenizer::tokenize(args.pop().unwrap())?
        let tokens = tokenizer::tokenize(args.pop().unwrap())?;
        parser::parse(tokens)?
    };

    let root_node = parser::parse(tokens)?;
    eprintln!("{:?}", root_node);
    let codes = codegen::codegen(node);

    let assembly: String = translate::to_assembly(codes);

    let mut f = std::fs::File::create("asm.s")?;
    f.write_all(assembly.as_bytes())?;

    Ok(())
}
