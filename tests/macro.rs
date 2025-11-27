use slynx;
#[cfg(test)]
use std::path::PathBuf;

#[test]
fn test_macro() {
    let path = PathBuf::from("..");

    let file = std::fs::read_dir(&path).unwrap();
    eprintln!("{file:?}");
}
