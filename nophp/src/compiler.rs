use std::collections::HashMap;

use serde_json::Value;

use crate::modules::*;
use crate::prelude::*;

struct ModuleList(Vec<Box<dyn Module>>);

pub struct ScopeBuffer<'b> {
    pub variables: &'b mut HashMap<String, NpType>,
}

impl<'b> ScopeBuffer<'b> {
    pub fn new(variables: &'b mut HashMap<String, NpType>) -> Self {
        Self { variables }
    }
}

/// This is the interpreter
/// And also the scope
pub struct Compiler<'a, 'b> {
    modules: ModuleList,
    buffer: &'a mut String,
    scope: ScopeBuffer<'b>,
}

// Lifetimes (gasp) (?!?!)
// 'a is for the first call to the Compiler
// 'b is for the call to the constructor of
// the current Compiler/scope
impl<'a, 'b> Compiler<'a, 'b> {
    pub fn new(buffer: &'a mut String, variables: &'b mut HashMap<String, NpType>) -> Self {
        let scope = ScopeBuffer::new(variables);
        Self {
            modules: ModuleList(vec![]),
            buffer,
            scope,
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
            module.proc_tree(&mut self.buffer, &mut self.scope);
        });
    }

    pub fn eval(&mut self) -> Vec<NpType> {
        let values: Vec<_> = self
            .modules
            .0
            .iter()
            .filter_map(|module| module.eval(&mut self.buffer, &mut self.scope))
            .collect();

        values
    }

    pub fn get_buffer(&self) -> String {
        self.buffer.clone()
    }

    fn parse_module(&self, value: &Vec<Value>) -> Result<Box<dyn Module>> {
        match (value.get(0), value.get(1)) {
            (Some(AstStr(id)), Some(AstMap(value))) => match id.as_str() {
                // TODO: Fix clone maddness
                "PHP" => Ok(Box::new(Php::try_new(value.clone())?)),
                "FUNCTION_CALL" => Ok(Box::new(FunctionCall::try_new(value.clone())?)),
                "CONCAT" => Ok(Box::new(ConcatMod::try_new(value.clone())?)),
                "VARIABLE_ASSIGNMENT" => Ok(Box::new(VariableAssignment::try_new(value.clone())?)),
                "CONDITIONAL" => Ok(Box::new(Conditional::try_new(value)?)),
                // Identifiers (for variables for example)
                "ID" => Ok(Box::new(Identifier::try_new(value.clone())?)),
                // NoPHP Types
                "STRING" => Ok(Box::new(ResolutMod::try_new(id, value.clone())?)),
                id => unimplemented!("The module for {id} is not implimented"),
            },
            _ => Err(NoPhpError::MalformedAST),
        }
    }

    fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules.0.push(module);
    }
}
