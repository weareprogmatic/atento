use crate::workflow::vartype::VarType;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct RawVariable {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Option<VarType>,
    pub value: Option<Value>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[serde(from = "RawVariable")]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: VarType,
    pub value: Option<Value>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
}

fn infer_type(value: &serde_yaml::Value) -> Option<VarType> {
    match value {
        serde_yaml::Value::String(_) => Some(VarType::String),
        serde_yaml::Value::Number(n) if n.is_i64() => Some(VarType::Int),
        serde_yaml::Value::Number(_) => Some(VarType::Float),
        serde_yaml::Value::Bool(_) => Some(VarType::Bool),
        serde_yaml::Value::Sequence(_) => Some(VarType::List),
        serde_yaml::Value::Mapping(_) => Some(VarType::Map),
        _ => None,
    }
}

impl RawVariable {
    pub fn into_strict(self) -> Result<Variable, String> {
        let type_ = match (self.type_, &self.value, &self.ref_) {
            (Some(t), _, _) => {
                if t == VarType::None {
                    return Err(format!(
                        "Variable '{}' type none is invalid - internal only",
                        self.name
                    ));
                }

                t
            } // type explicitly provided
            (None, Some(val), _) => {
                infer_type(val).ok_or_else(|| format!("Cannot infer type for '{}'", self.name))?
            }
            (None, None, Some(_)) => {
                return Ok(Variable {
                    name: self.name,
                    type_: VarType::None, // temporary placeholder, will be replaced during reference resolution
                    value: None,
                    ref_: self.ref_,
                });

                // variable has a reference, type can be inferred later from the referenced variable
                // for now, you can leave it as a placeholder or error depending on your workflow design
                return Err(format!(
                    "Variable '{}' has a reference but no type — type must be resolved from reference later",
                    self.name
                ));
            }
            (None, None, None) => {
                return Err(format!(
                    "Variable '{}' missing type, value, and ref",
                    self.name
                ));
            }
        };

        Ok(Variable {
            name: self.name,
            type_,
            value: self.value,
            ref_: self.ref_,
        })
    }
}

impl From<RawVariable> for Variable {
    fn from(v: RawVariable) -> Self {
        v.into_strict()
            .unwrap_or_else(|e| panic!("Invalid variable: {}", e))
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", serde_json::to_string(&self))
    }
}
