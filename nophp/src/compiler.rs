use serde_json::Value;

use crate::modules::*;
use crate::prelude::*;

struct ModuleList(Vec<Box<dyn Module>>);

/// This is the interpreter
pub struct Compiler<'a> {
    modules: ModuleList,
    buffer: &'a mut String,
}

impl<'a> Compiler<'a> {
    pub fn new(buffer: &'a mut String) -> Self {
        Self {
            modules: ModuleList(vec![]),
            buffer,
        }
    }

    pub fn execute(&mut self, ast: &Vec<Value>) {
        for action in ast {
            if let Value::Array(code) = action {
                let module = self.parse_module(&code).unwrap();
                self.add_module(module);
            }
        }
    }

    pub fn run(&mut self) {
        self.modules.0.iter().for_each(|module| {
            module.proc_tree(&mut self.buffer);
        });
    }

    pub fn eval(&mut self) -> Vec<NpType> {
        let values: Vec<_> = self
            .modules
            .0
            .iter()
            .filter_map(|module| module.eval(&mut self.buffer))
            .collect();

        values
    }

    pub fn get_buffer(&self) -> String {
        self.buffer.clone()
    }

    fn parse_module(&self, value: &Vec<Value>) -> Result<Box<dyn Module>> {
        match (value.get(0), value.get(1)) {
            (Some(AstStr(id)), Some(AstMap(value))) => match id.as_str() {
                "PHP" => Ok(Box::new(Php::try_new(value.clone())?)),
                "FUNCTION_CALL" => Ok(Box::new(FunctionCall::try_new(value.clone())?)),
                "CONCAT" => Ok(Box::new(ConcatMod::try_new(value.clone())?)),
                "VARIABLE_ASSIGNMENT" => todo!("The module for VARIABLE_ASSIGNMENT is not implimented"),
                "STRING" => Ok(Box::new(ResolutMod::try_new(id, value.clone())?)),
                id => unimplemented!("The module for {id} is not implimented"),
            },
            _ => Err("Malformed AST".into()),
        }
    }

    fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules.0.push(module);
    }
}
