use std::{collections::HashMap, path::PathBuf, sync::Arc};

use color_eyre::eyre::Result;

use crate::parser::lexer::{Lexer, error::LexerError};

#[derive(Debug)]
///The type of the error that was generated
pub enum SlynxErrorType {
    Lexer,
    Parser,
    Type,
    Compilation,
}

#[derive(Debug)]
///An error that will be shown if something fails
pub struct SlynxError {
    ty: SlynxErrorType,
    line: usize,
    column: usize,
    message: String,
}
impl std::error::Error for SlynxError {}

impl std::fmt::Display for SlynxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_error = match self.ty {
            SlynxErrorType::Lexer => "Lexing",
            SlynxErrorType::Parser => "Parsing",
            SlynxErrorType::Compilation => "Compilation",
            SlynxErrorType::Type => "Type ",
        };
        writeln!(
            f,
            "{type_error} Error: {} at line: {}:{}",
            self.message, self.line, self.column
        )
    }
}

///Context that will have all the information needed when erroring or retrieving metadata about the code itself during compilation.
///For example, this can be used when erroring to retrieve the correct line where the file errored
pub struct SlynxContext {
    ///The source code of the files. Maps the name of some to it's source code. Can and is used when importing contents(will be implemented yet)
    files: HashMap<Arc<PathBuf>, String>,
    ///Maps the name of some file to it's lines. Used when wanting to retrieve for example, returning the lines where an error occuried
    lines: HashMap<Arc<PathBuf>, Vec<usize>>,
    entry_point: Arc<PathBuf>,
}

impl SlynxContext {
    pub fn new(entry_point: Arc<PathBuf>) -> Result<Self> {
        let mut out = Self {
            files: HashMap::new(),
            lines: HashMap::new(),
            entry_point: entry_point.clone(),
        };
        out.insert_file(entry_point)?;
        Ok(out)
    }

    ///Gets the source code of the file that will start all the compilation
    pub fn get_entry_point_source(&self) -> &str {
        self.files
            .get(&self.entry_point)
            .expect("Entry point should map to a file")
    }

    ///Inserts the file with provided `path` if it exists.
    pub fn insert_file(&mut self, path: Arc<PathBuf>) -> Result<()> {
        let file = std::fs::read_to_string(&path.as_path())?;
        let lines = file
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| if c == '\n' { Some(idx) } else { None })
            .collect::<Vec<_>>();

        self.files.insert(path.clone(), file);
        self.lines.insert(path, lines);
        Ok(())
    }

    ///Based on the provided `index`, which is the index of a char on the source code of `path`, returns the line where it's located on the file of the provided `path`.
    ///This will return its line and the column
    pub fn get_line_info(&self, path: &Arc<PathBuf>, index: usize) -> (usize, usize) {
        let lines = self
            .lines
            .get(path)
            .expect("Path should be provided on the context");
        let out = match lines.binary_search(&index) {
            Ok(v) => (v, index - lines[v]),
            Err(e) => (e + 1, {
                let mut column = index.saturating_sub(lines[e.saturating_sub(1)]);
                if column == 0 {
                    column = index + 1;
                }
                column
            }),
        };

        out
    }

    pub fn start_compilation(self) -> Result<()> {
        let stream = match Lexer::tokenize(self.get_entry_point_source()) {
            Ok(value) => value,
            Err(e) => match e {
                LexerError::UnrecognizedChar { index, .. } => {
                    let (line, column) = self.get_line_info(&self.entry_point, index);
                    return Err(SlynxError {
                        line,
                        ty: SlynxErrorType::Lexer,
                        column,
                        message: e.to_string(),
                    }
                    .into());
                }
            },
        };

        Ok(())
    }
}
