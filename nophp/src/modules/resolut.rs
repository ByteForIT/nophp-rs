use super::*;

#[derive(Module)]
pub struct ResolutMod {
    np_type: NpType,
}

impl ResolutMod {
    pub fn try_new(id: &String, value: Map<String, Value>) -> Result<Self> {
        let value = value.get("VALUE").expect("Malformed AST").to_owned();

        match (id.as_str(), value) {
            ("STRING", AstStr(value)) => Ok(Self {
                np_type: NpType::String(value),
            }),
            _ => Err(NoPhpError::Other("Invalid type reached")),
        }
    }
}

impl ModuleImpl for ResolutMod {
    fn proc_tree(&self, _buffer: &mut String, _scope: &mut ScopeBuffer) {
        println!("resolved type of {}", self.np_type);
    }

    fn eval(&self, _buffer: &mut String, _scope: &mut ScopeBuffer) -> Option<NpType> {
        return Some(self.np_type.clone());
    }
}
