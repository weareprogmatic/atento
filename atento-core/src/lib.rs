#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(clippy::missing_errors_doc)] // Until docs are added

use crate::runner::Runner;
use crate::workflow::Workflow;
use std::{path::PathBuf, process};

mod bash;
mod data_type;
mod runner;
mod script;
mod step;
mod variable;
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

    // register available runners
    wf.register(Runner {
        type_: "script::bash".to_string(),
        func: bash::run,
    });

    match wf.validate() {
        Ok(()) => match wf.run() {
            Ok(result) => {
                let json = serde_json::to_string_pretty(&result);
                match json {
                    Ok(j) => println!("{}", j),
                    Err(e) => {
                        eprintln!("Failed to serialize results '{}': {}", filename, e);
                        process::exit(1);
                    }
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
