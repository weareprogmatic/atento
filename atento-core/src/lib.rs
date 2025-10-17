use script::bash;
use std::{path::PathBuf, process};
use workflow::{runner::Runner, workflow::Workflow};

mod script;
mod workflow;

pub fn run(filename: &str) {
    let path = PathBuf::from(filename);

    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let mut wf: Workflow = match serde_yaml::from_str(&contents) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to parse file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // register runners
    wf.register(Runner {
        type_: "script::bash".to_string(),
        func: bash::run,
    });

    match wf.validate() {
        Ok(()) => match wf.run() {
            Ok(result) => {
                let json = serde_json::to_string_pretty(&result);
                if !json.is_err() {
                    println!("{}", json.unwrap());
                }
            }
            Err(e) => {
                eprintln!("Failed to validate file '{}': {}", filename, e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Failed to validate file '{}': {}", filename, e);
            process::exit(1);
        }
    }
}

// TODO - to remove

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
