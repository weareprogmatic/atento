use crate::variable::Variable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Step {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub inputs: Vec<Variable>,
    pub script: String,
    #[serde(default)]
    pub outputs: Vec<Variable>,
}
