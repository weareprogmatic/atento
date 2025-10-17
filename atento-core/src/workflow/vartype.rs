use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VarType {
    None,
    String,
    Int,
    Float,
    Bool,
    DateTime,
    List,
    Map,
}

impl Default for VarType {
    fn default() -> Self {
        VarType::None
    }
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", serde_json::to_string(&self))
    }
}
