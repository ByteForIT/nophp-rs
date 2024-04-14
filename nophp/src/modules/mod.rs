mod concat;
mod resolut;

use std::fmt::Display;
use std::fs;
use std::io::Write;

pub use concat::*;
pub use nophp_derive::Module;
pub use resolut::*;

use serde_json::{Map, Value};
pub use Value::Object as AstMap;
pub use Value::String as AstStr;

use crate::compiler::Compiler;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum NpType {
    String(String),
}

impl Display for NpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => {
                let value = value.replace("\\n", "\n");
                write!(f, "{value}")
            }
        }
    }
}

pub trait ModuleImpl {
    // TODO: Return Result
    fn proc_tree(&self, buffer: &mut String);

    fn eval(&self, _buffer: &mut String) -> Option<NpType> {
        None
    }
}

pub trait Module: ModuleImpl {
    fn name(&self) -> &'static str;
}

impl PartialEq for dyn Module {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

#[derive(Module)]
pub struct Php {
    data: Vec<Value>,
}

impl Php {
    pub fn try_new(data: Map<String, Value>) -> Result<Self> {
        // we first check if the data provided is proper
        let data = data.get("PROGRAM").expect("Malformed AST");
        let data = data.as_array().expect("Malformed AST").to_owned();

        Ok(Self { data })
    }
}

impl ModuleImpl for Php {
    fn proc_tree(&self, buffer: &mut String) {
        println!("[PHP] triggered SP (single page) build");

        let ast = &self.data;

        let mut compiler = Compiler::new(buffer);

        compiler.execute(ast);
        compiler.run();
    }
}

#[derive(Module)]
pub struct FunctionCall {
    arguments: Vec<Value>,
    function: String,
}

impl FunctionCall {
    pub fn try_new(data: Map<String, Value>) -> Result<Self> {
        // we first check if the data provided is proper
        let arguments = data
            .get("FUNCTION_ARGUMENTS")
            .expect("Malformed AST")
            .get("POSITIONAL_ARGS")
            .expect("Positional args are the only kind supported")
            .to_owned();

        let arguments = arguments.as_array().expect("Malformed AST").to_owned();

        let function = || {
            let function = data
                .get("ID")?
                .as_array()?
                .get(1)?
                .as_object()?
                .get("VALUE")?
                .as_str()?
                .to_owned();

            Some(function)
        };

        let function = function().expect("Malformed AST");

        Ok(Self {
            arguments,
            function,
        })
    }
}

impl ModuleImpl for FunctionCall {
    fn proc_tree(&self, buffer: &mut String) {
        let ast = &self.arguments;

        let mut compiler = Compiler::new(buffer);

        compiler.execute(ast);

        // just echo
        if self.function != "echo" {
            panic!("Only echo is implimented");
        }

        let binding = compiler.eval();
        let out = binding.get(0).unwrap();

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("testout.txt")
            .expect("Could not create an output buffer");

        write!(file, "{out}").unwrap();

        buffer.push_str(&out.to_string());
    }
}
