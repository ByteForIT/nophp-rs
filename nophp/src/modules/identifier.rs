use super::*;

#[derive(Module)]
pub struct Identifier {
    id: String,
}

impl Identifier {
    pub fn try_new(data: Map<String, Value>) -> Result<Self> {
        let id = data.get("VALUE").ok_or(NoPhpError::MalformedAST)?;
        let id = id.as_str().ok_or(NoPhpError::MalformedAST)?.to_string();
        Ok(Self { id })
    }
}

impl ModuleImpl for Identifier {
    fn proc_tree(&self, _buffer: &mut String, _scope: &mut ScopeBuffer) {
    }

    fn eval(&self, _buffer: &mut String, scope: &mut ScopeBuffer) -> Option<NpType> {
        let value = scope.variables.get(&self.id);
        let value = value.unwrap_or(&NpType::Null);
        Some(value.clone())
    }

}
