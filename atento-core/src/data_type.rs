use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    None,
    String,
    Int,
    Float,
    Bool,
    DateTime,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::None
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", serde_json::to_string(&self))
    }
}
