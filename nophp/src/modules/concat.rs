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
    fn proc_tree(&self) {
        // does nothing ig
    }

    fn eval(&self) -> Option<NpType> {
        let val = &self.data;

        match (val.get("0"), val.get("1")) {
            (Some(first), Some(second)) => {
                let mut compiler = Compiler::new();
                compiler.execute(&vec![first.clone()]);
                let eval = &compiler.eval();
                let first = eval.get(0).unwrap();

                let mut compiler = Compiler::new();
                compiler.execute(&vec![second.clone()]);
                let eval = &compiler.eval();
                let second = eval.get(0).unwrap();

                Some(NpType::String(f!("{first}{second}").into()))
            }
            _ => panic!("Malformed AST"),
        }
    }
}
