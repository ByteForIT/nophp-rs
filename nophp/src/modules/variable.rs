use super::*;

#[derive(Module)]
pub struct VariableAssignment {
    id: String,
    value: NpType,
}

impl VariableAssignment {
    pub fn try_new(value: Map<String, Value>) -> Result<Self> {
        let expr = value.get("EXPRESSION").ok_or(NoPhpError::MalformedAST)?;

        let id = value.get("ID").ok_or(NoPhpError::MalformedAST)?;
        let id = id.as_str().ok_or(NoPhpError::MalformedAST)?.to_string();

        let expr = expr.as_array().ok_or(NoPhpError::MalformedAST)?;

        match (expr.get(0), expr.get(1)) {
            (Some(AstStr(var_type)), Some(AstMap(var_value))) => {
                let var_value = var_value.get("VALUE").ok_or(NoPhpError::MalformedAST)?;
                let value = NpType::try_new(var_type, var_value)?;
                Ok(Self { id, value })
            }
            _ => {
                Err(NoPhpError::MalformedAST)
            }
        }
    }
}

impl ModuleImpl for VariableAssignment {
    fn proc_tree(&self, _buffer: &mut String, scope: &mut ScopeBuffer) {
        scope.variables.insert(self.id.clone(), self.value.clone());
    }
}
