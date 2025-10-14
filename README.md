# atento
Run, chain, and automate CLI tasks with Atento’s lightweight engine.

# Atento Core

[![Crates.io](https://img.shields.io/crates/v/atento-core.svg)](https://crates.io/crates/atento-core)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg)](LICENSE)

**Atento** is a Rust-native engine for linear task automation.  
It lets you define scripts with sequential steps, pass global and local variables between steps, and run CLI tasks reliably. Designed for sysadmins, IT admins, and MSPs.

## Features

- Linear step execution for CLI scripts
- Global and local variable passing
- YAML or TOML configuration for task definitions
- Easy integration with Rust projects or CLI tools
- Open-core engine ready for extension

## Getting Started

Add `atento-core` to your Cargo.toml:

```toml
[dependencies]
atento-core = "0.1.0"
