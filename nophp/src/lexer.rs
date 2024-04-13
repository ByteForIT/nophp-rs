use crate::prelude::*;
use pyo3::prelude::*;
use pythonize::depythonize_bound;
use serde_json::Value;

const LEXER: &str = c!(
    include_str!(c!(env!("OUT_DIR"), "/lexer.py")),
    "\nlexer = PyettyLexer()",
);

const PARSER: &str = c!(
    include_str!(c!(env!("OUT_DIR"), "/pparser.py")),
    "\nparser = PyettyParser()",
);

pub fn lex(file: &str) -> Result<Value> {
    println!("🐍 initiating python...");
    let ast = Python::with_gil(|py| -> PyResult<Value> {
        println!("🐍 python initiated ({})", py.version());

        println!("🐍 creating ast");
        let lexer = PyModule::from_code_bound(py, LEXER, "nophp.lexer.py", "nophp.lexer")?;
        let parser = PyModule::from_code_bound(py, PARSER, "nophp.parser.py", "nophp.parser")?;

        let lexer = lexer.getattr("lexer")?;
        let parser = parser.getattr("parser")?;

        let tokens = lexer.call_method("tokenize", (file,), None)?;
        let tokens = parser.call_method("parse", (tokens,), None)?;
        println!("🐍 ast creation completed");

        let ast: Value = depythonize_bound(tokens)?;

        Ok(ast)
    })?;
    println!("🦀 ast recieved");

    Ok(ast)
}
