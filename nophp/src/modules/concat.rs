use super::*;

#[derive(Module)]
pub struct ConcatMod {
    data: Map<String, Value>,
}

impl ConcatMod {
    pub fn try_new(data: Map<String, Value>) -> Result<Self> {
        Ok(Self { data })
    }
}

impl ModuleImpl for ConcatMod {
    fn proc_tree(&self, _buffer: &mut String, _scope: &mut ScopeBuffer) {
        // does nothing ig
    }

    fn eval(&self, buffer: &mut String, scope: &mut ScopeBuffer) -> Option<NpType> {
        let val = &self.data;

        match (val.get("0"), val.get("1")) {
            (Some(first), Some(second)) => {
                // TODO Make scopes slightly global
                let mut scope_vars = &mut scope.variables;
                let mut compiler = Compiler::new(buffer, &mut scope_vars);
                compiler.execute(&vec![first.clone()]);
                let eval = &compiler.eval();
                let first = eval.get(0).unwrap();

                let mut scope_vars = &mut scope.variables;
                let mut compiler = Compiler::new(buffer, &mut scope_vars);
                compiler.execute(&vec![second.clone()]);
                let eval = &compiler.eval();
                let second = eval.get(0).unwrap();

                Some(NpType::String(f!("{first}{second}").into()))
            }
            _ => panic!("Malformed AST"),
        }
    }
}
