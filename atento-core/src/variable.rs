use crate::data_type::DataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct RawVariable {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Option<DataType>,
    pub value: Option<Value>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[serde(from = "RawVariable")]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: DataType,
    pub value: Option<Value>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
}

fn infer_type(value: &Value) -> Option<DataType> {
    match value {
        Value::Number(n) if n.is_i64() => Some(DataType::Int),
        Value::Number(n) if n.is_f64() => Some(DataType::Float),
        Value::Bool(_) => Some(DataType::Bool),

        // Check for String and attempt to infer DateTime before falling back to String
        Value::String(s) => {
            // Attempt to parse the string as an RFC3339 DateTime (a widely accepted format).
            // This is the core logic for inferring a date from a string.
            if s.parse::<DateTime<Utc>>().is_ok() {
                Some(DataType::DateTime)
            } else {
                Some(DataType::String)
            }
        }
        _ => None,
    }
}

impl RawVariable {
    pub fn into_strict(self) -> Result<Variable, String> {
        let type_ = match (self.type_, &self.value, &self.ref_) {
            (Some(t), _, _) => {
                if t == DataType::None {
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
                    type_: DataType::None, // temporary placeholder, will be replaced during reference resolution
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
