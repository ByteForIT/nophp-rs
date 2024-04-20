use std::collections::HashMap;

use super::*;

enum EqualityOp {
    EqEq, // Works
    NotEqEq, // Works
    Gt, // Works
    Lt, // Broken, same for Ge, Le and Eee is not implimented
}

impl EqualityOp {
    pub fn try_new(value: &str) -> Result<Self> {
        match value {
            "EQEQ" => Ok(Self::EqEq),
            "NOT_EQEQ" => Ok(Self::NotEqEq),
            "GREATER" => Ok(Self::Gt),
            "LESSER" => Ok(Self::Lt),
            _ => Err(NoPhpError::InvalidEqOp(value.to_string())),
        }
    }

    pub fn compare<T: PartialEq + PartialOrd>(&self, left: T, right: T) -> bool {
        match self {
            Self::EqEq => left == right,
            Self::NotEqEq => left != right,
            Self::Gt => left > right,
            Self::Lt => left < right,
        }
    }
}

#[derive(Module)]
pub struct Conditional {
    op: EqualityOp,
    a: Vec<Value>,
    b: Vec<Value>,
    code: Vec<Value>,
}

impl Conditional {
    pub fn try_new(value: &Map<String, Value>) -> Result<Self> {
        let (code, condition) = match value.get("IF") {
            Some(Value::Array(code)) => {
                let code = Self::get_code_and_cond(code).ok_or(NoPhpError::MalformedAST)?;
                Ok(code)
            }
            _ => Err(NoPhpError::MalformedAST),
        }?;

        match (condition.get(0), condition.get(1), condition.get(2)) {
            (Some(AstStr(operation)), Some(AstArr(a)), Some(AstArr(b))) => {
                let op = EqualityOp::try_new(operation)?;

                Ok(Self {
                    op,
                    a: a.clone(),
                    b: b.clone(),
                    code: code.to_vec(),
                })
            }
            _ => Err(NoPhpError::MalformedAST),
        }
    }

    fn qualify_side(&self, value: &[Value], scope: &HashMap<String, NpType>) -> Option<NpType> {
        match (value.get(0), value.get(1)) {
            (Some(AstStr(id)), Some(AstMap(value))) => {
                let value = value.get("VALUE")?;
                if id == "ID" {
                    let value = value.as_str()?;
                    scope.get(value).cloned()
                } else {
                    NpType::try_new(id, value).ok()
                }
            },
            a => {
                println!("{a:?}");
                None
            }
        }
    }

    fn get_code_and_cond(code: &[Value]) -> Option<(&Vec<Value>, &Vec<Value>)> {
        let value = code.get(1)?.as_object()?;

        let code = value.get("CODE")?.as_array()?;
        let condition = value.get("CONDITION")?.as_array()?;

        Some((code, condition))
    }
}

impl ModuleImpl for Conditional {
    fn proc_tree(&self, buffer: &mut String, scope: &mut ScopeBuffer) {
        let lhs = self.qualify_side(&self.a, scope.variables);
        let rhs = self.qualify_side(&self.b, scope.variables);

        if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
            let eq = self.op.compare(lhs, rhs);

            if eq {
                let scope_vars = &mut scope.variables;
                let mut compiler = Compiler::new(buffer, scope_vars);

                compiler.execute(&self.code);
                compiler.run();
            }

        };
    }
}
