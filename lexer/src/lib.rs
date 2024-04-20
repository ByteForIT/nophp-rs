use std::collections::HashMap;

use log::info;
use pyo3::prelude::*;
use pythonize::depythonize_bound;
use serde_json::Value;

const LEXER: &str = concat!(
    include_str!(concat!(env!("OUT_DIR"), "/lexer.py")),
    "\nlexer = PyettyLexer()",
);

const PARSER: &str = concat!(
    include_str!(concat!(env!("OUT_DIR"), "/pparser.py")),
    "\nparser = PyettyParser()",
);

pub struct Lexer<'a> {
    files: &'a [String],
}

pub type LexerOutput = Vec<Value>;
pub type Project = HashMap<String, Value>;

impl<'a> Lexer<'a> {
    pub fn new(files: &'a Vec<String>) -> Self {
        Lexer { files }
    }

    pub fn parse(&self) -> PyResult<LexerOutput> {
        Python::with_gil(|py| -> PyResult<LexerOutput> {
            info!("Python GIL acquired");
            let (lexer, parser) = self.get_funcs(py)?;
            let files = self.files;
            let ast = files
                .iter()
                .map(|file| self.lex(&lexer, &parser, file))
                .filter_map(|file| file.ok())
                .collect();
            info!("Exiting python");
            Ok(ast)
        })
    }

    fn get_funcs(&self, py: Python<'a>) -> PyResult<(Bound<'_, PyAny>, Bound<'_, PyAny>)> {
        let lexer = PyModule::from_code_bound(py, LEXER, "nophp.lexer.py", "nophp.lexer")?;
        let parser = PyModule::from_code_bound(py, PARSER, "nophp.parser.py", "nophp.parser")?;

        let lexer = lexer.getattr("lexer")?;
        let parser = parser.getattr("parser")?;

        Ok((lexer, parser))
    }

    fn lex(&self, lexer: &Bound<PyAny>, parser: &Bound<PyAny>, file: &str) -> PyResult<Value> {
        let tokens = lexer.call_method("tokenize", (file,), None)?;
        let tokens = parser.call_method("parse", (tokens,), None)?;

        let ast: Value = depythonize_bound(tokens)?;

        Ok(ast)
    }
}
