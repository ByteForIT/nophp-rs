mod concat;
mod conditional;
mod identifier;
mod resolut;
mod variable;

pub use concat::*;
pub use conditional::*;
pub use identifier::*;
pub use resolut::*;
pub use variable::*;

use std::fmt::Display;

pub use nophp_derive::Module;


use serde_json::{Map, Value};
pub use Value::Object as AstMap;
pub use Value::String as AstStr;
pub use Value::Array as AstArr;

use crate::compiler::Compiler;
use crate::compiler::ScopeBuffer;
pub use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NpType {
    String(String),
    Number(i64),
    //Null, // source of evil
}

impl NpType {
    pub fn try_new(var_type: &String, var_value: &Value) -> Result<Self> {
        match var_type.as_str() {
            "STRING" => {
                let value = var_value.as_str().ok_or(NoPhpError::ValueParseError)?;
                Ok(Self::String(value.into()))
            }
            "INT" => {
                let value = var_value.as_str().ok_or(NoPhpError::ValueParseError)?;
                // TODO: Fix the serialisation in the lexer module
                let value = value.parse::<i64>().map_err(|_| NoPhpError::ValueParseError)?;
                Ok(Self::Number(value))
            }
            _ => todo!("Type {var_type} is not yet implimented"),
        }
    }
}

impl Display for NpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => {
                let value = value.replace("\\n", "\n");
                write!(f, "{value}")
            }
            Self::Number(value) => write!(f, "{value}"),
            //Self::Null => write!(f, "nuh uh"),
        }
    }
}

pub trait ModuleImpl {
    // TODO: Return Result
    fn proc_tree(&self, buffer: &mut String, scope: &mut ScopeBuffer);

    fn eval(&self, _buffer: &mut String, _scope: &mut ScopeBuffer) -> Option<NpType> {
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
    fn proc_tree(&self, buffer: &mut String, scope: &mut ScopeBuffer) {
        println!("[PHP] triggered SP (single page) build");

        let ast = &self.data;

        let mut scope_vars = &mut scope.variables;
        let mut compiler = Compiler::new(buffer, &mut scope_vars);

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
    fn proc_tree(&self, buffer: &mut String, scope: &mut ScopeBuffer) {
        let ast = &self.arguments;

        let mut scope_vars = &mut scope.variables;
        let mut compiler = Compiler::new(buffer, &mut scope_vars);

        compiler.execute(ast);

        // just echo
        if self.function != "echo" {
            panic!("Only echo is implimented");
        }

        let binding = compiler.eval();
        let out = binding.get(0).unwrap();

        // let mut file = fs::OpenOptions::new()
        //     .append(true)
        //     .create(true)
        //     .open("testout.txt")
        //     .expect("Could not create an output buffer");
        //
        // write!(file, "{out}").unwrap();

        buffer.push_str(&out.to_string());
    }
}
