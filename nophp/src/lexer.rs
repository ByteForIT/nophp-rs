use std::collections::HashMap;

use crate::prelude::*;
use pyo3::prelude::*;
use pythonize::depythonize_bound;
use serde_json::Value;

#[allow(unused)]
pub type Project = HashMap<String, Value>;

const LEXER: &str = c!(
    include_str!(c!(env!("OUT_DIR"), "/lexer.py")),
    "\nlexer = PyettyLexer()",
);

const PARSER: &str = c!(
    include_str!(c!(env!("OUT_DIR"), "/pparser.py")),
    "\nparser = PyettyParser()",
);

pub fn lex_one(file: &str) -> Result<Value> {
    let ast = Python::with_gil(|py| -> PyResult<Value> {
        let (lexer, parser) = get_funcs(py)?;
        let ast = lex(&lexer, &parser, file)?;
        Ok(ast)
    })?;
    Ok(ast)
}

#[allow(unused)]
pub fn lex_many(files: &Vec<String>) -> Result<Vec<Value>> {
    let ast = Python::with_gil(|py| -> PyResult<Vec<Value>> {
        let (lexer, parser) = get_funcs(py)?;
        let ast = files
            .iter()
            .map(|file| lex(&lexer, &parser, file))
            .filter_map(|file| file.ok())
            .collect();
        Ok(ast)
    })?;

    Ok(ast)
}

fn get_funcs<'a>(py: Python<'a>) -> PyResult<(Bound<'a, PyAny>, Bound<'a, PyAny>)> {
    let lexer = PyModule::from_code_bound(py, LEXER, "nophp.lexer.py", "nophp.lexer")?;
    let parser = PyModule::from_code_bound(py, PARSER, "nophp.parser.py", "nophp.parser")?;

    let lexer = lexer.getattr("lexer")?;
    let parser = parser.getattr("parser")?;

    Ok((lexer, parser))
}

fn lex(lexer: &Bound<PyAny>, parser: &Bound<PyAny>, file: &str) -> PyResult<Value> {
    let tokens = lexer.call_method("tokenize", (file,), None)?;
    let tokens = parser.call_method("parse", (tokens,), None)?;

    let ast: Value = depythonize_bound(tokens)?;

    Ok(ast)
}
