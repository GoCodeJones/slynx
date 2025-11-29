pub mod checker;
pub mod compiler;
pub mod hir;
pub mod intermediate;
pub mod parser;
use std::path::PathBuf;

use crate::{
    checker::TypeChecker,
    compiler::Compiler,
    intermediate::IntermediateRepr,
    parser::{Parser as SlynxParser, lexer::Lexer},
    //flattener::{FlattenedHir, Flattener},
};
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    target: String,
}

/*#[link_name = "slynx_backend"]
unsafe extern "C" {
    fn compile_code(hir: FlattenedHir);
}*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let path = PathBuf::from(cli.target);

    let file = std::fs::read_to_string(&path).unwrap();
    let lines = file
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| if c == '\n' { Some(idx) } else { None })
        .collect::<Vec<usize>>();
    let stream = Lexer::tokenize(&file);
    let mut value = SlynxParser::new(stream);
    let ast = value.parse_declarations()?;

    let mut hir = hir::SlynxHir::new();
    hir.generate(ast)?;
    if let Err(e) = TypeChecker::check(&mut hir) {
        eprint!("Type Error: {:?}; ", e.kind);
        let line = match lines.binary_search(&e.span.start) {
            Ok(idx) => idx,
            Err(idx) => idx + 1,
        };
        let column = e.span.end - e.span.start;
        eprintln!("At {}:{}", line, column);
        std::process::exit(1);
    };
    let mut intermediate = IntermediateRepr::new();
    let _ = intermediate.generate(hir.declarations);
    let mut compiler = Compiler::new();
    let out = compiler.compile(&intermediate);
    let _ = std::fs::write(path.with_extension("js"), out);
    Ok(())
}
