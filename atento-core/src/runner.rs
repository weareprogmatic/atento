use crate::step::Step;
use serde::Serialize;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Runner {
    pub type_: String,
    pub func: fn(step: &mut Step) -> RunnerResult,
}

impl Runner {
    pub fn run(&self, step: &mut Step) -> RunnerResult {
        (self.func)(step)
    }
}

#[derive(Debug, Serialize)]
pub struct RunnerResult {
    pub exit_code: i32,
    pub duration_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<HashMap<String, Value>>,
}
