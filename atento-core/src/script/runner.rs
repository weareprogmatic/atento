use crate::workflow::runner::RunnerResult;
use std::fs::Permissions;
use std::io::Write;
use std::os::unix::fs::PermissionsExt; // for chmod on Unix
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub const PREFIX: &str = "OUT__";
const TEMP_FILENAME: &'static str = "atento_temp_file_";

pub fn run(script: &str, ext: &str, args: &str) -> Result<RunnerResult, String> {
    if script.is_empty() {
        return Err("Please provide the script to run".to_string());
    }

    if args.is_empty() {
        return Err("Please provide the args to run".to_string());
    }

    // debug purposes only
    println!(".{script}");

    // Create a uniquely-named temporary file - NamedTempFile has drop
    let mut temp_file: NamedTempFile = tempfile::Builder::new()
        .prefix(TEMP_FILENAME)
        .suffix(ext)
        .tempfile()
        .map_err(|e| e.to_string())?;

    let path: PathBuf = temp_file.path().to_path_buf();
    writeln!(temp_file, "{script}");

    // Set explicit permissions
    #[cfg(unix)]
    {
        // 0o700 = rwx------ → readable, writable, and executable by the owner
        let perm = Permissions::from_mode(0o700);
        temp_file
            .as_file()
            .set_permissions(perm)
            .map_err(|e| e.to_string())?;
    }

    #[cfg(windows)]
    {
        // On Windows, Permissions only allows read-only flag
        let perm = Permissions::from_readonly(false);
        temp_file.as_file().set_permissions(perm)?;
    }

    let arg_list: Vec<String> = args
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if arg_list.is_empty() {
        return Err("empty args".to_string());
    }

    let mut cmd = Command::new(&arg_list[0]);
    cmd.args(&arg_list[1..]);

    // PowerShell do not to send telemetry data to Microsoft
    if ext == ".ps1" {
        cmd.env("POWERSHELL_TELEMETRY_OPTOUT", "1");
    }

    let output = cmd
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Error running command: {}", e))
        .unwrap();

    // File will be deleted automatically when `temp_file` is dropped
    drop(temp_file); // explicitly drop to delete now

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    stderr = stderr
        .lines()
        .filter(|line| !line.contains("[Perftrack") && !line.contains("NamedPipeIPC"))
        .collect::<Vec<_>>()
        .join("\n");

    // debub purposes
    // println!("STDOUT: .{}", stdout);
    // println!("STDERR: .{}", stderr);

    Ok(RunnerResult {
        exit_code,
        stdout: Some(stdout.trim().to_string()).filter(|s| !s.is_empty()),
        stderr: Some(stderr.trim().to_string()).filter(|s| !s.is_empty()),
        outputs: None,
        duration_ms: 0,
    })
}
