use crate::data_type::DataType;
use crate::runner::{Runner, RunnerResult};
use crate::step::Step;
use crate::variable::Variable;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub name: Option<String>,
    pub version: Option<String>,
    pub timeout: Option<u64>,
    #[serde(default)]
    pub config: Vec<Variable>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub results: Vec<Variable>,

    #[serde(skip)]
    runners: Vec<Runner>,
}

#[derive(Debug, Serialize)]
pub struct WorkflowResult {
    pub name: Option<String>,
    pub version: Option<String>,
    pub duration: u64,

    #[serde(default)]
    pub config: HashMap<String, Value>,

    #[serde(default)]
    pub results: HashMap<String, Value>,

    #[serde(default)]
    pub out: Vec<RunnerResult>,
}

impl Workflow {
    pub fn validate(&mut self) -> Result<(), String> {
        // 1. Build global registry (config + step outputs)
        // This breaks the lifetime dependency on self.steps.
        let mut registry: HashMap<String, Variable> = HashMap::new();

        // Add config vars
        for v in &self.config {
            let key = format!("config.{}", v.name);

            if registry.contains_key(&key) {
                return Err(format!("Duplicate config variable '{}'", v.name));
            }

            registry.insert(key, v.clone());
        }

        // Add output vars
        for v in &self.results {
            let key = format!("results.{}", v.name);

            if registry.contains_key(&key) {
                return Err(format!("Duplicate result variable '{}'", v.name));
            }

            registry.insert(key, v.clone());
        }

        // Add step outputs
        for step in &self.steps {
            let step_id = step.id.as_deref().unwrap_or(&step.name);
            for v in &step.outputs {
                let key = format!("steps.{}.outputs.{}", step_id, v.name);
                if registry.contains_key(&key) {
                    return Err(format!(
                        "Duplicate output '{}' in step '{}'",
                        v.name, step_id
                    ));
                }

                registry.insert(key, v.clone());
            }
        }

        // 2. Validate step inputs
        // This second loop can now safely mutably borrow self.steps because the registry no longer holds references into it.
        for step in &mut self.steps {
            let step_id = step.id.as_deref().unwrap_or(&step.name);
            for input in &mut step.inputs {
                if let Some(reference) = &input.ref_ {
                    // registry.get() now returns Option<&Variable> from the HashMap storage
                    if let Some(source) = registry.get(reference) {
                        // type compatibility
                        if input.type_ == DataType::None {
                            // Type is being updated, requiring &mut self.steps
                            input.type_ = source.type_.clone();
                            input.value = source.value.clone();
                        } else if input.type_ != source.type_ {
                            return Err(format!(
                                "Type mismatch for '{}': {:?} expected, found {:?}",
                                reference, input.type_, source.type_
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Unresolved reference '{}' in step '{}'",
                            reference, step_id
                        ));
                    }
                } else if input.value.is_none() {
                    return Err(format!(
                        "Input '{}' in step '{}' has no value or ref",
                        input.name, step_id
                    ));
                }
            }

            for output in &mut step.outputs {
                if let Some(reference) = &output.ref_ {
                    // registry.get() now returns Option<&Variable> from the HashMap storage
                    if let Some(target) = registry.get(reference) {
                        // type compatibility
                        if output.type_ == DataType::None {
                            // Type is being updated, requiring &mut self.steps
                            output.type_ = target.type_.clone();
                        }
                    } else {
                        return Err(format!(
                            "Unresolved reference '{}' in step '{}'",
                            reference, step_id
                        ));
                    }
                }
            }
        }
        /*
                // 3. Validate results
                for result in &self.results {
                    if let Some(reference) = &result.ref_ {
                        if let Some(source) = registry.get(reference) {
                            if result.type_ != source.type_ {
                                return Err(format!(
                                    "Result type mismatch for '{}': {:?} vs {:?}",
                                    result.name, result.type_, source.type_
                                ));
                            }
                        } else {
                            return Err(format!("Invalid result reference '{}'", reference));
                        }
                    } else {
                        return Err(format!("Result '{}' missing ref", result.name));
                    }
                }
        */
        Ok(())
    }

    pub fn run(&mut self) -> Result<WorkflowResult, String> {
        let mut result = WorkflowResult {
            name: self.name.clone(),
            version: self.version.clone(),
            duration: 0,
            config: HashMap::new(),
            results: HashMap::new(),
            out: Vec::new(),
        };

        // 2. Run each step sequentially
        for step in self.steps.iter_mut() {
            println!("➡️ Running step: {}", step.name);

            let runner = self
                .runners
                .iter()
                .find(|v| v.type_ == step.type_)
                .ok_or_else(|| format!("Runner '{}' not registered", step.type_))?;

            let run_result: RunnerResult = runner.run(step);
            let json = serde_json::to_string_pretty(&run_result);
            if !json.is_err() {
                println!("{}", json.unwrap());
            }

            result.out.push(run_result);

            let outputs = serde_json::to_string_pretty(&step.outputs);
            if !outputs.is_err() {
                println!("{}", outputs.unwrap());
            }

            for output in step.outputs.iter() {
                if let Some(reference) = &output.ref_ {
                    for result in self.results.iter_mut() {
                        if format!("results.{}", result.name) == *reference {
                            result.value = output.value.clone();
                            break;
                        }
                    }
                }
            }
            //step.run(&mut ctx, )?;
        }

        // 3. Collect final results
        let mut results: HashMap<String, serde_yaml::Value> = HashMap::new();
        for res in &self.results {
            /*
            if let Some(var) = ctx.vars.get(&res.ref_) {
                results.insert(
                    res.ref_.clone(),
                    var.value.clone().unwrap_or(serde_yaml::Value::Null),
                );
            } else {
                return Err(format!("Result reference '{}' not found", res.ref_));
            }*/
            println!("Key: {} Value: {:#?}", res.name, res.value);
        }

        Ok(result)
    }

    pub fn register(&mut self, runner: Runner) {
        self.runners.push(runner);
    }
}
