use crate::data_type::DataType;
use crate::runner::RunnerResult;
use crate::script;
use crate::step::Step;
use crate::variable::Variable;
use chrono::{DateTime, Utc};
use serde_json::Number as JsonNumber;
use serde_yaml::Value;

const ARGS: &str = "bash -c";
const EXTENSION: &str = ".sh";

pub fn run(step: &mut Step) -> RunnerResult {
    let footer: Vec<String> = step.outputs.iter().map(|i| to_output(&i)).collect();

    let mut script = format!("{}\n{}\n", step.script, footer.join("\n"))
        .trim()
        .to_string();

    step.inputs.iter().for_each(|f| {
        script = script.replace(&format!("{{{{ {} }}}}", f.name), to_input(f).as_str());
    });

    // Debug purposes
    println!("{script}");

    match script::run(&script, EXTENSION, ARGS) {
        Ok(o) => {
            let stdout = output(step, o.stdout);

            RunnerResult {
                exit_code: o.exit_code,
                duration_ms: o.duration_ms,
                stdout: stdout,
                stderr: o.stderr,
                outputs: None,
            }
        }
        Err(e) => {
            eprintln!("Error in step {}: {}", step.name, e);

            RunnerResult {
                exit_code: 0,
                duration_ms: 0,
                stdout: None,
                stderr: None,
                outputs: None,
            }
        }
    }
}

fn to_input(var: &Variable) -> String {
    var.value
        .as_ref()
        .and_then(|value| match var.type_ {
            DataType::String => value.as_str().map(|s| format!("{}", escape(s))),

            DataType::Bool => value
                .as_bool()
                .map(|b| format!("{}", if b { "true" } else { "false" })),

            DataType::Int => value.as_i64().map(|i| format!("{i}")),

            DataType::Float => value.as_f64().map(|f| format!("{f}")),

            DataType::DateTime => value.as_str().map(|s| {
                if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                    format!("{}", dt.to_rfc3339())
                } else {
                    eprintln!("Warning: Invalid datetime for key '{}': {s}", var.name);
                    format!("{s}")
                }
            }),

            _ => {
                eprintln!("Warning: Unsupported DataType for key '{}'", var.name);
                None // Returns None for unsupported types
            }
        })
        // 3. If the final result is Some(String), unwrap it; otherwise, return "".to_string().
        .unwrap_or_else(|| "".to_string())
}

fn to_output(var: &Variable) -> String {
    format!("echo \"{}{}=${}\"", script::PREFIX, var.name, var.name)
}

fn escape(s: &str) -> String {
    let escaped_content = s.replace('\'', "'\\''");
    format!("{escaped_content}")
}

fn output(step: &mut Step, stdout: Option<String>) -> Option<String> {
    if stdout.is_none() {
        return None;
    }

    let out = stdout.unwrap();

    let output_lines: Vec<&str> = out
        .lines()
        .filter(|line| line.starts_with(script::PREFIX))
        .collect();

    output_lines
        .iter()
        .filter_map(|line| line[script::PREFIX.len()..].split_once('='))
        .for_each(|(key, value)| {
            if let Some(output) = step.outputs.iter_mut().find(|o| o.name == key) {
                let trimmed = value.trim_matches('"');
                let value = match output.type_ {
                    DataType::String => Value::String(trimmed.to_string()),

                    DataType::Bool => Value::Bool(trimmed.parse().unwrap_or(false)),

                    DataType::Int => Value::Number(serde_yaml::Number::from(
                        trimmed.parse::<i64>().unwrap_or(0),
                    )),

                    DataType::Float => {
                        // Safely parse f64, convert to a serde_json::Number, and then map to a serde_yaml::Value::Number
                        trimmed
                            .parse::<f64>()
                            .ok() // Convert Result<f64, _> to Option<f64>
                            // Try to convert to serde_json::Number (since serde_yaml doesn't have a direct from_f64)
                            .and_then(JsonNumber::from_f64)
                            // Convert the serde_json::Number to serde_yaml::Number
                            .and_then(|num| num.as_f64().map(serde_yaml::Number::from))
                            // If all fails, use Null
                            .map_or(Value::Null, Value::Number)
                    }

                    DataType::DateTime => trimmed
                        .parse::<DateTime<chrono::Utc>>()
                        .map(|dt| Value::String(dt.to_rfc3339()))
                        .unwrap_or(Value::Null),

                    _ => Value::Null,
                };

                output.value = Some(value);
            }
        });

    // remove output lines from stdout
    Some(
        out.lines()
            .filter(|line| !line.starts_with(script::PREFIX))
            .collect::<Vec<&str>>()
            .join("\n"),
    )
}
