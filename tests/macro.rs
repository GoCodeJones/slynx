use slynx::{checker::TypeChecker, hir::SlynxHir};
#[cfg(test)]
use std::path::PathBuf;

#[test]
fn test_macro() {
    let path = PathBuf::from("./slynx/macro.slynx");

    let file = std::fs::read_to_string(&path).unwrap();
    let lines = file
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| if c == '\n' { Some(idx) } else { None })
        .collect::<Vec<usize>>();

    let value = slynx::slynx::ProgramParser::new().parse(&file).unwrap();
    println!("{value:#?}");
    let mut hir = match SlynxHir::new(value) {
        Ok(hir) => hir,
        Err(e) => panic!("{e:#?}"),
    };
    if let Err(e) = TypeChecker::check(&mut hir) {
        eprint!("Type Error: {:?}; ", e.kind);
        let line = match lines.binary_search(&e.span.start) {
            Ok(idx) => idx,
            Err(idx) => idx + 1,
        };
        let column = e.span.end - e.span.start;
        panic!("At {}:{}", line, column);
    };
    println!("\n{hir:?}");
}
